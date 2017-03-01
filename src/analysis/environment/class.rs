use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use analysis::environment::constructor::analyze_constructor_declaration;
use analysis::environment::constructor::ConstructorEnvironment;
use analysis::environment::field::analyze_field_declaration;
use analysis::environment::field::FieldEnvironment;
use analysis::environment::interface::InterfaceEnvironment;
use analysis::environment::method::analyze_abstract_method_declaration;
use analysis::environment::method::analyze_method_declaration;
use analysis::environment::method::MethodEnvironment;

#[derive(Clone,Debug)]
pub struct ClassEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub name: ASTNode,
    pub extends: Vec<ASTNode>,
    pub implements: Vec<ASTNode>,
    pub constructors: Vec<ConstructorEnvironment>,
    pub fields: Vec<FieldEnvironment>,
    pub methods: Vec<MethodEnvironment>,
}

pub fn analyze_class_declaration(classes: &mut Vec<ClassEnvironment>,
                                 interfaces: &Vec<InterfaceEnvironment>,
                                 node: &ASTNode)
                                 -> Result<(), String> {
    let mut modifiers = Vec::new();
    for child in node.children[0].clone().children {
        modifiers.push(child);
    }

    let name = node.children[2].clone();

    let object = ASTNode {
        token: Token::new(TokenKind::Identifier, Some("Object")),
        children: Vec::new(),
    };
    let object_name = ASTNode {
        token: Token::new(TokenKind::NonTerminal, Some("Name")),
        children: vec![object],
    };
    let mut extends = vec![object_name];

    let mut implements = Vec::new();
    let mut constructors = Vec::new();
    let mut fields = Vec::new();
    let mut methods = Vec::new();
    for (idx, child) in node.children.iter().enumerate() {
        if idx < 3 {
            continue;
        }

        match child.token.lexeme {
            Some(ref le) if le == "Implements" => {
                let grandkid = child.children[1].clone().flatten().clone();
                for mut greatgrandkid in grandkid.children {
                    if greatgrandkid.token.kind != TokenKind::Comma {
                        implements.push(greatgrandkid.flatten().clone());
                    }
                }

                for class in classes.clone() {
                    for implemented in &implements {
                        if &class.name == implemented {
                            return Err("classes cannot implement classes".to_owned());
                        }
                    }
                }
                // TODO: no dups, non-circular
            }
            Some(ref le) if le == "ClassExtends" => {
                // remove implicit Object inheritance
                extends = Vec::new();
                extends.push(child.children[1].clone());

                let fnode = ASTNode {
                    token: Token::new(TokenKind::Final, None),
                    children: Vec::new(),
                };
                for class in classes.clone() {
                    for extended in &extends {
                        if &class.name == extended {
                            if class.modifiers.contains(&fnode) {
                                return Err("classes cannot extend final classes".to_owned());
                            }
                            break;
                        }
                    }
                }

                for interface in interfaces.clone() {
                    for extended in &extends {
                        if &interface.name == extended {
                            return Err("classes cannot extend interfaces".to_owned());
                        }
                    }
                }
                // TODO: no dups, non-circular
            }
            Some(ref le) if le == "ClassBody" && child.children.len() == 3 => {
                let anode = ASTNode {
                    token: Token::new(TokenKind::Abstract, None),
                    children: Vec::new(),
                };

                let mut decls = child.children[1].clone();
                while decls.clone().token.lexeme.unwrap_or("".to_owned()) ==
                      "ClassBodyDeclarations" {
                    let result = match decls.children[1].clone().token.lexeme {
                        Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                            match analyze_abstract_method_declaration(classes,
                                                                      &extends,
                                                                      &interfaces,
                                                                      &implements,
                                                                      &mut methods,
                                                                      &decls.children[1].children
                                                                           [0]) {
                                Ok(_) => (),
                                Err(e) => return Err(e),
                            }

                            match methods.last() {
                                Some(m) if m.modifiers.contains(&anode) &&
                                           !modifiers.contains(&anode) => {
                                    Err("a class with an abstract method must be abstract"
                                        .to_owned())
                                }
                                _ => Ok(()),
                            }
                        }
                        Some(ref lex) if lex == "ConstructorDeclaration" => {
                            analyze_constructor_declaration(&mut constructors, &decls.children[1])
                        }
                        Some(ref lex) if lex == "FieldDeclaration" => {
                            analyze_field_declaration(&mut fields, &decls.children[1])
                        }
                        Some(ref lex) if lex == "MethodDeclaration" => {
                            analyze_method_declaration(classes,
                                                       &extends,
                                                       &interfaces,
                                                       &implements,
                                                       &mut methods,
                                                       &decls.children[1].children[0],
                                                       &decls.children[1].children[1])
                        }
                        _ => Ok(()),
                    };
                    if result.is_err() {
                        return result;
                    }
                    decls = decls.children[0].clone();
                }
                let result = match decls.token.lexeme {
                    Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                        match analyze_abstract_method_declaration(classes,
                                                                  &extends,
                                                                  &interfaces,
                                                                  &implements,
                                                                  &mut methods,
                                                                  &decls.children[0]) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }

                        match methods.last() {
                            Some(m) if m.modifiers.contains(&anode) &&
                                       !modifiers.contains(&anode) => {
                                Err("a class with an abstract method must be abstract".to_owned())
                            }
                            _ => Ok(()),
                        }
                    }
                    Some(ref lex) if lex == "ConstructorDeclaration" => {
                        analyze_constructor_declaration(&mut constructors, &decls)
                    }
                    Some(ref lex) if lex == "FieldDeclaration" => {
                        analyze_field_declaration(&mut fields, &decls)
                    }
                    Some(ref lex) if lex == "MethodDeclaration" => {
                        analyze_method_declaration(classes,
                                                   &extends,
                                                   &interfaces,
                                                   &implements,
                                                   &mut methods,
                                                   &decls.children[0],
                                                   &decls.children[1])
                    }
                    _ => Ok(()),
                };
                if result.is_err() {
                    return result;
                }
            }
            _ => (),
        }
    }

    // TODO: qualified path matters here
    for class in classes.clone() {
        if class.name == name {
            return Err("class/interface names must be unique".to_owned());
        }
    }

    // TODO: qualified path matters here
    for interface in interfaces {
        if interface.name == name {
            return Err("class/interface names must be unique".to_owned());
        }
    }

    classes.push(ClassEnvironment {
        modifiers: modifiers,
        name: name,
        extends: extends,
        implements: implements,
        constructors: constructors,
        fields: fields,
        methods: methods,
    });

    Ok(())
}

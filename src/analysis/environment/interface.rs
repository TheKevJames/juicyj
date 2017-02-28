use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use analysis::environment::class::ClassEnvironment;
use analysis::environment::field::analyze_constant_declaration;
use analysis::environment::field::FieldEnvironment;
use analysis::environment::method::analyze_abstract_method_declaration;
use analysis::environment::method::MethodEnvironment;

#[derive(Clone,Debug)]
pub struct InterfaceEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub name: ASTNode,
    pub extends: Vec<ASTNode>,
    pub fields: Vec<FieldEnvironment>,
    pub methods: Vec<MethodEnvironment>,
}

pub fn analyze_interface_declaration(classes: &Vec<ClassEnvironment>,
                                     interfaces: &mut Vec<InterfaceEnvironment>,
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

    let mut fields = Vec::new();
    let mut methods = Vec::new();
    match node.children[3].token.lexeme {
        Some(ref l) if l == "InterfaceExtends" => {
            // remove implicit Object inheritance
            extends = Vec::new();

            let mut grandkid = node.children[3].children[1].clone();
            while grandkid.clone().token.lexeme.unwrap_or("".to_owned()) != "Name" {
                extends.push(grandkid.children[2].clone());
                grandkid = grandkid.children[0].clone();
            }
            extends.push(grandkid.clone());
            // TODO: no dups, non-circular
        }
        Some(ref l) if l == "InterfaceBody" && node.children[3].children.len() == 3 => {
            let mut decls = node.children[3].clone().children[1].clone();
            while decls.clone().token.lexeme.unwrap_or("".to_owned()) ==
                  "InterfaceMemberDeclarations" {
                let result = match decls.children[1].clone().token.lexeme {
                    Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                        analyze_abstract_method_declaration(classes,
                                                            &extends,
                                                            &interfaces,
                                                            &Vec::new(),
                                                            &mut methods,
                                                            &decls.children[1].children[0])
                    }
                    Some(ref lex) if lex == "ConstantDeclaration" => {
                        analyze_constant_declaration(&mut fields, &decls.children[1])
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
                    analyze_abstract_method_declaration(classes,
                                                        &extends,
                                                        &interfaces,
                                                        &Vec::new(),
                                                        &mut methods,
                                                        &decls.children[0])
                }
                Some(ref lex) if lex == "ConstantDeclaration" => {
                    analyze_constant_declaration(&mut fields, &decls)
                }
                _ => Ok(()),
            };
            if result.is_err() {
                return result;
            }
        }
        _ => (),
    }

    // TODO: qualified path matters here
    for class in classes {
        if class.name == name {
            return Err("class/interface names must be unique".to_owned());
        }
    }

    // TODO: qualified path matters here
    for interface in interfaces.clone() {
        if interface.name == name {
            return Err("class/interface names must be unique".to_owned());
        }
    }

    interfaces.push(InterfaceEnvironment {
        modifiers: modifiers,
        name: name,
        extends: extends,
        fields: fields,
        methods: methods,
    });

    Ok(())
}

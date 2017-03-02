use analysis::environment::classorinterface::ClassOrInterface;
use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use analysis::environment::constructor::analyze_constructor_declaration;
use analysis::environment::field::analyze_field_declaration;
use analysis::environment::method::analyze_abstract_method_declaration;
use analysis::environment::method::analyze_method_declaration;
use scanner::ASTNode;
use scanner::ASTNodeImport;
use scanner::Token;
use scanner::TokenKind;

pub fn analyze_class_declaration(canonical: &Vec<Token>,
                                 kinds: &mut Vec<ClassOrInterfaceEnvironment>,
                                 imports: &Vec<ASTNodeImport>,
                                 node: &ASTNode)
                                 -> Result<(), String> {
    let mut current = ClassOrInterfaceEnvironment {
        constructors: Vec::new(),
        extends: vec![vec![Token::new(TokenKind::Identifier, Some("Object"))]],
        fields: Vec::new(),
        implements: Vec::new(),
        kind: ClassOrInterface::CLASS,
        methods: Vec::new(),
        modifiers: Vec::new(),
        name: canonical.clone(),
    };

    for class_or_interface in kinds.clone() {
        if class_or_interface.name == current.name {
            return Err("class/interface names must be unique".to_owned());
        }
    }

    for child in node.children[0].clone().children {
        current.modifiers.push(child);
    }

    for (idx, child) in node.children.iter().enumerate() {
        if idx < 3 {
            continue;
        }

        match child.token.lexeme {
            Some(ref le) if le == "Implements" => {
                let mut grandkid = child.children[1].clone();
                let grandkid = match grandkid.clone().token.lexeme {
                    Some(ref l) if l == "InterfaceTypeList" => grandkid.flatten().clone(),
                    _ => grandkid,
                };
                for mut greatgrandkid in grandkid.children {
                    if greatgrandkid.token.kind == TokenKind::Identifier {
                        let interface = vec![greatgrandkid.clone().token];
                        if current.implements.contains(&interface) {
                            return Err("interfaces must not be repeated in implements clauses"
                                .to_owned());
                        }
                        current.implements.push(interface);
                    } else if greatgrandkid.clone().token.lexeme.unwrap_or("".to_owned()) ==
                              "Name" {
                        let mut children = Vec::new();
                        for child in greatgrandkid.flatten().clone().children {
                            children.push(child.token);
                        }
                        if current.implements.contains(&children) {
                            return Err("interfaces must not be repeated in implements clauses"
                                .to_owned());
                        }
                        current.implements.push(children);
                    } else if greatgrandkid.token.kind == TokenKind::Comma {
                        continue;
                    } else {
                        return Err(format!("got invalid InterfaceTypeList child {}",
                                           greatgrandkid.token));
                    }
                }

                for implemented in &current.implements {
                    for class_or_interface in kinds.clone() {
                        // TODO: name lookup
                        if class_or_interface.kind == ClassOrInterface::CLASS &&
                           &class_or_interface.name == implemented {
                            return Err("classes cannot implement classes".to_owned());
                        }
                    }
                }
                // TODO: no dups
            }
            Some(ref le) if le == "ClassExtends" => {
                // remove implicit Object inheritance
                current.extends = Vec::new();
                if child.children[1].token.kind == TokenKind::Identifier {
                    current.extends.push(vec![child.children[1].clone().token]);
                } else if child.children[1].clone().token.lexeme.unwrap_or("".to_owned()) ==
                          "Name" {
                    let mut children = Vec::new();
                    for child in child.children[1].clone().flatten().clone().children {
                        children.push(child.token);
                    }
                    current.extends.push(children);
                } else {
                    return Err(format!("got invalid ClassExtends child {}",
                                       child.children[1].token));
                }

                let fnode = ASTNode {
                    token: Token::new(TokenKind::Final, None),
                    children: Vec::new(),
                };
                for extended in &current.extends {
                    for class_or_interface in kinds.clone() {
                        // TODO: name lookup
                        if class_or_interface.kind == ClassOrInterface::CLASS &&
                           &class_or_interface.name == extended {
                            if class_or_interface.modifiers.contains(&fnode) {
                                return Err("classes cannot extend final classes".to_owned());
                            }
                            break;
                        }
                    }
                }

                for extended in &current.extends {
                    for class_or_interface in kinds.clone() {
                        // TODO: name lookup
                        if class_or_interface.kind == ClassOrInterface::INTERFACE &&
                           &class_or_interface.name == extended {
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
                let decls = match decls.clone().token.lexeme {
                    Some(ref l) if l == "ClassBodyDeclarations" => decls.flatten().clone(),
                    _ => decls,
                };
                for decl in &decls.children {
                    let result = match decl.token.lexeme {
                        Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                            match analyze_abstract_method_declaration(kinds,
                                                                      &mut current,
                                                                      &decl.children[0]) {
                                Ok(_) => (),
                                Err(e) => return Err(e),
                            }

                            // TODO: ensure non-abstract class does not contain
                            // un-overriden abstract methods
                            match current.methods.last() {
                                Some(m) if m.modifiers.contains(&anode) &&
                                           !current.modifiers.contains(&anode) => {
                                    Err("a class with an abstract method must be abstract"
                                        .to_owned())
                                }
                                _ => Ok(()),
                            }
                        }
                        Some(ref lex) if lex == "ConstructorDeclaration" => {
                            analyze_constructor_declaration(kinds,
                                                            imports,
                                                            &mut current,
                                                            &decl.children[0],
                                                            &decl.children[1],
                                                            &decl.children[2])
                        }
                        Some(ref lex) if lex == "FieldDeclaration" => {
                            analyze_field_declaration(&mut current.fields, &decl)
                        }
                        Some(ref lex) if lex == "MethodDeclaration" => {
                            analyze_method_declaration(kinds,
                                                       imports,
                                                       &mut current,
                                                       &decl.children[0],
                                                       &decl.children[1])
                        }
                        _ => Ok(()),
                    };
                    if result.is_err() {
                        return result;
                    }
                }
            }
            _ => (),
        }
    }

    kinds.push(current);
    Ok(())
}

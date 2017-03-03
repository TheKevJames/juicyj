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

pub fn analyze_class_declaration(canonical: &ASTNode,
                                 kinds: &mut Vec<ClassOrInterfaceEnvironment>,
                                 imports: &Vec<ASTNodeImport>,
                                 node: &ASTNode)
                                 -> Result<(), String> {
    let mut current = ClassOrInterfaceEnvironment::new(canonical.clone(), ClassOrInterface::CLASS);

    for kind in kinds.clone() {
        if kind.name == current.name {
            return Err("class/interface names must be unique".to_owned());
        }
    }

    current.imports = imports.clone();
    if let Some(class_name) = current.name.children.last() {
        for import in &current.imports {
            if let Some(import_name) = import.import.children.last() {
                if import_name == class_name {
                    return Err(format!("single-type-import declaration clashes with class {}",
                                       class_name));
                }
            }
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
                    _ => {
                        ASTNode {
                            token: Token::new(TokenKind::NonTerminal, Some("InterfaceTypeList")),
                            children: vec![grandkid],
                        }
                    }
                };
                for mut greatgrandkid in grandkid.children {
                    if greatgrandkid.token.kind == TokenKind::Identifier {
                        let interface = greatgrandkid.clone();
                        if current.implements.contains(&interface) {
                            return Err("interfaces must not be repeated in implements clauses"
                                .to_owned());
                        }
                        current.implements.push(interface);
                    } else if greatgrandkid.clone().token.lexeme.unwrap_or("".to_owned()) ==
                              "Name" {
                        let interface = greatgrandkid.flatten().clone();
                        if current.implements.contains(&interface) {
                            return Err("interfaces must not be repeated in implements clauses"
                                .to_owned());
                        }
                        current.implements.push(interface);
                    } else if greatgrandkid.token.kind == TokenKind::Comma {
                        continue;
                    } else {
                        return Err(format!("got invalid InterfaceTypeList child {}",
                                           greatgrandkid.token));
                    }
                }
            }
            Some(ref le) if le == "ClassExtends" => {
                if child.children[1].token.kind == TokenKind::Identifier {
                    current.extends.push(child.children[1].clone());
                } else if child.children[1].clone().token.lexeme.unwrap_or("".to_owned()) ==
                          "Name" {
                    current.extends.push(child.children[1].clone().flatten().clone());
                } else {
                    return Err(format!("got invalid ClassExtends child {}",
                                       child.children[1].token));
                }
            }
            Some(ref le) if le == "ClassBody" && child.children.len() == 3 => {
                let mut decls = child.children[1].clone();
                let decls = match decls.clone().token.lexeme {
                    Some(ref l) if l == "ClassBodyDeclarations" => decls.flatten().clone(),
                    _ => {
                        ASTNode {
                            token: Token::new(TokenKind::NonTerminal,
                                              Some("ClassBodyDeclarations")),
                            children: vec![decls],
                        }
                    }
                };
                for decl in &decls.children {
                    let result = match decl.token.lexeme {
                        Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                            analyze_abstract_method_declaration(&mut current, &decl.children[0])
                        }
                        Some(ref lex) if lex == "ConstructorDeclaration" => {
                            analyze_constructor_declaration(&mut current,
                                                            &decl.children[0],
                                                            &decl.children[1],
                                                            &decl.children[2])
                        }
                        Some(ref lex) if lex == "FieldDeclaration" => {
                            analyze_field_declaration(&mut current.fields, &decl)
                        }
                        Some(ref lex) if lex == "MethodDeclaration" => {
                            analyze_method_declaration(&mut current,
                                                       &decl.children[0],
                                                       &decl.children[1])
                        }
                        Some(ref lex) => {
                            return Err(format!("no ClassBody analyzer for {}", lex));
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

    let object_name = ASTNode {
        token: Token::new(TokenKind::NonTerminal, Some("Name")),
        children: vec![ASTNode {
                           token: Token::new(TokenKind::Identifier, Some("java")),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Dot, None),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Identifier, Some("lang")),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Dot, None),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Identifier, Some("Object")),
                           children: Vec::new(),
                       }],
    };
    if current.extends.is_empty() && current.name != object_name {
        current.extends.push(ASTNode {
            token: Token::new(TokenKind::Identifier, Some("Object")),
            children: Vec::new(),
        });
    }

    kinds.push(current);
    Ok(())
}

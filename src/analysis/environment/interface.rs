use analysis::environment::classorinterface::ClassOrInterface;
use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use analysis::environment::field::analyze_constant_declaration;
use analysis::environment::method::analyze_abstract_method_declaration;
use scanner::ASTNode;
use scanner::ASTNodeImport;
use scanner::Token;
use scanner::TokenKind;

pub fn analyze_interface_declaration(canonical: &ASTNode,
                                     kinds: &mut Vec<ClassOrInterfaceEnvironment>,
                                     imports: &Vec<ASTNodeImport>,
                                     node: &ASTNode)
                                     -> Result<(), String> {
    let mut current = ClassOrInterfaceEnvironment::new(canonical.clone(),
                                                       ClassOrInterface::INTERFACE);

    for kind in kinds.clone() {
        if kind.name == current.name {
            return Err("class/interface names must be unique".to_owned());
        }
    }

    current.imports = imports.clone();
    if let Some((interface_name, interface_package)) = current.name.children.split_last() {
        for import in &current.imports {
            if let Some((import_name, import_package)) = import.import.children.split_last() {
                if import_name == interface_name && import_package != interface_package {
                    return Err(format!("single-type-import declaration clashes with interface {}",
                                       interface_name));
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
            Some(ref l) if l == "InterfaceExtends" => {
                let mut grandkid = child.children[1].clone();
                let grandkid = match grandkid.clone().token.lexeme {
                    Some(ref l) if l == "ClassOrInterfaceTypeList" => grandkid.flatten().clone(),
                    _ => {
                        ASTNode {
                            token: Token::new(TokenKind::NonTerminal, Some("ClassOrInterfaceTypeList")),
                            children: vec![grandkid],
                        }
                    }
                };
                for mut greatgrandkid in grandkid.children {
                    if greatgrandkid.token.kind == TokenKind::Identifier {
                        current.extends.push(greatgrandkid.clone());
                    } else if greatgrandkid.clone().token.lexeme.unwrap_or("".to_owned()) ==
                              "Name" {
                        current.extends.push(greatgrandkid.flatten().clone());
                    } else if greatgrandkid.token.kind == TokenKind::Comma {
                        continue;
                    } else {
                        return Err(format!("got invalid ClassOrInterfaceTypeList child {}",
                                           greatgrandkid.token));
                    }
                }
            }
            Some(ref l) if l == "InterfaceBody" && child.children.len() == 3 => {
                let mut decls = child.clone().children[1].clone();
                let decls = match decls.clone().token.lexeme {
                    Some(ref l) if l == "InterfaceMemberDeclarations" => decls.flatten().clone(),
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
                        Some(ref lex) if lex == "ConstantDeclaration" => {
                            analyze_constant_declaration(&mut current.fields, &decl)
                        }
                        Some(ref lex) => {
                            return Err(format!("no InterfaceBody analyzer for {}", lex));
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

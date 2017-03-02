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

    for child in node.children[0].clone().children {
        current.modifiers.push(child);
    }

    match node.children[3].token.lexeme {
        Some(ref l) if l == "InterfaceExtends" => {
            let mut grandkid = node.children[3].children[1].clone();
            let grandkid = match grandkid.clone().token.lexeme {
                Some(ref l) if l == "InterfaceExtendsList" => grandkid.flatten().clone(),
                _ => grandkid,
            };
            for mut greatgrandkid in grandkid.children {
                if greatgrandkid.token.kind == TokenKind::Identifier {
                    let cls = greatgrandkid.clone();
                    if current.extends.contains(&cls) {
                        return Err("objects must not be repeated in extends clauses".to_owned());
                    }
                    current.extends.push(cls);
                } else if greatgrandkid.clone().token.lexeme.unwrap_or("".to_owned()) == "Name" {
                    let mut cls = greatgrandkid.flatten().clone();
                    if current.extends.contains(&cls) {
                        return Err("objects must not be repeated in extends clauses".to_owned());
                    }
                    current.extends.push(cls);
                } else if greatgrandkid.token.kind == TokenKind::Comma {
                    continue;
                } else {
                    return Err(format!("got invalid InterfaceExtendsList child {}",
                                       greatgrandkid.token));
                }
            }
        }
        Some(ref l) if l == "InterfaceBody" && node.children[3].children.len() == 3 => {
            let mut decls = node.children[3].clone().children[1].clone();
            let decls = match decls.clone().token.lexeme {
                Some(ref l) if l == "InterfaceMemberDeclarations" => decls.flatten().clone(),
                _ => decls,
            };
            for decl in &decls.children {
                let result = match decl.token.lexeme {
                    Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                        analyze_abstract_method_declaration(kinds, &mut current, &decl.children[0])
                    }
                    Some(ref lex) if lex == "ConstantDeclaration" => {
                        analyze_constant_declaration(&mut current.fields, &decl)
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

    kinds.push(current);
    Ok(())
}

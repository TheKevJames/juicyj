use analysis::environment::classorinterface::ClassOrInterface;
use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use analysis::environment::field::analyze_constant_declaration;
use analysis::environment::method::analyze_abstract_method_declaration;
use scanner::ASTNode;
use scanner::ASTNodeImport;
use scanner::Token;
use scanner::TokenKind;

pub fn analyze_interface_declaration(canonical: &Vec<Token>,
                                     kinds: &mut Vec<ClassOrInterfaceEnvironment>,
                                     imports: &Vec<ASTNodeImport>,
                                     node: &ASTNode)
                                     -> Result<(), String> {
    let mut current = ClassOrInterfaceEnvironment {
        constructors: Vec::new(),
        extends: vec![vec![Token::new(TokenKind::Identifier, Some("Object"))]],
        fields: Vec::new(),
        implements: Vec::new(),
        kind: ClassOrInterface::INTERFACE,
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

    match node.children[3].token.lexeme {
        Some(ref l) if l == "InterfaceExtends" => {
            // remove implicit Object inheritance
            current.extends = Vec::new();

            let mut grandkid = node.children[3].children[1].clone();
            let grandkid = match grandkid.clone().token.lexeme {
                Some(ref l) if l == "InterfaceExtendsList" => grandkid.flatten().clone(),
                _ => grandkid,
            };
            for mut greatgrandkid in grandkid.children {
                if greatgrandkid.token.kind == TokenKind::Identifier {
                    current.extends.push(vec![greatgrandkid.clone().token]);
                } else if greatgrandkid.clone().token.lexeme.unwrap_or("".to_owned()) == "Name" {
                    let mut children = Vec::new();
                    for child in greatgrandkid.flatten().clone().children {
                        children.push(child.token);
                    }
                    current.extends.push(children);
                } else if greatgrandkid.token.kind == TokenKind::Comma {
                    continue;
                } else {
                    return Err(format!("got invalid InterfaceExtendsList child {}",
                                       greatgrandkid.token));
                }
            }

            for extended in &current.extends {
                for class_or_interface in kinds.clone() {
                    // TODO: name lookup
                    if class_or_interface.kind == ClassOrInterface::CLASS &&
                       &class_or_interface.name == extended {
                        return Err("interfaces cannot extend classes".to_owned());
                    }
                }
            }
            // TODO: no dups, non-circular
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

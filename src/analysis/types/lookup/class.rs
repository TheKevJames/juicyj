use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::types::lookup::array;
use analysis::types::verify;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref STAR: ASTNode = ASTNode {
        token: Token::new(TokenKind::Star, None),
        children: Vec::new(),
    };
}
const PRIMITIVES: [TokenKind; 7] = [TokenKind::Boolean,
                                    TokenKind::Byte,
                                    TokenKind::Char,
                                    TokenKind::Int,
                                    TokenKind::Null,
                                    TokenKind::Short,
                                    TokenKind::Void];

pub fn in_env(name: &ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>)
              -> Result<ClassOrInterfaceEnvironment, String> {
    let mut name = name.clone();
    name.flatten();

    if let Some(l) = name.clone().token.lexeme {
        if l == "ArrayType" {
            return Ok(array::create(&name));
        }

        if l == "Name" && name.children.len() == 1 {
            if PRIMITIVES.contains(&name.children[0].clone().token.kind) {
                return Ok(ClassOrInterfaceEnvironment::new(name.clone(), ClassOrInterface::CLASS));
            }
        }
    }

    if PRIMITIVES.contains(&name.token.kind) {
        return Ok(ClassOrInterfaceEnvironment::new(name.clone(), ClassOrInterface::CLASS));
    }

    lookup(&name, current, kinds)
}

pub fn lookup_step0_canonical(name: &ASTNode,
                              current: &ClassOrInterfaceEnvironment,
                              kinds: &Vec<ClassOrInterfaceEnvironment>)
                              -> Option<Result<ClassOrInterfaceEnvironment, String>> {
    for kind in kinds {
        if name != &kind.name {
            continue;
        }

        let result = verify::prefixes::canonical(name, current, kinds);
        if result.is_err() {
            return Some(Err(result.unwrap_err()));
        }

        return Some(Ok(kind.clone()));
    }

    None
}

fn lookup_step1_enclosing_class(name: &ASTNode,
                                current: &ClassOrInterfaceEnvironment)
                                -> Option<Result<ClassOrInterfaceEnvironment, String>> {
    if let Some(class_name) = current.name.children.last() {
        if name != class_name {
            return None;
        }

        return Some(Ok(current.clone()));
    }

    None
}

fn lookup_step2_import_single(name: &ASTNode,
                              current: &ClassOrInterfaceEnvironment,
                              kinds: &Vec<ClassOrInterfaceEnvironment>)
                              -> Option<Result<ClassOrInterfaceEnvironment, String>> {
    for import in &current.imports {
        if let Some(import_name) = import.import.children.last() {
            if *import_name == *STAR || import_name != name {
                continue;
            }

            let mut found = None;
            for kind in kinds {
                if kind.name != import.import {
                    continue;
                }

                if found.is_some() {
                    return Some(Err(format!("ambiguous type lookup for import {:?}",
                                            import.import)));
                }

                found = Some(Ok(kind.clone()));
            }

            if found.is_none() {
                return Some(Err(format!("could not find type for imported lookup {:?}",
                                        import.import)));
            }

            return found;
        }
    }

    None
}

pub fn lookup_step3_enclosing_package(name: &ASTNode,
                                      current: &ClassOrInterfaceEnvironment,
                                      kinds: &Vec<ClassOrInterfaceEnvironment>)
                                      -> Option<Result<ClassOrInterfaceEnvironment, String>> {
    let mut found = None;
    for kind in kinds {
        if let Some((kind_name, kind_package)) = kind.name.children.split_last() {
            if let Some((_, package)) = current.name.children.split_last() {
                if package != kind_package || name != kind_name {
                    continue;
                }

                if found.is_some() {
                    return Some(Err(format!("ambiguous type lookup in package {:?}", package)));
                }

                found = Some(Ok(kind.clone()));
            }
        }
    }

    found
}

fn lookup_step4_import_ondemand(name: &ASTNode,
                                current: &ClassOrInterfaceEnvironment,
                                kinds: &Vec<ClassOrInterfaceEnvironment>)
                                -> Option<Result<ClassOrInterfaceEnvironment, String>> {
    let mut found = None;
    for import in &current.imports {
        if let Some((import_name, import_package)) = import.import.children.split_last() {
            if *import_name != *STAR {
                continue;
            }

            for kind in kinds {
                if let Some((kind_name, kind_package)) = kind.name.children.split_last() {
                    if import_package != kind_package || name != kind_name {
                        continue;
                    }

                    if found.is_some() {
                        return Some(Err(format!("ambiguous on-demand lookup for {:?} in {:?}",
                                                name,
                                                kind_package)));
                    }

                    found = Some(Ok(kind.clone()));
                }
            }
        }
    }

    found
}

fn lookup(name: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>)
          -> Result<ClassOrInterfaceEnvironment, String> {
    let result = lookup_step0_canonical(name, current, kinds);
    if result.is_some() {
        return result.unwrap();
    }

    let result = lookup_step1_enclosing_class(name, current);
    if result.is_some() {
        return result.unwrap();
    }

    let result = lookup_step2_import_single(name, current, kinds);
    if result.is_some() {
        return result.unwrap();
    }

    let result = lookup_step3_enclosing_package(name, current, kinds);
    if result.is_some() {
        return result.unwrap();
    }

    let result = lookup_step4_import_ondemand(name, current, kinds);
    if result.is_some() {
        return result.unwrap();
    }

    Err(format!("could not lookup kind {:?} from class {:?}",
                name,
                current.name))
}

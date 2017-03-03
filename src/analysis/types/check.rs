use analysis::environment::ClassOrInterfaceEnvironment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

pub fn lookup(name: &ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>)
              -> Result<ClassOrInterfaceEnvironment, String> {
    let name = name.clone();
    let node_star = ASTNode {
        token: Token::new(TokenKind::Star, None),
        children: Vec::new(),
    };

    // 0. lookup canonical path
    for kind in kinds {
        if name == kind.name {
            return match verify_prefixes(name, current, kinds) {
                Ok(_) => Ok(kind.clone()),
                Err(e) => return Err(e),
            };
        }
    }

    // 1. try the enclosing class or interface
    if let Some(class_name) = current.name.children.last() {
        if name == class_name.clone() {
            return Ok(current.clone());
        }
    }

    let mut found = None;

    // 2. try any single-type-import (A.B.C.D)
    for import in &current.imports {
        if let Some(import_name) = import.import.children.last() {
            if import_name == &node_star {
                continue;
            }

            // find the right import
            if &name == import_name {
                for kind in kinds {
                    // find the associated kind
                    if kind.name == import.import {
                        match found {
                            Some(_) => {
                                return Err(format!("ambiguous type lookup for import {:?}",
                                                   import.import))
                            }
                            None => found = Some(kind.clone()),
                        }
                    }
                }

                if found.is_none() {
                    return Err(format!("could not find type for imported lookup {:?}",
                                       import.import));
                }
            }
        }
    }
    if let Some(f) = found {
        return Ok(f);
    }

    // 3. try the same package
    for kind in kinds {
        if let Some((kind_name, kind_package)) = kind.name.children.split_last() {
            if let Some((_, package)) = current.name.children.split_last() {
                if package == kind_package && &name == kind_name {
                    match found {
                        Some(_) => {
                            return Err(format!("ambiguous type lookup in package {:?}", package))
                        }
                        None => found = Some(kind.clone()),
                    }
                }
            }
        }
    }
    if let Some(f) = found {
        return Ok(f);
    }

    // 4. try any import-on-demand package (A.B.C.*) including java.lang.*
    for import in &current.imports {
        if let Some((import_name, import_package)) = import.import.children.split_last() {
            if import_name != &node_star {
                continue;
            }

            for kind in kinds {
                if let Some((kind_name, kind_package)) = kind.name.children.split_last() {
                    if import_package == kind_package && &name == kind_name {
                        match found {
                            Some(_) => {
                                return Err(format!("ambiguous on-demand lookup for {:?} in {:?}",
                                                   name,
                                                   kind_package))
                            }
                            None => found = Some(kind.clone()),
                        }
                    }
                }
            }
        }
    }

    match found {
        Some(f) => Ok(f),
        None => Err(format!("could not lookup kind {:?}", name)),
    }
}

pub fn verify(kind: ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>)
              -> Result<(), String> {
    let mut kind = kind;
    if let Some(l) = kind.clone().token.lexeme {
        if l == "ArrayType" {
            kind = kind.children[0].clone();
        }
    }
    if vec![TokenKind::Boolean,
            TokenKind::Byte,
            TokenKind::Char,
            TokenKind::Int,
            TokenKind::Short,
            TokenKind::Void]
        .contains(&kind.token.kind) {
        return Ok(());
    }

    match lookup(&kind.flatten(), current, kinds) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn verify_prefixes(kind: ASTNode,
                       current: &ClassOrInterfaceEnvironment,
                       kinds: &Vec<ClassOrInterfaceEnvironment>)
                       -> Result<(), String> {
    let mut prefix = Vec::new();
    for (idx, child) in kind.children.iter().enumerate() {
        prefix.push(child.clone());

        let testable = ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("Name")),
            children: prefix.clone(),
        };
        if idx % 2 == 0 && testable != kind {
            match verify(testable.clone(), current, kinds) {
                Ok(_) => return Err(format!("strict prefix {} resolves to type", testable)),
                Err(_) => (),
            }
        }
    }

    Ok(())
}

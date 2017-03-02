use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::Environment;
use scanner::ASTNode;
use scanner::ASTNodeImport;
use scanner::Token;
use scanner::TokenKind;

// if let Some(l) = declared_kind_astnode.clone().token.lexeme {
//     if l == "ArrayType" {
//         kind = kind.children[0].clone();
//     }
// }
// if !vec![TokenKind::Boolean,
//          TokenKind::Byte,
//          TokenKind::Char,
//          TokenKind::Int,
//          TokenKind::Short,
//          TokenKind::Void]
//     .contains(&kind.token.kind) {
//     match lookup(&kind.flatten(), current, kinds, imports) {
//         Ok(c) => (),  // TODO: do something with this kind
//         Err(e) => return Err(e),
//     }
// }
fn lookup(name: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          imports: &Vec<ASTNodeImport>)
          -> Result<ClassOrInterfaceEnvironment, String> {
    let name = name.clone();
    let node_star = ASTNode {
        token: Token::new(TokenKind::Star, None),
        children: Vec::new(),
    };

    // 0. lookup canonical path
    for kind in kinds {
        if name == kind.name {
            return Ok(kind.clone());
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
    for import in imports {
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
    for import in imports {
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

pub fn verify_inheritance(env: &Environment,
                          current: &ClassOrInterfaceEnvironment,
                          visited: &mut Vec<ASTNode>)
                          -> Result<(), String> {
    if visited.contains(&current.name) {
        return Err("cyclic class hierarchy detected".to_owned());
    }
    visited.push(current.name.clone());

    for extended in &current.extends {
        let found = match lookup(&extended, &current, &env.kinds, &current.imports) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let result = verify_inheritance(env, &found, &mut visited.clone());
        if result.is_err() {
            return result;
        }
    }

    Ok(())
}

pub fn verify(env: &Environment) -> Result<(), String> {
    let modifier_abstract = ASTNode {
        token: Token::new(TokenKind::Abstract, None),
        children: Vec::new(),
    };
    let modifier_final = ASTNode {
        token: Token::new(TokenKind::Final, None),
        children: Vec::new(),
    };
    let modifier_native = ASTNode {
        token: Token::new(TokenKind::Native, None),
        children: Vec::new(),
    };
    let modifier_static = ASTNode {
        token: Token::new(TokenKind::Static, None),
        children: Vec::new(),
    };

    for current in &env.kinds {
        if current.kind == ClassOrInterface::CLASS {
            for extended in &current.extends {
                // TODO: non-circular

                let found = match lookup(&extended, &current, &env.kinds, &current.imports) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if found.kind == ClassOrInterface::CLASS &&
                   found.modifiers.contains(&modifier_final) {
                    return Err(format!("class {} cannot extend final class {}", current, found));
                } else if found.kind == ClassOrInterface::INTERFACE {
                    return Err(format!("class {} cannot extend interface {}", current, found));
                }
            }

            for implemented in &current.implements {
                let found = match lookup(&implemented, &current, &env.kinds, &current.imports) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if found.kind == ClassOrInterface::CLASS {
                    return Err(format!("class {} cannot implement class {}", current, found));
                }
            }
        } else if current.kind == ClassOrInterface::INTERFACE {
            for extended in &current.extends {
                // TODO: non-circular

                let found = match lookup(&extended, &current, &env.kinds, &current.imports) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if found.kind == ClassOrInterface::CLASS {
                    return Err(format!("interface {} cannot extend class {}", current, found));
                }
            }
        }

        let result = verify_inheritance(env, &current, &mut Vec::new());
        if result.is_err() {
            return result;
        }

        for constructor in &current.constructors {
            // TODO: lookup each constructor.parameters

            // TODO: analyze constructor.body:
            // if body.children.len() == 3 {
            //     // TODO: eventually, this should need fields, etc, but since they can be
            //     // shadowed... meh.
            //     let globals = Vec::new();

            //     match analyze_block(kinds, imports, current, &globals, &mut child) {
            //     let mut child = body.children[1].clone();
            //         Ok(_) => (),
            //         Err(e) => return Err(e),
            //     }
            // }
        }

        for field in &current.fields {
            // TODO: lookup field.kind
        }

        for method in &current.methods {
            if method.body.is_none() {
                if !method.modifiers.contains(&modifier_abstract) &&
                   !method.modifiers.contains(&modifier_native) {
                    return Err(format!("concrete method {} has no body", method));
                }
            }
            // TODO: lookup each method.parameters
            // TODO: lookup method.return_type
            // TODO: if body, analyze method.body

            // TODO: analyze override:
            // match verify_override(env.kinds, current, &method) {
            //     Ok(_) => (),
            //     Err(e) => return Err(e),
            // }

            if method.modifiers.contains(&modifier_abstract) {
                if method.modifiers.contains(&modifier_final) {
                    return Err(format!("final method {} is abstract", method));
                }

                if method.modifiers.contains(&modifier_static) {
                    return Err(format!("static method {} is abstract", method));
                }
            }
        }
    }

    // TODO: ensure non-abstract class does not contain un-overriden abstract
    // methods or define new ones

    // TODO: check type of parameter lists

    Ok(())
}

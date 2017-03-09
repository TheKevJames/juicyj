use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::FieldEnvironment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

pub fn lookup_canonical(name: &ASTNode,
                        current: &ClassOrInterfaceEnvironment,
                        kinds: &Vec<ClassOrInterfaceEnvironment>)
                        -> Option<Result<ClassOrInterfaceEnvironment, String>> {
    let name = name.clone();
    for kind in kinds {
        if name == kind.name {
            return match verify_prefixes(name, current, kinds) {
                Ok(_) => Some(Ok(kind.clone())),
                Err(e) => Some(Err(e)),
            };
        }
    }

    None
}

pub fn lookup_in_package(name: &ASTNode,
                         current: &ClassOrInterfaceEnvironment,
                         kinds: &Vec<ClassOrInterfaceEnvironment>)
                         -> Option<Result<ClassOrInterfaceEnvironment, String>> {
    let mut found = None;

    for kind in kinds {
        if let Some((kind_name, kind_package)) = kind.name.children.split_last() {
            if let Some((_, package)) = current.name.children.split_last() {
                if package == kind_package && name == kind_name {
                    match found {
                        Some(_) => {
                            return Some(Err(format!("ambiguous type lookup in package {:?}",
                                                    package)))
                        }
                        None => found = Some(kind.clone()),
                    }
                }
            }
        }
    }
    match found {
        Some(f) => Some(Ok(f)),
        _ => None,
    }
}

// TODO: does this include inherited fields, etc?
// TODO: lookup("thing") is String (j1_stringliteralinvoke)
pub fn lookup(name: &ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>)
              -> Result<ClassOrInterfaceEnvironment, String> {
    let name = name.clone();
    let node_star = ASTNode {
        token: Token::new(TokenKind::Star, None),
        children: Vec::new(),
    };

    if let Some(l) = name.clone().token.lexeme {
        if l == "ArrayType" {
            let mut array = ClassOrInterfaceEnvironment::new(name.clone(), ClassOrInterface::CLASS);

            let object = ASTNode {
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
            array.extends.push(object);

            // array.fields clone is public

            let mut length = FieldEnvironment::new(ASTNode {
                                                       token: Token::new(TokenKind::Identifier,
                                                                         Some("length")),
                                                       children: Vec::new(),
                                                   },
                                                   ASTNode {
                                                       token: Token::new(TokenKind::Int, None),
                                                       children: Vec::new(),
                                                   });
            length.modifiers.push(ASTNode {
                token: Token::new(TokenKind::Public, None),
                children: Vec::new(),
            });
            length.modifiers.push(ASTNode {
                token: Token::new(TokenKind::Final, None),
                children: Vec::new(),
            });
            array.fields.push(length);

            return Ok(array);
        }
    }

    // 0. lookup canonical path
    match lookup_canonical(&name, current, kinds) {
        Some(x) => return x,
        None => (),
    }

    // 1. try the enclosing class or interface
    if let Some(class_name) = current.name.children.last() {
        if &name == class_name {
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
    let result = lookup_in_package(&name, current, kinds);
    if result.is_some() {
        return result.unwrap();
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

pub fn lookup_or_primitive(kind: &ASTNode,
                           current: &ClassOrInterfaceEnvironment,
                           kinds: &Vec<ClassOrInterfaceEnvironment>)
                           -> Result<ClassOrInterfaceEnvironment, String> {
    let mut child_kind = kind.clone();
    if let Some(l) = kind.clone().token.lexeme {
        if l == "ArrayType" {
            child_kind = kind.children[0].clone();
        }
    }
    if vec![TokenKind::Boolean,
            TokenKind::Byte,
            TokenKind::Char,
            TokenKind::Int,
            TokenKind::Short,
            TokenKind::Void]
        .contains(&child_kind.token.kind) {
        return Ok(ClassOrInterfaceEnvironment::new(kind.clone(), ClassOrInterface::CLASS));
    }

    lookup(&child_kind.flatten(), current, kinds)
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

        let mut testable = ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("Name")),
            children: prefix.clone(),
        };
        testable.flatten();
        if idx % 2 == 0 && testable != kind {
            if vec![TokenKind::Boolean,
                    TokenKind::Byte,
                    TokenKind::Char,
                    TokenKind::Int,
                    TokenKind::Short,
                    TokenKind::Void]
                .contains(&testable.token.kind) {
                return Err(format!("strict prefix {} resolves to primitive type", testable));
            }

            match lookup_canonical(&testable, current, kinds) {
                Some(Ok(_)) => {
                    return Err(format!("strict prefix {} resolves to canonical type", testable))
                }
                _ => (),
            }

            match lookup_in_package(&testable, current, kinds) {
                Some(Ok(ref cls)) if cls.name != kind => {
                    return Err(format!("strict prefix {} resolves to local type", testable))
                }
                _ => (),
            }
        }
    }

    Ok(())
}

pub fn verify_package_prefixes(kind: ASTNode,
                               current: &ClassOrInterfaceEnvironment,
                               kinds: &Vec<ClassOrInterfaceEnvironment>)
                               -> Result<(), String> {
    let mut prefix = Vec::new();
    for (idx, child) in kind.children.iter().enumerate() {
        prefix.push(child.clone());

        let mut testable = ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("Name")),
            children: prefix.clone(),
        };
        testable.flatten();
        if idx % 2 == 0 && testable != kind {
            if vec![TokenKind::Boolean,
                    TokenKind::Byte,
                    TokenKind::Char,
                    TokenKind::Int,
                    TokenKind::Short,
                    TokenKind::Void]
                .contains(&testable.token.kind) {
                return Err(format!("strict package prefix {} resolves to primitive type",
                                   testable));
            }

            match lookup_canonical(&testable, current, kinds) {
                Some(Ok(_)) => {
                    return Err(format!("strict package prefix {} resolves to canonical type",
                                       testable))
                }
                _ => (),
            }
        }
    }

    Ok(())
}

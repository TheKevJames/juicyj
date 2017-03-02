mod class;
mod classorinterface;
mod constructor;
mod field;
mod interface;
mod method;
mod variable;

use std::collections::HashMap;

use scanner::AST;
use scanner::Token;
use scanner::TokenKind;

// TODO: create common trait for Class/Interface Envs
use self::class::analyze_class_declaration;
pub use self::classorinterface::ClassOrInterfaceEnvironment;
use self::interface::analyze_interface_declaration;

#[derive(Clone,Debug)]
pub struct Environment {
    kinds: Vec<ClassOrInterfaceEnvironment>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { kinds: Vec::new() }
    }

    pub fn annotate_asts(trees: &Vec<AST>) -> Result<(), String> {
        let mut env = Environment::new();

        let mut dependencies = HashMap::new();
        for tree in trees {
            let mut imports = Vec::new();
            for import in &tree.imports {
                if tree.canonical ==
                   vec![Token::new(TokenKind::Identifier, Some("java")),
                        Token::new(TokenKind::Identifier, Some("io")),
                        Token::new(TokenKind::Identifier, Some("PrintStream"))] ||
                   tree.canonical ==
                   vec![Token::new(TokenKind::Identifier, Some("java")),
                        Token::new(TokenKind::Identifier, Some("util")),
                        Token::new(TokenKind::Identifier, Some("Arrays"))] {
                    break;
                }
                imports.push(import.import.clone());
            }

            dependencies.insert(tree.canonical.clone(), imports);
        }

        let mut ordered_canonicals = Vec::new();
        while !dependencies.clone().is_empty() {
            let mut acyclic = false;
            for (canonical, edges) in dependencies.clone().iter() {
                let mut delete = true;
                for edge in edges {
                    if dependencies.contains_key(edge) {
                        delete = false;
                        break;
                    } else if edge.last().unwrap().kind == TokenKind::Star {
                        let mut dpackages = Vec::new();
                        for dependency in dependencies.keys() {
                            dpackages.push(dependency[0..dependency.len() - 1].to_vec());
                        }

                        let epackage = &edge[0..edge.len() - 1].to_vec();

                        if dpackages.contains(epackage) {
                            delete = false;
                            break;
                        }
                    }
                }
                if delete {
                    acyclic = true;
                    dependencies.remove(canonical);
                    ordered_canonicals.push(canonical.clone());
                }
            }
            if !acyclic {
                return Err("cyclic hierarchy detected".to_owned());
            }
        }

        let mut ordered_trees = Vec::new();
        for canonical in ordered_canonicals {
            for tree in trees {
                if tree.canonical == canonical {
                    ordered_trees.push(tree);
                }
            }
        }

        for tree in ordered_trees {
            let root = match tree.root {
                Some(ref r) => r,
                None => continue,
            };

            let result = match root.token.lexeme {
                Some(ref l) if l == "ClassDeclaration" => {
                    analyze_class_declaration(&tree.canonical, &mut env.kinds, &tree.imports, &root)
                }
                Some(ref l) if l == "InterfaceDeclaration" => {
                    analyze_interface_declaration(&tree.canonical,
                                                  &mut env.kinds,
                                                  &tree.imports,
                                                  &root)
                }
                _ => Ok(()),
            };
            if result.is_err() {
                return result;
            }
        }

        Ok(())
    }
}

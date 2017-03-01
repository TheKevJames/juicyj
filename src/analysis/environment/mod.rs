mod class;
mod constructor;
mod field;
mod interface;
mod method;
mod variable;

use std::collections::HashMap;

use scanner::AST;

use self::class::analyze_class_declaration;
use self::class::ClassEnvironment;
use self::interface::analyze_interface_declaration;
use self::interface::InterfaceEnvironment;

#[derive(Clone,Debug)]
pub struct Environment {
    classes: Vec<ClassEnvironment>,
    interfaces: Vec<InterfaceEnvironment>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            classes: Vec::new(),
            interfaces: Vec::new(),
        }
    }

    pub fn annotate_asts(trees: &Vec<AST>) -> Result<(), String> {
        let mut env = Environment::new();

        let mut dependencies = HashMap::new();
        for tree in trees {
            let mut import_vec = Vec::new();
            for import in &tree.imports {
                let token = import.import.last().unwrap();
                // TODO: "*" is probably wrong
                import_vec.push(token.clone().lexeme.unwrap_or("*".to_owned()));
            }
            dependencies.insert(tree.filename.clone(), import_vec);
        }

        let mut ordered_files = Vec::new();
        while !dependencies.clone().is_empty() {
            let mut acyclic = false;
            for (node, edges) in dependencies.clone().iter() {
                let mut delete = true;
                for edge in edges {
                    if dependencies.contains_key(edge) {
                        delete = false;
                        break;
                    }
                }
                if delete {
                    acyclic = true;
                    dependencies.remove(node);
                    ordered_files.push(node.clone());
                }
            }
            if !acyclic {
                return Err("cyclic or invalid imports".to_owned());
            }
        }

        let mut ordered_trees = Vec::new();
        for filename in ordered_files {
            for tree in trees {
                if tree.filename == filename {
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
                    analyze_class_declaration(&mut env.classes, &env.interfaces, &root)
                }
                Some(ref l) if l == "InterfaceDeclaration" => {
                    analyze_interface_declaration(&env.classes, &mut env.interfaces, &root)
                }
                _ => Ok(()),
            };
            if result.is_err() {
                return result;
            }
        }

        // println!("{:#?}", env);

        Ok(())
    }
}

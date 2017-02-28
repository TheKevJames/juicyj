mod class;
mod constructor;
mod field;
mod interface;
mod method;

use scanner::AST;

use self::class::analyze_class_declaration;
use self::class::ClassEnvironment;
use self::interface::analyze_interface_declaration;
use self::interface::InterfaceEnvironment;
use std::collections::HashMap;

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
                let mut import_string = "".to_owned();
                for token in &import.import {
                    match &token.lexeme {
                        &Some(ref l) => import_string += &l,
                        &None => import_string += ".",
                    }
                }
                import_vec.push(import_string);
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
                return Err("Cyclic or invalid imports".to_owned());
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

            match root.token.lexeme {
                Some(ref l) if l == "ClassDeclaration" => {
                    match analyze_class_declaration(&mut env.classes, &env.interfaces, &root) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    };
                }
                Some(ref l) if l == "InterfaceDeclaration" => {
                    match analyze_interface_declaration(&env.classes, &mut env.interfaces, &root) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    };
                }
                _ => (),
            }
        }

        // println!("{:#?}", env);

        Ok(())
    }
}

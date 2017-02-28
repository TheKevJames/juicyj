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

        for tree in trees {
            println!("{}", tree);
        }

        // TODO: check imports for ordering and circular dependencies
        // for tree in trees {
        //     for import in &tree.imports {
        //     }
        // }

        // TODO: iterate in order specified above
        for tree in trees {
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

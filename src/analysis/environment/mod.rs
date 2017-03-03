mod class;
mod classorinterface;
mod constructor;
mod field;
mod interface;
mod method;
mod variable;

use scanner::AST;

use self::class::analyze_class_declaration;
pub use self::classorinterface::ClassOrInterface;
pub use self::classorinterface::ClassOrInterfaceEnvironment;
use self::interface::analyze_interface_declaration;
pub use self::variable::VariableEnvironment;

#[derive(Clone,Debug)]
pub struct Environment {
    pub kinds: Vec<ClassOrInterfaceEnvironment>,
}

impl Environment {
    pub fn new(trees: &Vec<AST>) -> Result<Environment, String> {
        let mut env = Environment { kinds: Vec::new() };

        for tree in trees {
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
            match result {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            'import: for import in &tree.imports {
                let mut import_package = import.import.clone();
                import_package.children.pop();
                import_package.children.pop();

                for kind in trees {
                    if kind.package.package.children.starts_with(&import_package.children) {
                        continue 'import;
                    }
                }

                return Err(format!("could not find imported package {}", import_package));
            }
        }

        Ok(env)
    }
}

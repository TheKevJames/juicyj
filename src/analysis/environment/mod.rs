mod constructor;
mod field;
mod method;

use scanner::AST;
use scanner::ASTNode;

use self::constructor::analyze_constructor_declaration;
use self::constructor::ConstructorEnvironment;
use self::field::analyze_constant_declaration;
use self::field::analyze_field_declaration;
use self::field::FieldEnvironment;
use self::method::analyze_abstract_method_declaration;
use self::method::analyze_method_declaration;
use self::method::MethodEnvironment;

#[derive(Clone,Debug)]
pub struct ClassEnvironment {
    modifiers: Vec<ASTNode>,
    name: ASTNode,
    extends: Vec<ASTNode>,
    implements: Vec<ASTNode>,
    constructors: Vec<ConstructorEnvironment>,
    fields: Vec<FieldEnvironment>,
    methods: Vec<MethodEnvironment>,
}

#[derive(Clone,Debug)]
pub struct InterfaceEnvironment {
    modifiers: Vec<ASTNode>,
    name: ASTNode,
    extends: Vec<ASTNode>,
    fields: Vec<FieldEnvironment>,
    methods: Vec<MethodEnvironment>,
}

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

        println!("{}", trees[0]);

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
                    let mut modifiers = Vec::new();
                    for child in root.children[0].clone().children {
                        modifiers.push(child);
                    }

                    let name = root.children[2].clone();

                    let mut extends = Vec::new();
                    let mut implements = Vec::new();
                    let mut constructors = Vec::new();
                    let mut fields = Vec::new();
                    let mut methods = Vec::new();
                    for (idx, child) in root.children.iter().enumerate() {
                        if idx < 3 {
                            continue;
                        }

                        match child.token.lexeme {
                            Some(ref le) if le == "Implements" => {
                                // TODO: classes can't implement classes
                                // TODO: inheritance must not be circular
                                // TODO: no repeats
                                let mut grandkid = child.children[1].clone();
                                while grandkid.clone().token.lexeme.unwrap_or("".to_owned()) !=
                                      "Name" {
                                    implements.push(grandkid.children[2].clone());
                                    grandkid = grandkid.children[0].clone();
                                }
                                implements.push(grandkid.clone());
                            }
                            Some(ref le) if le == "ClassExtends" => {
                                // TODO: classes extend Object is nothing else
                                // TODO: classes can't extend interfaces
                                // TODO: classes can't extend final classes
                                // TODO: extension must not be circular
                                extends.push(child.children[1].clone());
                            }
                            Some(ref le) if le == "ClassBody" && child.children.len() == 3 => {
                                let mut decls = child.children[1].clone();
                                while decls.clone().token.lexeme.unwrap_or("".to_owned()) ==
                                      "ClassBodyDeclarations" {
                                    match decls.children[1].clone().token.lexeme {
                                        Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                                            match analyze_abstract_method_declaration(
                                                    &mut methods,
                                                    &decls.children[1].children[0]) {
                                                Ok(_) => (),
                                                Err(e) => return Err(e),
                                            };
                                        }
                                        Some(ref lex) if lex == "ConstructorDeclaration" => {
                                            match analyze_constructor_declaration(
                                                    &mut constructors,
                                                    &decls.children[1]) {
                                                Ok(_) => (),
                                                Err(e) => return Err(e),
                                            };
                                        }
                                        Some(ref lex) if lex == "FieldDeclaration" => {
                                            match analyze_field_declaration(&mut fields,
                                                                            &decls.children[1]) {
                                                Ok(_) => (),
                                                Err(e) => return Err(e),
                                            };
                                        }
                                        Some(ref lex) if lex == "MethodDeclaration" => {
                                            match analyze_method_declaration(&mut methods,
                                                                             &decls.children[1]
                                                                                 .children
                                                                                  [0]) {
                                                Ok(_) => (),
                                                Err(e) => return Err(e),
                                            };
                                        }
                                        _ => (),
                                    }
                                    decls = decls.children[0].clone();
                                // }
                                match decls.token.lexeme {
                                    Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                                        match analyze_abstract_method_declaration(
                                                &mut methods,
                                                &decls.children[0]) {
                                            Ok(_) => (),
                                            Err(e) => return Err(e),
                                        };
                                    }
                                    Some(ref lex) if lex == "ConstructorDeclaration" => {
                                        match analyze_constructor_declaration(&mut constructors,
                                                                              &decls) {
                                            Ok(_) => (),
                                            Err(e) => return Err(e),
                                        };
                                    }
                                    Some(ref lex) if lex == "FieldDeclaration" => {
                                        match analyze_field_declaration(&mut fields, &decls) {
                                            Ok(_) => (),
                                            Err(e) => return Err(e),
                                        };
                                    }
                                    Some(ref lex) if lex == "MethodDeclaration" => {
                                        match analyze_method_declaration(&mut methods,
                                                                         &decls.children[0]) {
                                            Ok(_) => (),
                                            Err(e) => return Err(e),
                                        };
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    }

                    // TODO: class/interface names must be unique
                    env.classes.push(ClassEnvironment {
                        modifiers: modifiers,
                        name: name,
                        extends: extends,
                        implements: implements,
                        constructors: constructors,
                        fields: fields,
                        methods: methods,
                    });
                }
                Some(ref l) if l == "InterfaceDeclaration" => {
                    let mut modifiers = Vec::new();
                    for child in root.children[0].clone().children {
                        modifiers.push(child);
                    }

                    let name = root.children[2].clone();

                    let mut extends = Vec::new();
                    let mut fields = Vec::new();
                    let mut methods = Vec::new();
                    match root.children[3].token.lexeme {
                        Some(ref l) if l == "InterfaceExtends" => {
                            // TODO: interfaces extend Object is nothing else
                            // TODO: no repeats
                            // TODO: extension must not be circular
                            let mut grandkid = root.children[3].children[1].clone();
                            while grandkid.clone().token.lexeme.unwrap_or("".to_owned()) != "Name" {
                                extends.push(grandkid.children[2].clone());
                                grandkid = grandkid.children[0].clone();
                            }
                            extends.push(grandkid.clone());
                        }
                        Some(ref le) if le == "InterfaceBody" &&
                                        root.children[3].children.len() == 3 => {
                            let mut decls = root.children[3].clone().children[1].clone();
                            while decls.clone().token.lexeme.unwrap_or("".to_owned()) ==
                                  "InterfaceMemberDeclarations" {
                                match decls.children[1].clone().token.lexeme {
                                    Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                                        match analyze_abstract_method_declaration(
                                                &mut methods,
                                                &decls.children[1].children[0]) {
                                            Ok(_) => (),
                                            Err(e) => return Err(e),
                                        };
                                    }
                                    Some(ref lex) if lex == "ConstantDeclaration" => {
                                        match analyze_constant_declaration(&mut fields,
                                                                           &decls.children[1]) {
                                            Ok(_) => (),
                                            Err(e) => return Err(e),
                                        };
                                    }
                                    _ => (),
                                }
                                decls = decls.children[0].clone();
                            }
                            match decls.token.lexeme {
                                Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                                    match analyze_abstract_method_declaration(&mut methods,
                                                                              &decls.children[0]) {
                                        Ok(_) => (),
                                        Err(e) => return Err(e),
                                    };
                                }
                                Some(ref lex) if lex == "ConstantDeclaration" => {
                                    match analyze_constant_declaration(&mut fields, &decls) {
                                        Ok(_) => (),
                                        Err(e) => return Err(e),
                                    };
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }

                    // TODO: class/interface names must be unique
                    env.interfaces.push(InterfaceEnvironment {
                        modifiers: modifiers,
                        name: name,
                        extends: extends,
                        fields: fields,
                        methods: methods,
                    });
                }
                _ => (),
            }
        }

        // println!("{:#?}", env);

        Ok(())
    }
}

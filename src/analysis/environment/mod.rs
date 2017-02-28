use scanner::AST;
use scanner::ASTNode;

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
pub struct ConstructorEnvironment {
    modifiers: Vec<ASTNode>,
    name: ASTNode,
    parameters: Vec<ASTNode>,
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
pub struct FieldEnvironment {
    modifiers: Vec<ASTNode>,
    kind: ASTNode,
    name: ASTNode,
}

#[derive(Clone,Debug)]
pub struct MethodEnvironment {
    modifiers: Vec<ASTNode>,
    return_type: ASTNode,
    name: ASTNode,
    parameters: Vec<ASTNode>,
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

        // TODO: check imports for ordering and circular dependencies
        // for tree in &trees {
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
                                // TODO: classes can't extend interfaces
                                // TODO: classes can't extend final classes
                                extends.push(child.children[1].clone());
                            }
                            Some(ref le) if le == "ClassBody" && child.children.len() == 3 => {
                                for decl in child.children[1].clone().children {
                                    match decl.token.lexeme {
                                        Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                                            // TODO: a class with an abstract method must be abstract itself
                                            let header = decl.children[0].clone();

                                            let mut local_modifiers = Vec::new();
                                            for kid in header.children[0].clone().children {
                                                local_modifiers.push(kid);
                                            }

                                            let local_type = header.children[1].clone();
                                            let local_name = header.children[2].clone().children[0]
                                                .clone();

                                            let mut local_params = Vec::new();
                                            if header.children[2].children.len() == 4 {
                                                let mut local_param =
                                                    header.children[2].clone().children[2].clone();
                                                while local_param.clone()
                                                    .token
                                                    .lexeme
                                                    .unwrap_or("".to_owned()) !=
                                                      "Parameter" {
                                                    local_params.push(local_param.children[2].clone());
                                                    local_param = local_param.children[0].clone();
                                                }
                                                local_params.push(local_param.clone());
                                            }

                                            methods.push(MethodEnvironment {
                                                modifiers: local_modifiers,
                                                return_type: local_type,
                                                name: local_name,
                                                parameters: local_params,
                                            });
                                        }
                                        Some(ref lex) if lex == "ConstructorDeclaration" => {
                                            // TODO: constructor signatures must be unique
                                            let mut local_modifiers = Vec::new();
                                            for kid in decl.children[0].clone().children {
                                                local_modifiers.push(kid);
                                            }

                                            let local_name = decl.children[1].clone().children[0]
                                                .clone();

                                            let mut local_params = Vec::new();
                                            if decl.children[1].children.len() == 4 {
                                                let mut local_param =
                                                    decl.children[1].clone().children[2].clone();
                                                while local_param.clone()
                                                    .token
                                                    .lexeme
                                                    .unwrap_or("".to_owned()) !=
                                                      "Parameter" {
                                                    local_params.push(local_param.children[2].clone());
                                                    local_param = local_param.children[0].clone();
                                                }
                                                local_params.push(local_param.clone());
                                            }

                                            constructors.push(ConstructorEnvironment {
                                                modifiers: local_modifiers,
                                                name: local_name,
                                                parameters: local_params,
                                            });
                                        }
                                        Some(ref lex) if lex == "FieldDeclaration" => {
                                            // TODO: fields names must be unique
                                            let mut local_modifiers = Vec::new();
                                            for kid in decl.children[0].clone().children {
                                                local_modifiers.push(kid);
                                            }

                                            let local_type = decl.children[1].clone();
                                            let local_name = decl.children[2].clone().children[0]
                                                .clone();

                                            fields.push(FieldEnvironment {
                                                modifiers: local_modifiers,
                                                kind: local_type,
                                                name: local_name,
                                            });
                                        }
                                        Some(ref lex) if lex == "MethodDeclaration" => {
                                            // TODO: method signature must be unique
                                            // TODO: method signatures must not vary only in return type
                                            // TODO: if non-static, cannot override static
                                            // TODO: cannot override method with different return type
                                            // TODO: cannot override permissions with looser permissions
                                            // TODO: cannot override final method
                                            let header = decl.children[0].clone();

                                            let mut local_modifiers = Vec::new();
                                            for kid in header.children[0].clone().children {
                                                local_modifiers.push(kid);
                                            }

                                            let local_type = header.children[1].clone();
                                            let local_name = header.children[2].clone().children[0]
                                                .clone();

                                            let mut local_params = Vec::new();
                                            if header.children[2].children.len() == 4 {
                                                let mut local_param =
                                                    header.children[2].clone().children[2].clone();
                                                while local_param.clone()
                                                    .token
                                                    .lexeme
                                                    .unwrap_or("".to_owned()) !=
                                                      "Parameter" {
                                                    local_params.push(local_param.children[2].clone());
                                                    local_param = local_param.children[0].clone();
                                                }
                                                local_params.push(local_param.clone());
                                            }

                                            methods.push(MethodEnvironment {
                                                modifiers: local_modifiers,
                                                return_type: local_type,
                                                name: local_name,
                                                parameters: local_params,
                                            });
                                        }
                                        _ => (),
                                    }
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
                            // TODO: no repeats
                            let mut grandkid = root.children[3].children[1].clone();
                            while grandkid.clone().token.lexeme.unwrap_or("".to_owned()) != "Name" {
                                extends.push(grandkid.children[2].clone());
                                grandkid = grandkid.children[0].clone();
                            }
                            extends.push(grandkid.clone());
                        }
                        Some(ref le) if le == "InterfaceBody" &&
                                        root.children[3].children.len() == 3 => {
                            for decl in root.children[3].children[1].clone().children {
                                match decl.token.lexeme {
                                    Some(ref lex) if lex == "AbstractMethodDeclaration" => {
                                        let header = decl.children[0].clone();

                                        let mut local_modifiers = Vec::new();
                                        for kid in header.children[0].clone().children {
                                            local_modifiers.push(kid);
                                        }

                                        let local_type = header.children[1].clone();
                                        let local_name = header.children[2].clone().children[0]
                                            .clone();

                                        let mut local_params = Vec::new();
                                        if header.children[2].children.len() == 4 {
                                            let mut local_param =
                                                header.children[2].clone().children[2].clone();
                                            while local_param.clone()
                                                .token
                                                .lexeme
                                                .unwrap_or("".to_owned()) !=
                                                  "Parameter" {
                                                local_params.push(local_param.children[2].clone());
                                                local_param = local_param.children[0].clone();
                                            }
                                            local_params.push(local_param.clone());
                                        }

                                        methods.push(MethodEnvironment {
                                            modifiers: local_modifiers,
                                            return_type: local_type,
                                            name: local_name,
                                            parameters: local_params,
                                        });
                                    }
                                    Some(ref lex) if lex == "ConstantDeclaration" => {
                                        let mut local_modifiers = Vec::new();
                                        for kid in decl.children[0].clone().children {
                                            local_modifiers.push(kid);
                                        }

                                        let local_type = decl.children[1].clone();
                                        let local_name = decl.children[2].clone().children[0]
                                            .clone();

                                        fields.push(FieldEnvironment {
                                            modifiers: local_modifiers,
                                            kind: local_type,
                                            name: local_name,
                                        });
                                    }
                                    _ => (),
                                }
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

        println!("{:#?}", env);

        Ok(())
    }
}

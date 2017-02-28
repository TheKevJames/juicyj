use scanner::ast::AST;

impl environment {
    pub fn check(tree: &AST) -> Result<&Vec<string> > {
        let mut vec = Vec::new();
        vec.push(Vec::new()) //classes
        vec.push(Vec::new()) //interfaces
        vec.push(Vec::new()) //fields
        vec.push(Vec::new()) //methods
        vec.push(Vec::new()) //local vars
        vec.push(Vec::new()) //formal parameters

        for node in &tree.root.children {
            match node.token.lexeme {
                Some(ref l) if l == "Imports" => {
                    //Check imports for ordering and circular dependencies
                }

                Some(ref l) if l == "ClassDeclaration" => {
                    let mut attributes = Vec<string>; //Where should we store these?
                                                      //Includes absinalitive and extend/implement
                    let mut className = "";
                    for child in node.children {
                        while (child.token.kind != Token.Class) {
                            attributes.push(child.token.lexeme);
                            continue;
                        }
                        match child.token.kind {
                            Some(ref j) if j == Token.LBrace {
                                break; //Done, our class is now declared
                            }
                            Some(ref j) if j == Token.ClassExtends {
                                let mut extend = "";
                                while (child.token.kind != Token.ClassBody) {
                                    while(child.token.kind != Token.Name) {
                                        continue;
                                    }
                                    extend += child.token.lexeme;
                                    continue;
                                }
                                //classes can't extend interfaces
                                if let Some(_) = vec[1].iter().position(|&i| i == extend) {
                                    Err
                                }
                                //classes can't extend final classes
                                if let Some(_) = attributes.iter().position(|&i| i == "final") {
                                    Err
                                }                                
                                attributes.push(extend);
                            }
                            Some(ref j) if j == Token.Implements {
                                let mut implement = "";
                                while (child.token.kind != Token.ClassBody) {
                                    while(child.token.kind != Token.Name) {
                                        continue;
                                    }
                                    implement += child.token.lexeme;
                                    continue;
                                }
                                //classes can't implement classes
                                if let Some(_) = vec[0].iter().position(|&i| i == implement) {
                                    Err
                                }
                                attributes.push(implement);
                            }
                            Some(ref j) if j != Token.Identifier {
                                continue;
                            }
                            className += child.token.lexeme;
                        }
                    }
                    vec[0].push(className);
                }

                Some(ref l) if l == "InterfaceDeclaration" => {
                    let mut attributes = Vec<string>; //Where should we store these?
                                                      //Includes absinalitive and extend/implement
                    let mut interfaceName = "";
                    for child in node.children {
                        while (child.token.kind != Token.Class) {
                            attributes.push(child.token.lexeme);
                            continue;
                        }
                        match child.token.kind {
                            Some(ref j) if j == Token.LBrace {
                                break; //Done, our interface is declared
                            }
                            Some(ref j) if j == Token.ClassExtends {
                                let mut extend = "";
                                while (child.token.kind != Token.ClassBody) {
                                    while(child.token.kind != Token.Name) {
                                        continue;
                                    }
                                    extend += child.token.lexeme;
                                    
                                    continue;
                                }
                                //interfaces can't extend classes
                                if let Some(_) = vec[0].iter().position(|&i| i == extend) {
                                    Err
                                }
                                attributes.push(extend);
                            }
                            Some(ref j) if j != Token.Identifier {
                                continue;
                            }
                            interfaceName += child.token.lexeme;
                        }
                    }
                    vec[1].push(interfaceName) 
                }

                Some(ref l) if l == "FieldDeclaration" {
                    for child in node.children {
                        match child.token.kind {
                            Some(ref j) if j == Token.Identifier {
                                vec[2].push(chlidchild.token.lexeme);
                                break;
                            }
                        }
                    }
                }

                Some(ref l) if l == "MethodDeclaration" {
                    for child in node.children {
                        match child.token.kind {
                            Some(ref j) if j == Token.Abstract {
                                //TODO: Ensure this class is abstract
                            }
                            Some(ref j) if j == Token.Protected {
                                //TODO: Ensure this does not replace a public method
                            }
                            Some(ref j) if j == Token.Identifier {
                                //TODO: Also check parameters
                                //cannot declare duplicate methods
                                if let Some(_) = vec[3].iter().position(|&i| i == child.token.lexeme) {
                                    Err
                                }
                                vec[3].push(child.token.lexeme);
                                break;
                            }
                        }
                    }
                }

                Some(ref l) if l == "LocalVariableDeclaration" {
                    for child in node.children {
                        match child.token.kind {
                            Some(ref j) if j == Token.Identifier {
                                vec[4].push(child.token.lexeme);
                                break;
                            }
                        }
                    }
                }

                Some(ref l) if l == "ParameterList" {
                    for child in node.children {
                        match child.token.lexeme {
                            Some(ref j) if j == "Parameter" {
                                for param in child {
                                    match param.token.kind {
                                        Some(ref k) if k == Token.Identifier {
                                            vec[5].push(param.token.lexeme)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                

                Some(ref l) if l == "RBrace" => {
                    //Environments built, check restrictions

                    //No two fields declared in the same class may have the same name.
                    vec[2].sort();
                    for (int i = 0; i < vec[2].length()-1; i++) {
                        if (vec[2][i] == vec[2][i+1]) {
                            Err 42;
                        }
                    }

                    //No two local variables with overlapping scope have the same name.
                    vec[4].sort();
                    for (int i = 0; i < vec[4].length()-1; i++) {
                        if (vec[4][i] == vec[4][i+1]) {
                            Err 42;
                        }
                    }

                    return;
                }
                Some(ref l) if l == "LBrace" => {
                    self.check(node) //Run on the node until we hit a closebrace
                }

                _ => continue 
            }
        }

        node.environment = vec;

        for env in vec {
            env.clear();
        }
    }
}
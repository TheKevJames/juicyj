use std::fmt;

use analysis::environment::field::FieldEnvironment;
use analysis::environment::method::MethodEnvironment;
use analysis::types::lookup;
use scanner::ASTNode;
use scanner::ASTNodeImport;
use scanner::Token;
use scanner::TokenKind;

#[derive(Clone,Debug,PartialEq)]
pub enum ClassOrInterface {
    CLASS,
    INTERFACE,
}

#[derive(Clone,Debug)]
pub struct ClassOrInterfaceEnvironment {
    pub constructors: Vec<MethodEnvironment>,
    pub extends: Vec<ASTNode>,
    pub fields: Vec<FieldEnvironment>,
    pub implements: Vec<ASTNode>,
    pub imports: Vec<ASTNodeImport>,
    pub methods: Vec<MethodEnvironment>,
    pub modifiers: Vec<ASTNode>,
    pub name: ASTNode,
    pub kind: ClassOrInterface,
}

impl ClassOrInterfaceEnvironment {
    pub fn new(name: ASTNode, kind: ClassOrInterface) -> ClassOrInterfaceEnvironment {
        ClassOrInterfaceEnvironment {
            constructors: Vec::new(),
            extends: Vec::new(),
            fields: Vec::new(),
            implements: Vec::new(),
            imports: Vec::new(),
            kind: kind,
            methods: Vec::new(),
            modifiers: Vec::new(),
            name: name,
        }
    }

    /// This method is used for building subclasses. If `A extends B, C`, we
    /// can build a new subclass by applying this function to `A`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let A: ClassOrInterfaceEnvironment;
    /// let B: ClassOrInterfaceEnvironment;
    /// let C: ClassOrInterfaceEnvironment;
    ///
    /// let mut child = ClassOrInterfaceEnvironment::new(A.name, A.kind);
    /// child.inherit(B);
    /// child.inherit(C);
    /// child.apply(A);
    /// ```
    pub fn apply(&mut self,
                 child: &ClassOrInterfaceEnvironment,
                 kinds: &Vec<ClassOrInterfaceEnvironment>)
                 -> Result<(), String> {
        let modifier_final = ASTNode {
            token: Token::new(TokenKind::Final, None),
            children: Vec::new(),
        };
        let modifier_static = ASTNode {
            token: Token::new(TokenKind::Static, None),
            children: Vec::new(),
        };
        let public = ASTNode {
            token: Token::new(TokenKind::Public, None),
            children: Vec::new(),
        };
        let protected = ASTNode {
            token: Token::new(TokenKind::Protected, None),
            children: Vec::new(),
        };
        let private = ASTNode {
            token: Token::new(TokenKind::Private, None),
            children: Vec::new(),
        };

        for constructor in &child.constructors {
            for (idx, existing) in self.constructors.clone().iter().enumerate() {
                let mut different = constructor.parameters.len() != existing.parameters.len();
                if different {
                    continue;
                }

                for (constructor_param, existing_param) in
                    constructor
                        .parameters
                        .iter()
                        .zip(existing.parameters.iter()) {
                    let found_constructor_param =
                        match lookup::class::in_env(&constructor_param.kind, child, kinds) {
                            Ok(mp) => mp,
                            Err(e) => return Err(e),
                        };
                    let found_existing_param =
                        match lookup::class::in_env(&existing_param.kind, child, kinds) {
                            Ok(mp) => mp,
                            Err(e) => return Err(e),
                        };
                    if found_constructor_param.name != found_existing_param.name {
                        different = true;
                        break;
                    }
                }
                if !different {
                    self.constructors.remove(idx);
                }
            }

            self.constructors.push(constructor.clone());
        }
        for field in &child.fields {
            for (idx, existing) in self.fields.clone().iter().enumerate() {
                if field.name == existing.name {
                    self.fields.remove(idx);
                }
            }

            self.fields.push(field.clone());
        }
        for method in &child.methods {
            for (idx, existing) in self.methods.clone().iter().enumerate() {
                if method.name != existing.name {
                    continue;
                }

                let mut different = method.parameters.len() != existing.parameters.len();
                if different {
                    continue;
                }

                for (method_param, existing_param) in
                    method.parameters.iter().zip(existing.parameters.iter()) {
                    let found_method_param =
                        match lookup::class::in_env(&method_param.kind, child, kinds) {
                            Ok(mp) => mp,
                            Err(e) => return Err(e),
                        };
                    let found_existing_param =
                        match lookup::class::in_env(&existing_param.kind, child, kinds) {
                            Ok(mp) => mp,
                            Err(e) => return Err(e),
                        };
                    if found_method_param.name != found_existing_param.name {
                        different = true;
                        break;
                    }
                }
                if !different {
                    let method_return_type =
                        match lookup::class::in_env(&method.return_type, child, kinds) {
                            Ok(rt) => rt,
                            Err(e) => return Err(e),
                        };
                    let existing_return_type =
                        match lookup::class::in_env(&existing.return_type, child, kinds) {
                            Ok(rt) => rt,
                            Err(e) => return Err(e),
                        };
                    if method_return_type.name != existing_return_type.name {
                        return Err(format!("cannot override method {} with different return type",
                                           method.name));
                    }

                    if existing.modifiers.contains(&modifier_final) &&
                       existing.body != method.body {
                        return Err(format!("cannot override final method {}", existing.name));
                    }

                    if existing.modifiers.contains(&public) &&
                       (method.modifiers.contains(&protected) ||
                        method.modifiers.contains(&private)) {
                        return Err(format!("cannot weaken perms of public method {}",
                                           existing.name));
                    } else if existing.modifiers.contains(&protected) &&
                              method.modifiers.contains(&private) {
                        return Err(format!("cannot weaken perms of protected method {}",
                                           existing.name));
                    }

                    if existing.modifiers.contains(&modifier_static) !=
                       method.modifiers.contains(&modifier_static) {
                        return Err(format!("cannot override method {} with different static",
                                           existing.name));
                    }

                    self.methods.remove(idx);
                }
            }

            self.methods.push(method.clone());
        }

        self.extends = child.extends.clone();
        self.implements = child.implements.clone();
        self.imports = child.imports.clone();
        self.modifiers = child.modifiers.clone();

        Ok(())
    }

    /// This method is used for building subclasses. If `A extends B, C`, we
    /// can build a new subclass by applying this function to both `B` and `C`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let A: ClassOrInterfaceEnvironment;
    /// let B: ClassOrInterfaceEnvironment;
    /// let C: ClassOrInterfaceEnvironment;
    ///
    /// let mut child = ClassOrInterfaceEnvironment::new(A.name, A.kind);
    /// child.inherit(B);
    /// child.inherit(C);
    /// child.apply(A);
    /// ```
    pub fn inherit(&mut self,
                   parent: &ClassOrInterfaceEnvironment,
                   kinds: &Vec<ClassOrInterfaceEnvironment>)
                   -> Result<(), String> {
        let modifier_abstract = ASTNode {
            token: Token::new(TokenKind::Abstract, None),
            children: Vec::new(),
        };
        let modifier_final = ASTNode {
            token: Token::new(TokenKind::Final, None),
            children: Vec::new(),
        };
        let modifier_static = ASTNode {
            token: Token::new(TokenKind::Static, None),
            children: Vec::new(),
        };
        let public = ASTNode {
            token: Token::new(TokenKind::Public, None),
            children: Vec::new(),
        };
        let protected = ASTNode {
            token: Token::new(TokenKind::Protected, None),
            children: Vec::new(),
        };
        let private = ASTNode {
            token: Token::new(TokenKind::Private, None),
            children: Vec::new(),
        };
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

        for field in &parent.fields {
            self.fields.push(field.clone());
        }

        if self.kind == ClassOrInterface::INTERFACE && parent.name == object {
            for method in &parent.methods {
                let mut new = method.clone();
                new.modifiers.push(modifier_abstract.clone());
                self.methods.push(new);
            }

            return Ok(());
        }

        for method in &parent.methods {
            let mut overwrite = true;
            for (idx, existing) in self.methods.clone().iter().enumerate() {
                if method.name != existing.name {
                    continue;
                }

                let mut different = method.parameters.len() != existing.parameters.len();
                if different {
                    continue;
                }

                for (method_param, existing_param) in
                    method.parameters.iter().zip(existing.parameters.iter()) {
                    let found_method_param =
                        match lookup::class::in_env(&method_param.kind, parent, kinds) {
                            Ok(mp) => mp,
                            Err(e) => return Err(e),
                        };
                    let found_existing_param =
                        match lookup::class::in_env(&existing_param.kind, parent, kinds) {
                            Ok(mp) => mp,
                            Err(e) => return Err(e),
                        };
                    if found_method_param.name != found_existing_param.name {
                        different = true;
                        break;
                    }
                }
                if !different {
                    let method_return_type =
                        match lookup::class::in_env(&method.return_type, parent, kinds) {
                            Ok(rt) => rt,
                            Err(e) => return Err(e),
                        };
                    let existing_return_type =
                        match lookup::class::in_env(&existing.return_type, parent, kinds) {
                            Ok(rt) => rt,
                            Err(e) => return Err(e),
                        };
                    if method_return_type.name != existing_return_type.name {
                        return Err(format!("could not inherit methods {} with conflicting returns",
                                           method.name));
                    }

                    if existing.modifiers.contains(&modifier_abstract) {
                        if !method.modifiers.contains(&modifier_abstract) {
                            if existing.modifiers.contains(&modifier_final) &&
                               existing.body != method.body {
                                return Err(format!("cannot override final method {}",
                                                   existing.name));
                            }

                            if existing.modifiers.contains(&public) &&
                               (method.modifiers.contains(&protected) ||
                                method.modifiers.contains(&private)) {
                                return Err(format!("cannot weaken perms of public method {}",
                                                   existing.name));
                            } else if existing.modifiers.contains(&protected) &&
                                      method.modifiers.contains(&private) {
                                return Err(format!("cannot weaken perms of protected method {}",
                                                   existing.name));
                            }

                            if existing.modifiers.contains(&modifier_static) !=
                               method.modifiers.contains(&modifier_static) {
                                return Err(format!("cannot override staticity of method {}",
                                                   existing.name));
                            }

                            self.methods.remove(idx);
                        } else {
                            overwrite = false;
                        }
                    } else {
                        if !method.modifiers.contains(&modifier_abstract) {
                            if existing.modifiers == method.modifiers &&
                               existing.body == method.body {
                                overwrite = false;
                                break;
                            }

                            return Err(format!("could not inherit conflicting methods {}",
                                               method.name));
                        } else {
                            // existing -> concrete, method -> abstract
                            // thus we're basically overridding `method` with
                            // `existing`
                            if method.modifiers.contains(&modifier_final) &&
                               existing.body != method.body {
                                return Err(format!("cannot override final method {}", method.name));
                            }

                            if method.modifiers.contains(&public) &&
                               (existing.modifiers.contains(&protected) ||
                                existing.modifiers.contains(&private)) {
                                return Err(format!("cannot weaken perms of public method {}",
                                                   method.name));
                            } else if method.modifiers.contains(&protected) &&
                                      existing.modifiers.contains(&private) {
                                return Err(format!("cannot weaken perms of protected method {}",
                                                   method.name));
                            }

                            if method.modifiers.contains(&modifier_static) !=
                               existing.modifiers.contains(&modifier_static) {
                                return Err(format!("cannot override staticity of method {}",
                                                   method.name));
                            }

                            overwrite = false;
                        }
                    }

                    break;
                }
            }
            if overwrite {
                self.methods.push(method.clone());
            }
        }

        Ok(())
    }
}

impl fmt::Display for ClassOrInterfaceEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

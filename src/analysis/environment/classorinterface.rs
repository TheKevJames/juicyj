use std::fmt;

use analysis::environment::constructor::ConstructorEnvironment;
use analysis::environment::field::FieldEnvironment;
use analysis::environment::method::MethodEnvironment;
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
    pub constructors: Vec<ConstructorEnvironment>,
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
    /// ```rust
    /// let A: ClassOrInterfaceEnvironment;
    /// let B: ClassOrInterfaceEnvironment;
    /// let C: ClassOrInterfaceEnvironment;
    ///
    /// let mut child = ClassOrInterfaceEnvironment::new(A.name, A.kind);
    /// child.inherit(B);
    /// child.inherit(C);
    /// child.apply(A);
    /// ```
    pub fn apply(&mut self, child: &ClassOrInterfaceEnvironment) -> Result<(), String> {
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
            for (idx, inherited) in self.constructors.clone().iter().enumerate() {
                if constructor.parameters == inherited.parameters {
                    self.constructors.remove(idx);
                }
            }
            // TODO: any restrictions?
            self.constructors.push(constructor.clone());
        }
        for field in &child.fields {
            for (idx, existing) in self.fields.clone().iter().enumerate() {
                if field.name == existing.name {
                    self.fields.remove(idx);
                }
            }
            // TODO: any restrictions?
            self.fields.push(field.clone());
        }
        for method in &child.methods {
            for (idx, existing) in self.methods.clone().iter().enumerate() {
                if method.name == existing.name && method.parameters == existing.parameters {
                    // TODO: lookup?
                    if method.return_type != existing.return_type {
                        return Err(format!("cannot override method {} with different return type",
                                           method.name));
                    }

                    if existing.modifiers.contains(&modifier_final) {
                        return Err(format!("cannot override final method {}", existing.name));
                    }

                    if existing.modifiers.contains(&public) &&
                       (method.modifiers.contains(&protected) ||
                        method.modifiers.contains(&private)) {
                        return Err(format!("cannot weaken access controls of public method {}",
                                           existing.name));
                    } else if existing.modifiers.contains(&protected) &&
                              method.modifiers.contains(&private) {
                        return Err(format!("cannot weaken access controls of protected method {}",
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
    /// ```rust
    /// let A: ClassOrInterfaceEnvironment;
    /// let B: ClassOrInterfaceEnvironment;
    /// let C: ClassOrInterfaceEnvironment;
    ///
    /// let mut child = ClassOrInterfaceEnvironment::new(A.name, A.kind);
    /// child.inherit(B);
    /// child.inherit(C);
    /// child.apply(A);
    /// ```
    pub fn inherit(&mut self, parent: &ClassOrInterfaceEnvironment) -> Result<(), String> {
        let modifier_abstract = ASTNode {
            token: Token::new(TokenKind::Abstract, None),
            children: Vec::new(),
        };
        let object_name = ASTNode {
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

        for constructor in &parent.constructors {
            let mut inherited = constructor.clone();
            inherited.name = self.name.clone();

            for (idx, existing) in self.constructors.clone().iter().enumerate() {
                if &inherited == existing {
                    self.constructors.remove(idx);
                    continue;
                }

                if inherited.parameters == existing.parameters {
                    return Err("could not inherit conflicting constructors".to_owned());
                }
            }
            // TODO: any other restrictions?

            self.constructors.push(inherited);
        }
        for field in &parent.fields {
            // TODO: any restrictions?
            self.fields.push(field.clone());
        }

        if self.kind == ClassOrInterface::INTERFACE && parent.name == object_name {
            for method in &parent.methods {
                let mut inherited = method.clone();
                inherited.modifiers.push(modifier_abstract.clone());
                self.methods.push(inherited);
            }

            return Ok(());
        }

        for method in &parent.methods {
            for existing in self.methods.clone() {
                if method.name == existing.name && method.parameters == existing.parameters {
                    if method.return_type != existing.return_type {
                        return Err(format!("could not inherit methods {} with conflicting returns",
                                           method.name));
                    }

                    if existing.modifiers.contains(&modifier_abstract) {
                        if !method.modifiers.contains(&modifier_abstract) {
                            self.methods.push(method.clone());
                        }
                    } else {
                        if !method.modifiers.contains(&modifier_abstract) {
                            return Err(format!("could not inherit conflicting methods {}",
                                               method.name));
                        }
                    }

                    break;
                }
            }
            self.methods.push(method.clone());
        }

        Ok(())
    }
}

impl fmt::Display for ClassOrInterfaceEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

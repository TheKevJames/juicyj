use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::types::lookup;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref BOOLEAN: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Boolean, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref BYTE: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Byte, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref CHAR: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Char, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref INTEGER: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Int, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref NULL: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Null, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref SHORT: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Short, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref STRING_CANONICAL: Type = {
        let node = ASTNode {
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
                               token: Token::new(TokenKind::Identifier, Some("String")),
                               children: Vec::new(),
                           }],
        };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref STRING_SHORT: Type = {
        let node = ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("Name")),
            children: vec![ASTNode {
                               token: Token::new(TokenKind::Identifier, Some("String")),
                               children: Vec::new(),
                           }],
        };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref VOID: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Void, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
}

#[derive(Debug,Clone)]
pub struct Type {
    // TODO: make this non-public
    pub kind: ClassOrInterfaceEnvironment,
}

impl Type {
    pub fn new(kind: ClassOrInterfaceEnvironment) -> Type {
        Type { kind: kind }
    }

    pub fn apply_cast(&self,
                      other: &Type,
                      current: &ClassOrInterfaceEnvironment,
                      kinds: &Vec<ClassOrInterfaceEnvironment>)
                      -> Result<Type, String> {
        let lhs = match lookup::class::in_env(&self.kind.name, current, kinds) {
            Ok(cls) => Type::new(cls),
            Err(e) => return Err(e),
        };
        let rhs = match lookup::class::in_env(&other.kind.name, current, kinds) {
            Ok(cls) => Type::new(cls),
            Err(e) => return Err(e),
        };

        let result = lhs.clone();

        // TODO: figure out these rules
        // is it backwards from assignability? (J1_6_Assignable_Object_ObjectArray)
        // plus primitives? (stdlib)
        //   no, but maybe backwards + forwards?
        //   also needs Short<-Char and Byte<-Char
        let primitives = vec![BYTE.clone(), CHAR.clone(), INTEGER.clone(), SHORT.clone()];
        if primitives.contains(&lhs) && primitives.contains(&rhs) {
            return Ok(result);
        }

        let lhs_result = lhs.assign(&rhs, current, kinds);
        if lhs_result.is_ok() {
            return Ok(result);
        }

        let rhs_result = rhs.assign(&lhs, current, kinds);
        if rhs_result.is_ok() {
            return Ok(result);
        }

        Err(format!("could not cast {:?} to {:?}\ngot errors:\n\t{:?}\n\t{:?}",
                    rhs.kind.name,
                    lhs.kind.name,
                    lhs_result.unwrap_err(),
                    rhs_result.unwrap_err()))
    }

    pub fn apply_comparison(&self,
                            operation: &TokenKind,
                            other: &Type,
                            current: &ClassOrInterfaceEnvironment,
                            kinds: &Vec<ClassOrInterfaceEnvironment>)
                            -> Result<Type, String> {
        // Anything assignable is comparable. Comparability, though, is reflexive.
        let lhs_result = self.assign(other, current, kinds);
        if lhs_result.is_ok() {
            return Ok(BOOLEAN.clone());
        }

        let rhs_result = other.assign(self, current, kinds);
        if rhs_result.is_ok() {
            return Ok(BOOLEAN.clone());
        }

        Err(format!("could not apply comparison {:?} to {:?} and {:?}\ngot errors:\n\t{:?}\n\t{:?}",
                    operation,
                    self.kind.name,
                    other.kind.name,
                    lhs_result.unwrap_err(),
                    rhs_result.unwrap_err()))
    }

    pub fn apply_math(&self, operation: &TokenKind, other: &Type) -> Result<Type, String> {
        if *self == *VOID || *other == *VOID {
            return Err(format!("could not apply {:?} to voids {:?} and {:?}",
                               operation,
                               self.kind.name,
                               other.kind.name));
        }

        if *self == *CHAR && *other == *CHAR {
            return Ok(CHAR.clone());
        }

        let mut primitives = vec![BYTE.clone()];
        if primitives.contains(&self) && primitives.contains(&other) {
            return Ok(BYTE.clone());
        }

        primitives.push(SHORT.clone());
        if primitives.contains(&self) && primitives.contains(&other) {
            return Ok(SHORT.clone());
        }

        primitives.push(CHAR.clone());
        primitives.push(INTEGER.clone());
        if primitives.contains(&self) && primitives.contains(&other) {
            return Ok(INTEGER.clone());
        }

        match *operation {
            TokenKind::Plus => {
                // anything can resolve to a String
                let strings = vec![STRING_CANONICAL.clone(), STRING_SHORT.clone()];
                if strings.contains(self) || strings.contains(other) {
                    return Ok(STRING_CANONICAL.clone());
                }
            }
            _ => (),
        }

        Err(format!("could not apply math {:?} to {:?} and {:?}",
                    operation,
                    self.kind.name,
                    other.kind.name))
    }

    pub fn assign(&self,
                  rhs: &Type,
                  current: &ClassOrInterfaceEnvironment,
                  kinds: &Vec<ClassOrInterfaceEnvironment>)
                  -> Result<Type, String> {
        let mut lhs = match lookup::class::in_env(&self.kind.name, current, kinds) {
            Ok(cls) => Type::new(cls),
            Err(e) => return Err(e),
        };
        let mut rhs = match lookup::class::in_env(&rhs.kind.name, current, kinds) {
            Ok(cls) => Type::new(cls),
            Err(e) => return Err(e),
        };

        let result = lhs.clone();

        // can not assign anything to voids (except nulls as returns...)
        if lhs == *VOID && rhs != *NULL {
            return Err(format!("cannot assign {} to void", rhs.kind.name));
        }

        // can't assign classes to arrays, but can assign arrays to Object
        // can assign arrays to each other with rules equal to child kinds
        let lhs_array = lhs.kind.name.clone().token.lexeme.unwrap_or("".to_owned()) == "ArrayType";
        let rhs_array = rhs.kind.name.clone().token.lexeme.unwrap_or("".to_owned()) == "ArrayType";
        if lhs_array {
            // TODO: null works for all arrays, or just non-primitive arrays?
            if lhs == rhs || rhs == *NULL {
                return Ok(result);
            }

            if !rhs_array {
                return Err(format!("cannot assign class {} to array {}",
                                   rhs.kind.name,
                                   lhs.kind.name));
            }

            lhs = match lookup::class::in_env(&lhs.kind.name.children[0], current, kinds) {
                Ok(cls) => Type::new(cls),
                Err(e) => return Err(e),
            };
            rhs = match lookup::class::in_env(&rhs.kind.name.children[0], current, kinds) {
                Ok(cls) => Type::new(cls),
                Err(e) => return Err(e),
            };

            match lhs.is_parent(&rhs, kinds) {
                Some(Ok(_)) => return Ok(result),
                Some(Err(e)) => return Err(e),
                None => (),
            }

            return Err(format!("cannot assign non-subtype array {} to {}",
                               rhs.kind.name,
                               lhs.kind.name));
        }

        if lhs == rhs {
            return Ok(result);
        }

        let mut primitives = vec![BYTE.clone()];
        if lhs == *BYTE && primitives.contains(&rhs) {
            return Ok(result);
        }

        primitives.push(SHORT.clone());
        if lhs == *SHORT && primitives.contains(&rhs) {
            return Ok(result);
        }

        primitives.push(CHAR.clone());
        primitives.push(INTEGER.clone());
        if lhs == *INTEGER && primitives.contains(&rhs) {
            return Ok(INTEGER.clone());
        }

        // can assign null to anything non-primitive
        if !primitives.contains(&lhs) && rhs == *NULL {
            return Ok(result);
        }

        match lhs.is_parent(&rhs, kinds) {
            Some(Ok(_)) => return Ok(result),
            Some(Err(e)) => return Err(e),
            None => (),
        }

        Err(format!("can not assign {} to {}", rhs.kind.name, lhs.kind.name))
    }

    pub fn is_coercible_to_int(&self) -> bool {
        let primitives = vec![BYTE.clone(), CHAR.clone(), INTEGER.clone(), SHORT.clone()];
        primitives.contains(&self)
    }

    fn is_parent(&self,
                 child: &Type,
                 kinds: &Vec<ClassOrInterfaceEnvironment>)
                 -> Option<Result<(), String>> {
        let mut parents = vec![child.kind.clone()];
        while let Some(parent) = parents.pop() {
            if parent.name == self.kind.name {
                return Some(Ok(()));
            }

            // TODO: .chain()
            for grandparent in &parent.extends {
                match lookup::class::in_env(&grandparent, &parent, kinds) {
                    Ok(cls) => parents.push(cls),
                    Err(e) => return Some(Err(e)),
                };
            }
            for grandparent in &parent.implements {
                match lookup::class::in_env(&grandparent, &parent, kinds) {
                    Ok(cls) => parents.push(cls),
                    Err(e) => return Some(Err(e)),
                };
            }
        }

        None
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        let lhs_array = self.kind.name.clone().token.lexeme.unwrap_or("".to_owned()) == "ArrayType";
        let rhs_array = other.kind.name.clone().token.lexeme.unwrap_or("".to_owned()) ==
                        "ArrayType";

        if lhs_array != rhs_array {
            return false;
        }

        if lhs_array && rhs_array {
            let mut lhs = self.clone();
            lhs.kind.name = lhs.kind.name.children[0].clone();

            let mut rhs = other.clone();
            rhs.kind.name = rhs.kind.name.children[0].clone();

            return lhs == rhs;
        }

        if self.kind.name == other.kind.name {
            return true;
        }

        let mut lhs = self.kind.name.clone();
        if self.kind.name.children.len() >= 3 &&
           self.kind
            .name
            .children
            .first()
            .unwrap()
            .clone()
            .token
            .lexeme
            .unwrap_or("".to_owned()) == "juicyj-unnamed" {
            lhs.children.remove(0);
            lhs.children.remove(0);
        }

        let mut rhs = other.kind.name.clone();
        if other.kind.name.children.len() >= 3 &&
           other.kind
            .name
            .children
            .first()
            .unwrap()
            .clone()
            .token
            .lexeme
            .unwrap_or("".to_owned()) == "juicyj-unnamed" {
            rhs.children.remove(0);
            rhs.children.remove(0);
        }

        lhs == rhs
    }
}

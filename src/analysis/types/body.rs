use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::check;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

#[derive(Debug,Clone)]
struct Type {
    kind: ClassOrInterfaceEnvironment,
}

impl Type {
    fn new(kind: ClassOrInterfaceEnvironment) -> Type {
        Type { kind: kind }
    }

    fn apply_comparison(&self,
                        operation: &TokenKind,
                        other: &Type,
                        current: &ClassOrInterfaceEnvironment,
                        kinds: &Vec<ClassOrInterfaceEnvironment>)
                        -> Result<Type, String> {
        let boolean = ASTNode {
            token: Token::new(TokenKind::Boolean, None),
            children: Vec::new(),
        };
        let boolean = ClassOrInterfaceEnvironment::new(boolean.clone(), ClassOrInterface::CLASS);
        let boolean = Type::new(boolean);

        // Anything assignable is comparable. Comparability, though, is reflexive
        match self.assign(other, current, kinds) {
            Ok(_) => Ok(boolean),
            Err(_) => {
                match other.assign(self, current, kinds) {
                    Ok(_) => Ok(boolean),
                    Err(_) => {
                        Err(format!("could not apply {:?} to {:?} and {:?}",
                                    operation,
                                    self.kind.name,
                                    other.kind.name))
                    }
                }
            }
        }
    }

    // TODO: subset of operations (string concat only, no subtraction)
    fn apply_math(&self, operation: &TokenKind, other: &Type) -> Result<Type, String> {
        if self == other {
            return Ok(self.clone());
        }

        let charr = ASTNode {
            token: Token::new(TokenKind::Char, None),
            children: Vec::new(),
        };
        if self.kind.name == charr.clone() && other.kind.name == charr.clone() {
            return Ok(Type::new(ClassOrInterfaceEnvironment::new(charr.clone(),
                                                                 ClassOrInterface::CLASS)));
        }

        let boolean = ASTNode {
            token: Token::new(TokenKind::Boolean, None),
            children: Vec::new(),
        };
        if self.kind.name == boolean.clone() && other.kind.name == boolean.clone() {
            return Ok(Type::new(ClassOrInterfaceEnvironment::new(boolean.clone(),
                                                                 ClassOrInterface::CLASS)));
        }

        let byte = ASTNode {
            token: Token::new(TokenKind::Byte, None),
            children: Vec::new(),
        };
        let mut primitives = vec![boolean.clone(), byte.clone()];
        if primitives.contains(&self.kind.name) && primitives.contains(&other.kind.name) {
            return Ok(Type::new(ClassOrInterfaceEnvironment::new(byte.clone(),
                                                                 ClassOrInterface::CLASS)));
        }

        let short = ASTNode {
            token: Token::new(TokenKind::Short, None),
            children: Vec::new(),
        };
        primitives.push(short.clone());
        if primitives.contains(&self.kind.name) && primitives.contains(&other.kind.name) {
            return Ok(Type::new(ClassOrInterfaceEnvironment::new(short.clone(),
                                                                 ClassOrInterface::CLASS)));
        }

        let int = ASTNode {
            token: Token::new(TokenKind::Int, None),
            children: Vec::new(),
        };
        primitives.push(charr.clone());
        primitives.push(int.clone());
        if primitives.contains(&self.kind.name) && primitives.contains(&other.kind.name) {
            return Ok(Type::new(ClassOrInterfaceEnvironment::new(int.clone(),
                                                                 ClassOrInterface::CLASS)));
        }

        let string = ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("Name")),
            children: vec![ASTNode {
                               token: Token::new(TokenKind::Identifier, Some("String")),
                               children: Vec::new(),
                           }],
        };
        let java_lang_string = ASTNode {
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
        primitives.push(string.clone());
        primitives.push(java_lang_string.clone());
        if primitives.contains(&self.kind.name) && primitives.contains(&other.kind.name) {
            return Ok(Type::new(ClassOrInterfaceEnvironment::new(java_lang_string.clone(),
                                                                 ClassOrInterface::CLASS)));
        }

        Err(format!("could not apply {:?} to {:?} and {:?}",
                    operation,
                    self.kind.name,
                    other.kind.name))
    }

    fn assign(&self,
              rhs: &Type,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>)
              -> Result<Type, String> {
        let lhs = match check::lookup_or_primitive(&self.kind.name, current, kinds) {
            Ok(cls) => Type::new(cls),
            Err(e) => return Err(e),
        };
        let rhs = match check::lookup_or_primitive(&rhs.kind.name, current, kinds) {
            Ok(cls) => Type::new(cls),
            Err(e) => return Err(e),
        };

        if lhs == rhs {
            return Ok(lhs.clone());
        }

        // can assign null to anything
        let null = ASTNode {
            token: Token::new(TokenKind::Null, None),
            children: Vec::new(),
        };
        if rhs.kind.name == null {
            return Ok(lhs.clone());
        }

        let boolean = ASTNode {
            token: Token::new(TokenKind::Boolean, None),
            children: Vec::new(),
        };
        let byte = ASTNode {
            token: Token::new(TokenKind::Byte, None),
            children: Vec::new(),
        };
        let mut primitives = vec![boolean.clone(), byte.clone()];
        if lhs.kind.name == byte.clone() && primitives.contains(&rhs.kind.name) {
            return Ok(lhs.clone());
        }

        let short = ASTNode {
            token: Token::new(TokenKind::Short, None),
            children: Vec::new(),
        };
        primitives.push(short.clone());
        if lhs.kind.name == short.clone() && primitives.contains(&rhs.kind.name) {
            return Ok(lhs.clone());
        }

        let charr = ASTNode {
            token: Token::new(TokenKind::Char, None),
            children: Vec::new(),
        };
        let int = ASTNode {
            token: Token::new(TokenKind::Int, None),
            children: Vec::new(),
        };
        primitives.push(charr.clone());
        primitives.push(int.clone());
        if lhs.kind.name == int.clone() && primitives.contains(&rhs.kind.name) {
            return Ok(lhs.clone());
        }

        let mut parents = vec![rhs.kind.clone()];
        while let Some(parent) = parents.pop() {
            if parent.name == lhs.kind.name {
                return Ok(lhs.clone());
            }

            // TODO: .chain()
            for grandparent in &parent.extends {
                match check::lookup(&grandparent, &parent, kinds) {
                    Ok(cls) => parents.push(cls),
                    Err(e) => return Err(e),
                };
            }
            for grandparent in &parent.implements {
                match check::lookup(&grandparent, &parent, kinds) {
                    Ok(cls) => parents.push(cls),
                    Err(e) => return Err(e),
                };
            }
        }

        Err(format!("can not assign {} to {}", rhs.kind.name, lhs.kind.name))
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        if let Some(lhs_lex) = self.kind.name.clone().token.lexeme {
            if lhs_lex == "ArrayType" {
                let mut lhs = self.clone();
                lhs.kind.name = lhs.kind.name.children[0].clone();
                return &lhs == other;
            }
        }

        if let Some(rhs_lex) = other.kind.name.clone().token.lexeme {
            if rhs_lex == "ArrayType" {
                let mut rhs = other.clone();
                rhs.kind.name = rhs.kind.name.children[0].clone();
                return self == &rhs;
            }
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

fn resolve_expression(node: &ASTNode,
                      current: &ClassOrInterfaceEnvironment,
                      kinds: &Vec<ClassOrInterfaceEnvironment>,
                      globals: &Vec<VariableEnvironment>)
                      -> Result<Type, String> {
    match node.token.lexeme {
        Some(ref l) if l == "ArrayAccess" => {
            let array = match resolve_expression(&node.children[0], current, kinds, globals) {
                Ok(a) => a,
                Err(e) => return Err(e),
            };

            if array.kind.name.clone().token.lexeme.unwrap() != "ArrayType" {
                return Err(format!("got invalid array type {:?}", array));
            }

            let integer =
                Type::new(ClassOrInterfaceEnvironment::new(ASTNode {
                                                               token: Token::new(TokenKind::Int,
                                                                                 None),
                                                               children: Vec::new(),
                                                           },
                                                           ClassOrInterface::CLASS));

            // TODO: look up nth item
            match resolve_expression(&node.children[2], current, kinds, globals) {
                // Ok(idx) if idx == &integer => idx.clone(),
                // Ok(idx) => return Err(format!("got invalid index type {:?}", idx)),
                Ok(idx) => {
                    if idx == integer {
                        // idx
                        ()
                    } else {
                        return Err(format!("got invalid index type {:?}", idx));
                    }
                }
                Err(e) => return Err(e),
            }

            let mut kind = array.kind;
            kind.name = kind.name.children[0].clone();
            Ok(Type::new(kind))
        }
        Some(ref l) if l == "ArrayCreationExpression" => {
            match resolve_expression(&node.children[0], current, kinds, globals) {
                Ok(x) => {
                    let mut kind = x.kind;
                    kind.name = ASTNode {
                        token: Token::new(TokenKind::NonTerminal, Some("ArrayType")),
                        children: vec![kind.name],
                    };
                    Ok(Type::new(kind))
                }
                Err(e) => return Err(e),
            }
        }
        Some(ref l) if l == "Assignment" => {
            let lhs = match resolve_expression(&node.children[0], current, kinds, globals) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };

            let rhs = match resolve_expression(&node.children[2], current, kinds, globals) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            lhs.assign(&rhs, current, kinds)
        }
        Some(ref l) if l == "CastExpression" => {
            match resolve_expression(&node.children[1], current, kinds, globals) {
                Ok(x) => {
                    let mut kind = x.kind;
                    if node.children.len() == 5 {
                        kind.name = ASTNode {
                            token: Token::new(TokenKind::NonTerminal, Some("ArrayType")),
                            children: vec![kind.name],
                        };
                    }
                    Ok(Type::new(kind))
                }
                Err(e) => Err(e),
            }
        }
        Some(ref l) if l == "ClassInstanceCreationExpression" => {
            let mut kind = node.children[0].clone();
            kind.flatten();
            match check::lookup(&kind, current, kinds) {
                Ok(cls) => Ok(Type::new(cls)),
                Err(e) => Err(e),
            }
        }
        Some(ref l) if l == "FieldAccess" => {
            let lhs = match resolve_expression(&node.children[0], current, kinds, globals) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };

            let mut lhs_kind = lhs.kind.name.clone();
            lhs_kind.flatten();
            let cls = match check::lookup_or_primitive(&lhs_kind, current, kinds) {
                Ok(cls) => cls,
                Err(e) => return Err(e),
            };

            for field in &cls.fields {
                if field.name == node.children[2] {
                    match check::lookup_or_primitive(&field.to_variable().kind, current, kinds) {
                        Ok(cls) => return Ok(Type::new(cls)),
                        Err(_) => (),
                    }
                }
            }

            Err(format!("could not find field {} in class {}",
                        node.children[2],
                        cls.name))
        }
        Some(ref l) if l == "MethodInvocation" => {
            let lookup = match node.children.len() {
                3 | 4 => {
                    let mut name = node.children[0].clone();
                    let lhs = match resolve_expression(&name, current, kinds, globals) {
                        Ok(l) => {
                            let mut lhs_name = l.kind.name.clone();
                            lhs_name.flatten();
                            check::lookup_or_primitive(&lhs_name, current, kinds).ok()
                        }
                        Err(_) => None,
                    };

                    if lhs.is_some() {
                        Some((lhs.unwrap(), node.children[2].clone()))
                    } else {
                        name.flatten();

                        let method = name.children.pop().unwrap();
                        name.children.pop();

                        // TODO: if name.chlidren had 3+ anyway
                        // TODO: other.x, etc?
                        // TODO: just fucking write a globals.lookup
                        for var in globals {
                            // this.x
                            if var.name == name {
                                name = var.kind.clone();
                                break;
                            }

                            // x
                            if var.name.children.len() == 3 &&
                               var.name.children[0].clone().token.kind == TokenKind::This &&
                               var.name.children[2] == name {
                                // let result = check::lookup(&var.kind, current, kinds);
                                // if result.is_err() {
                                //     continue;
                                // }
                                // return Some((result.ok(), method.clone()))
                                name = var.kind.clone();
                                break;
                            }
                        }

                        match check::lookup(&name, current, kinds) {
                            Ok(cls) => Some((cls, method.clone())),
                            // assume we don't need to resolve
                            Err(_) => Some((current.clone(), node.children[0].clone())),
                        }
                    }
                }
                5 | 6 => {
                    let name = node.children[0].clone();
                    let lhs = match resolve_expression(&name, current, kinds, globals) {
                        Ok(r) => r,
                        Err(e) => return Err(e),
                    };

                    let mut lhs_name = lhs.kind.name.clone();
                    lhs_name.flatten();
                    match check::lookup_or_primitive(&lhs_name, current, kinds) {
                        Ok(cls) => Some((cls, node.children[2].clone())),
                        Err(e) => return Err(e),
                    }
                }
                _ => None,
            };

            if let Some((cls, method)) = lookup {
                let mut method = method;
                method.flatten();

                let mut found = None;
                for cls_method in &cls.methods {
                    if cls_method.name != method {
                        continue;
                    }

                    found = Some(cls_method);
                    break;
                }

                if found.is_none() {
                    return Err(format!("could not find method {} on class {}", method, cls.name));
                }

                return Ok(Type::new(ClassOrInterfaceEnvironment::new(found.unwrap()
                                                                         .return_type
                                                                         .clone(),
                                                                     ClassOrInterface::CLASS)));
            }

            Err(format!("malformated MethodInvocation {}", node))
        }
        // Sometimes "Name" can be a field access. TODO: fix this in grammar
        // Cases: x, this.x, other.x, x.field, this.x.field, other.x.field
        Some(ref l) if l == "Name" => {
            let mut node = node.clone();
            node.flatten();

            let mut node_fieldless = node.clone();
            let mut field = None;
            if node_fieldless.children.len() >= 3 {
                field = node_fieldless.children.pop();
                node_fieldless.children.pop();
            }

            for var in globals {
                // this.x
                if var.name == node {
                    return Ok(Type::new(ClassOrInterfaceEnvironment::new(var.kind.clone(),
                                                                         ClassOrInterface::CLASS)));
                }

                // x
                if var.name.children.len() == 3 &&
                   var.name.children[0].clone().token.kind == TokenKind::This &&
                   var.name.children[2] == node {
                    return Ok(Type::new(ClassOrInterfaceEnvironment::new(var.kind.clone(),
                                                                         ClassOrInterface::CLASS)));
                }

                // other.x
                if node.children[0].clone().token.kind != TokenKind::This &&
                   var.name == node.children[0] {
                    // at this point, var.name is other.
                    // lookup var.kind for a class, then find node.children[2]
                    let mut kind = var.kind.clone();
                    kind.flatten();
                    return match check::lookup(&kind, current, kinds) {
                        Ok(f) => {
                            let mut result = None;

                            for field in &f.fields {
                                if field.name != node.children[2] {
                                    continue;
                                }

                                result = Some(field.to_variable().kind.clone());
                            }

                            match result {
                                Some(k) => {
                                    let kind =
                                        ClassOrInterfaceEnvironment::new(k,
                                                                         ClassOrInterface::CLASS);
                                    Ok(Type::new(kind))
                                }
                                None => Err(format!("could not find(1) field {} on {}", node, f)),
                            }
                        }
                        Err(e) => Err(e),
                    };
                }

                if field.is_some() {
                    // this.x.field
                    if var.name == node_fieldless {
                        let mut kind = var.kind.clone();
                        kind.flatten();
                        return match check::lookup(&kind, current, kinds) {
                            Ok(f) => {
                                let mut result = None;

                                for cls_field in &f.fields {
                                    if cls_field.name != field.clone().unwrap() {
                                        continue;
                                    }

                                    result = Some(cls_field.to_variable().kind.clone());
                                }

                                match result {
                                    Some(k) => {
                                        let kind =
                                        ClassOrInterfaceEnvironment::new(k,
                                                                         ClassOrInterface::CLASS);
                                        Ok(Type::new(kind))
                                    }
                                    None => {
                                        Err(format!("could not find(2) field {} on {}", node, f))
                                    }
                                }
                            }
                            Err(e) => Err(e),
                        };
                    }

                    // x.field
                    if var.name.children.len() == 3 &&
                       var.name.children[0].clone().token.kind == TokenKind::This &&
                       var.name.children[2] == node_fieldless {
                        let mut kind = var.kind.clone();
                        kind.flatten();
                        return match check::lookup(&kind, current, kinds) {
                            Ok(f) => {
                                let mut result = None;

                                for cls_field in &f.fields {
                                    if cls_field.name != field.clone().unwrap() {
                                        continue;
                                    }

                                    result = Some(cls_field.to_variable().kind.clone());
                                }

                                match result {
                                    Some(k) => {
                                        let kind =
                                    ClassOrInterfaceEnvironment::new(k,
                                                                     ClassOrInterface::CLASS);
                                        Ok(Type::new(kind))
                                    }
                                    None => {
                                        Err(format!("could not find(3) field {} on {}", node, f))
                                    }
                                }
                            }
                            Err(e) => Err(e),
                        };
                    }

                    // TODO: this should nest arbitrarily...
                    // other.x.field
                }
            }

            loop {
                if field.is_none() {
                    break;
                }

                let cls = match check::lookup(&node_fieldless, current, kinds) {
                    Ok(c) => c,
                    Err(_) => break,
                };

                let field = field.unwrap();
                for f in &cls.fields {
                    if &f.name == &field {
                        match check::lookup_or_primitive(&f.to_variable().kind, current, kinds) {
                            Ok(cls) => return Ok(Type::new(cls)),
                            Err(_) => (),
                        }
                    }
                }

                break;
            }

            match check::lookup(&node, current, kinds) {
                Ok(f) => Ok(Type::new(f)),
                Err(e) => Err(e),
            }
        }
        _ => {
            match node.token.kind {
                // Expressions
                TokenKind::And | TokenKind::BitAnd | TokenKind::Or | TokenKind::BitOr |
                TokenKind::BitXor => {
                    let lhs =
                        match resolve_expression(&node.children[0], current, kinds, globals) {
                            Ok(l) => l,
                            Err(e) => return Err(e),
                        };
                    let rhs =
                        match resolve_expression(&node.children[1], current, kinds, globals) {
                            Ok(r) => r,
                            Err(e) => return Err(e),
                        };

                    let bitwise = vec![TokenKind::BitAnd, TokenKind::BitOr, TokenKind::BitXor];
                    let boolean = ASTNode {
                        token: Token::new(TokenKind::Boolean, None),
                        children: Vec::new(),
                    };

                    if lhs.kind.name == boolean && rhs.kind.name == boolean {
                        Ok(lhs)
                    } else if bitwise.contains(&node.token.kind) {
                        Err(format!("bitwise operations are not allowed"))
                    } else {
                        Err(format!("could not apply {:?} to {:?} and {:?}",
                                    node.token.kind,
                                    lhs.kind.name,
                                    rhs.kind.name))
                    }
                }
                TokenKind::Not => {
                    let arg =
                        match resolve_expression(&node.children[0], current, kinds, globals) {
                            Ok(l) => l,
                            Err(e) => return Err(e),
                        };

                    let boolean = ASTNode {
                        token: Token::new(TokenKind::Boolean, None),
                        children: Vec::new(),
                    };

                    if arg.kind.name == boolean {
                        Ok(arg)
                    } else {
                        Err(format!("could not apply {:?} to {:?}",
                                    node.token.kind,
                                    arg.kind.name))
                    }
                }
                TokenKind::Equality |
                TokenKind::NotEqual |
                TokenKind::LessThan |
                TokenKind::LessThanOrEqual |
                TokenKind::GreaterThan |
                TokenKind::GreaterThanOrEqual => {
                    let lhs =
                        match resolve_expression(&node.children[0], current, kinds, globals) {
                            Ok(l) => l,
                            Err(e) => return Err(e),
                        };
                    let rhs =
                        match resolve_expression(&node.children[1], current, kinds, globals) {
                            Ok(r) => r,
                            Err(e) => return Err(e),
                        };

                    lhs.apply_comparison(&node.token.kind, &rhs, current, kinds)
                }
                TokenKind::Instanceof => {
                    match resolve_expression(&node.children[0], current, kinds, globals) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                    match resolve_expression(&node.children[1], current, kinds, globals) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }

                    let boolean = ASTNode {
                        token: Token::new(TokenKind::Boolean, None),
                        children: Vec::new(),
                    };
                    let boolean = ClassOrInterfaceEnvironment::new(boolean,
                                                                   ClassOrInterface::CLASS);
                    let boolean = Type::new(boolean);

                    Ok(boolean)
                }
                TokenKind::FSlash | TokenKind::Minus | TokenKind::Percent | TokenKind::Plus |
                TokenKind::Star => {
                    let lhs =
                        match resolve_expression(&node.children[0], current, kinds, globals) {
                            Ok(l) => l,
                            Err(e) => return Err(e),
                        };
                    let rhs =
                        match resolve_expression(&node.children[1], current, kinds, globals) {
                            Ok(r) => r,
                            Err(e) => return Err(e),
                        };

                    lhs.apply_math(&node.token.kind, &rhs)
                }
                // Primitives
                TokenKind::Boolean | TokenKind::Byte | TokenKind::Char | TokenKind::Int |
                TokenKind::Null | TokenKind::Short => {
                    Ok(Type::new(ClassOrInterfaceEnvironment::new(node.clone(),
                                                                  ClassOrInterface::CLASS)))
                }
                // Primitive Values
                TokenKind::CharValue => {
                    let node = ASTNode {
                        token: Token::new(TokenKind::Char, None),
                        children: Vec::new(),
                    };
                    Ok(Type::new(ClassOrInterfaceEnvironment::new(node.clone(),
                                                                  ClassOrInterface::CLASS)))
                }
                TokenKind::NumValue => {
                    let node = ASTNode {
                        token: Token::new(TokenKind::Int, None),
                        children: Vec::new(),
                    };
                    Ok(Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS)))
                }
                TokenKind::StrValue => {
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
                    Ok(Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS)))
                }
                TokenKind::True | TokenKind::False => {
                    let node = ASTNode {
                        token: Token::new(TokenKind::Boolean, None),
                        children: Vec::new(),
                    };
                    Ok(Type::new(ClassOrInterfaceEnvironment::new(node.clone(),
                                                                  ClassOrInterface::CLASS)))
                }
                // Other
                TokenKind::This => Ok(Type::new(current.clone())),
                TokenKind::NonTerminal => {
                    Err(format!("unhandled resolve_expression NonTerminal {:?}",
                                node.token.lexeme))
                }
                _ => Err(format!("unhandled resolve_expression {:?}", node.token.kind)),
            }
        }
    }
}

// TODO: globals should be split into fields, parameters instead of fields to_var hack
pub fn verify(node: &mut ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>,
              globals: &Vec<VariableEnvironment>)
              -> Result<(), String> {
    let node = match node.clone().token.lexeme {
        Some(ref l) if l == "BlockStatements" => node.flatten().clone(),
        Some(ref l) if l == "Block" => {
            return verify_statement(&mut node.clone(), current, kinds, globals, &mut Vec::new())
        }
        _ => {
            ASTNode {
                token: Token::new(TokenKind::NonTerminal, Some("BlockStatements")),
                children: vec![node.clone()],
            }
        }
    };
    let mut locals = Vec::new();
    for child in &node.children {
        match verify_statement(&mut child.clone(), current, kinds, globals, &mut locals) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn verify_statement(node: &mut ASTNode,
                    current: &ClassOrInterfaceEnvironment,
                    kinds: &Vec<ClassOrInterfaceEnvironment>,
                    globals: &Vec<VariableEnvironment>,
                    locals: &mut Vec<VariableEnvironment>)
                    -> Result<(), String> {
    let modifier_abstract = ASTNode {
        token: Token::new(TokenKind::Abstract, None),
        children: Vec::new(),
    };

    match node.token.lexeme {
        // TODO: resolve_expression ....
        // TODO: UnaryExpression (Minus, Not)
        // TODO: verify CastExpression
        Some(ref l) if l == "ArrayCreationExpression" || l == "ClassInstanceCreationExpression" => {
            // TODO: ACE -> child1 may be expr, CICE -> child1 may be params
            let mut kind = node.children[0].clone();
            kind.flatten();
            match check::lookup_or_primitive(&kind, current, kinds) {
                Ok(ref k) if k.modifiers.contains(&modifier_abstract) => {
                    Err(format!("instantiated abstract class {}", k.name))
                }
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        // TODO: look into TokenKind::Assignment vs "Assignment"
        Some(ref l) if l == "Assignment" => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            match resolve_expression(&node, current, kinds, &block_globals) {
                Ok(_) => Ok(()),
                Err(e) => return Err(e),
            }
        }
        Some(ref l) if l == "Block" && node.children.len() == 3 => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            verify(&mut node.children[1], current, kinds, &block_globals)
        }
        Some(ref l) if l == "Block" => Ok(()),
        Some(ref l) if l == "ForStatement" || l == "ForStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            let mut block_locals = Vec::new();

            let mut init = node.children[2].clone();
            match verify_statement(&mut init, current, kinds, &block_globals, &mut block_locals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            let mut block = node.children.last().unwrap().clone();
            verify_statement(&mut block,
                             current,
                             kinds,
                             &block_globals,
                             &mut block_locals)
        }
        Some(ref l) if l == "IfStatement" || l == "WhileStatement" ||
                       l == "WhileStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }

            verify_statement(&mut node.children[4],
                             current,
                             kinds,
                             &block_globals,
                             &mut Vec::new())
        }
        Some(ref l) if l == "IfElseStatement" || l == "IfElseStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            match verify_statement(&mut node.children[4],
                                   current,
                                   kinds,
                                   &block_globals,
                                   &mut Vec::new()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            verify_statement(&mut node.children[6],
                             current,
                             kinds,
                             &block_globals,
                             &mut Vec::new())
        }
        Some(ref l) if l == "LocalVariableDeclaration" => {
            verify_declaration(kinds, current, globals, locals, &node)
        }
        Some(ref l) if l == "MethodInvocation" => {
            // TODO: check method calls Primary.Identifier and Name
            match node.children.len() {
                // "Name ..."
                3 | 4 => Ok(()),
                // "Primary Dot Identifier ..."
                5 | 6 => {
                    let primary = node.children[0].clone();
                    verify_statement(&mut primary.clone(),
                                     current,
                                     kinds,
                                     &globals,
                                     &mut locals.clone())
                }
                _ => Ok(()),
            }
        }
        Some(ref l) if l == "PrimaryNoNewArray" || l == "ReturnStatement" => {
            let mut expr = node.children[1].clone();
            verify_statement(&mut expr, current, kinds, &globals, &mut locals.clone())
        }
        // TODO: check accesses of protected fields, methods, and constructors are in
        // subtype or same package
        // TODO: check static/non-static accesses to fields and methods
        // TODO: resolve all non-static field and method usages
        // TODO: resolve all names except the above
        // TODO: check expressions are correctly types (no narrowing conversions)
        Some(_) => {
            // TODO
            // println!("uncaught {:?}", node);

            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }

            match resolve_expression(node, current, kinds, &block_globals) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        _ => Ok(()),
    }
}

fn verify_declaration(kinds: &Vec<ClassOrInterfaceEnvironment>,
                      current: &ClassOrInterfaceEnvironment,
                      globals: &Vec<VariableEnvironment>,
                      locals: &mut Vec<VariableEnvironment>,
                      node: &ASTNode)
                      -> Result<(), String> {
    let new = VariableEnvironment::new(node.clone());

    match node.children[1].clone().token.kind {
        TokenKind::Assignment => {
            let mut rvalue = node.children[1].clone().children[1].clone();
            match verify_statement(&mut rvalue, current, kinds, globals, locals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            let mut block_globals = globals.clone();
            for local in locals.clone() {
                block_globals.push(local);
            }

            let lhs = match check::lookup_or_primitive(&new.kind, current, kinds) {
                Ok(l) => Type::new(l),
                Err(e) => return Err(e),
            };
            let rhs = match resolve_expression(&rvalue, current, kinds, &block_globals) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            match lhs.assign(&rhs, current, kinds) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        _ => (),
    }

    for global in globals {
        if global.name == new.name {
            return Err(format!("cannot declare variable {} with conflict in outer scope",
                               new.name));
        }
    }

    for local in locals.clone() {
        if local.name == new.name {
            return Err(format!("cannot declare multiple variables with same name {} in same scope",
                               new.name));
        }
    }

    locals.push(new.clone());
    check::verify(new.kind.clone(), current, kinds)
}

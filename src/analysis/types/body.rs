// TODO: j1_forinitcast
use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::check;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

#[derive(Debug)]
struct Type {
    kind: ClassOrInterfaceEnvironment,
}

impl Type {
    fn new(kind: ClassOrInterfaceEnvironment) -> Type {
        Type { kind: kind }
    }

    fn assignable(&self, other: &Type) -> bool {
        // TODO: subset
        // TODO: j1_intcharinit ?
        self == other
    }

    // TODO: arrays?
    // TODO: subset of operations (string concat only, no subtraction)
    fn mathable(&self, other: &Type) -> bool {
        if self == other {
            return true;
        }

        let byte_node = ASTNode {
            token: Token::new(TokenKind::Byte, None),
            children: Vec::new(),
        };

        let char_node = ASTNode {
            token: Token::new(TokenKind::Char, None),
            children: Vec::new(),
        };

        let int_node = ASTNode {
            token: Token::new(TokenKind::Int, None),
            children: Vec::new(),
        };

        let short_node = ASTNode {
            token: Token::new(TokenKind::Short, None),
            children: Vec::new(),
        };

        let mathable_primitives = vec![byte_node, char_node, int_node, short_node];
        if mathable_primitives.contains(&self.kind.name) &&
        mathable_primitives.contains(&other.kind.name) {
            return true;
        }

        let string_node = ASTNode {
            token: Token::new(TokenKind::Identifier, Some("String")),
            children: Vec::new(),
        };

        let mathable_strings = vec![char_node, string_node];
        if mathable_strings.contains(&self.kind.name) && mathable_strings.contains(&other.kind.name) {
            return true;
        }

        false
    }
}

impl PartialEq for Type {
    // TODO: ArrayType { Name { .. } } (j1_namedtypearray)
    fn eq(&self, other: &Type) -> bool {
        if self.kind.name == other.kind.name {
            return true;
        }

        // unnammed package might get in the way
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

        if lhs == rhs {
            return true;
        }

        // can assign null to anything
        let null = ASTNode {
            token: Token::new(TokenKind::Null, None),
            children: Vec::new(),
        };
        if rhs == null {
            return true;
        }

        // can assign any non-primitive to Object
        let object_node = ASTNode {
            token: Token::new(TokenKind::Identifier, Some("Object")),
            children: Vec::new(),
        };

        let boolean_node = ASTNode {
            token: Token::new(TokenKind::Boolean, None),
            children: Vec::new(),
        };

        let byte_node = ASTNode {
            token: Token::new(TokenKind::Byte, None),
            children: Vec::new(),
        };

        let char_node = ASTNode {
            token: Token::new(TokenKind::Char, None),
            children: Vec::new(),
        };

        let int_node = ASTNode {
            token: Token::new(TokenKind::Int, None),
            children: Vec::new(),
        };

        let short_node = ASTNode {
            token: Token::new(TokenKind::Short, None),
            children: Vec::new(),
        };

        let primitives = vec![boolean_node, byte_node, char_node, int_node, short_node];

        // TODO: verify (j1_1_instanceof_inlazyexp)
        if lhs == object_node && !primitives.contains(&rhs) {
            return true;
        }

        false
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
            match check::lookup(&node.children[0], current, kinds) {
                Ok(cls) => Ok(Type::new(cls)),
                Err(e) => Err(e),
            }
        }
        Some(ref l) if l == "FieldAccess" => {
            // TODO: node can be MethodInvocation (j1_1_ambiguousname_accessresultfrommethod)
            // TODO: node can be ClassInstanceCreationExpression (j1_classinstance)
            let mut node = node.clone();
            node.token.lexeme = Some("Name".to_owned());

            for var in globals {
                // this.thing
                if var.name == node {
                    return Ok(Type::new(ClassOrInterfaceEnvironment::new(var.kind.clone(),
                                                                         ClassOrInterface::CLASS)));
                }

                // TODO: other.thing
            }

            Err(format!("incomplete FieldAccess for {}", node))
        }
        Some(ref l) if l == "MethodInvocation" => {
            let lookup = match node.children.len() {
                3 | 4 => {
                    let mut name = node.children[0].clone().flatten().clone();
                    let method = name.children.pop().unwrap();
                    name.children.pop();

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
                5 | 6 => {
                    match check::lookup(&node.children[0], current, kinds) {
                        Ok(cls) => Some((cls, node.children[2].clone())),
                        Err(e) => return Err(e),
                    }
                }
                _ => None,
            };

            if let Some((cls, method)) = lookup {
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
                    return match check::lookup(&var.kind, current, kinds) {
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
                                None => Err(format!("could not find field {} on {}", node, f)),
                            }
                        }
                        Err(e) => Err(e),
                    };
                }

                if field.is_some() {
                    // this.x.field
                    if var.name == node_fieldless {
                        return match check::lookup(&var.kind, current, kinds) {
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
                                    None => Err(format!("could not find field {} on {}", node, f)),
                                }
                            }
                            Err(e) => Err(e),
                        };
                    }

                    // x.field
                    if var.name.children.len() == 3 &&
                       var.name.children[0].clone().token.kind == TokenKind::This &&
                       var.name.children[2] == node_fieldless {
                        return match check::lookup(&var.kind, current, kinds) {
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
                                    None => Err(format!("could not find field {} on {}", node, f)),
                                }
                            }
                            Err(e) => Err(e),
                        };
                    }

                    // TODO: this should nest arbitrarily...
                    // other.x.field
                }
            }

            match check::lookup(&node, current, kinds) {
                Ok(f) => Ok(Type::new(f)),
                Err(e) => Err(e),
            }
        }
        _ => {
            match node.token.kind {
                // Expressions
                TokenKind::And | TokenKind::Or => {
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

                    let boolean = ASTNode {
                        token: Token::new(TokenKind::Boolean, None),
                        children: Vec::new(),
                    };

                    if lhs == rhs && lhs.kind.name == boolean {
                        Ok(lhs)
                    } else {
                        Err(format!("could not apply {:?} to {:?} and {:?}",
                                    node.token.kind,
                                    lhs,
                                    rhs))
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
                        Err(format!("could not apply {:?} to {:?}", node.token.kind, arg))
                    }
                }
                TokenKind::BitAnd | TokenKind::BitOr | TokenKind::BitXor => {
                    // TODO: wtf is a bitwise operation? (j1_eagerbooleanoperations)
                    // TODO: non-booleans on each side?
                    // these are allowed in grammar but not in type analysis
                    Err(format!("bitwise operations are not allowed"))
                }
                TokenKind::Equality |
                TokenKind::NotEqual |
                TokenKind::LessThan |
                TokenKind::LessThanOrEqual |
                TokenKind::GreaterThan |
                TokenKind::GreaterThanOrEqual |
                TokenKind::Instanceof => {
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

                    let boolean_node = ASTNode {
                        token: Token::new(TokenKind::Boolean, None),
                        children: Vec::new(),
                    };
                    let boolean =
                        Type::new(ClassOrInterfaceEnvironment::new(boolean_node,
                                                                   ClassOrInterface::CLASS));

                    // TODO: if comparable
                    if lhs == rhs {
                        Ok(boolean)
                    } else {
                        Err(format!("could not apply {:?} to {:?} and {:?}",
                                    node.token.kind,
                                    lhs,
                                    rhs))
                    }
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

                    if lhs.mathable(&rhs) {
                        Ok(lhs)
                        // return precedence: '' + "" => "", "" + '' => ""
                    } else {
                        Err(format!("could not apply {:?} to {:?} and {:?}",
                                    node.token.kind,
                                    lhs,
                                    rhs))
                    }
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
                        token: Token::new(TokenKind::Identifier, Some("String")),
                        children: Vec::new(),
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
        // TODO: verfify CastExpression is castable
        Some(ref l) if l == "ArrayCreationExpression" || l == "ClassInstanceCreationExpression" => {
            // TODO: ACE -> child1 may be expr, CICE -> child1 may be params
            match check::lookup_or_primitive(&node.children[0], current, kinds) {
                Ok(ref k) if k.modifiers.contains(&modifier_abstract) => {
                    Err(format!("instantiated abstract class {}", k.name))
                }
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        // TODO: uh... isn't this a TokenKind? Whytf does this work?
        Some(ref l) if l == "Assignment" => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }

            let lhs = match resolve_expression(&node.children[0], current, kinds, &block_globals) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };

            let rhs = match resolve_expression(&node.children[2], current, kinds, &block_globals) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            // TODO: canonical (j1_commentsinexp5)
            if lhs.assignable(&rhs) {
                Ok(())
            } else {
                Err(format!("can not assign {} to {}", rhs.kind.name, lhs.kind.name))
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

            let lhs = Type::new(ClassOrInterfaceEnvironment::new(new.kind.clone(),
                                                                 ClassOrInterface::CLASS));
            let rhs = match resolve_expression(&rvalue, current, kinds, &block_globals) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            // TODO: canonical (j1_commentsinexp5)
            if !lhs.assignable(&rhs) {
                return Err(format!("can not assign {} to {}", rhs.kind.name, lhs.kind.name));
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

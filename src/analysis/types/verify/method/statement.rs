use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use analysis::types::resolve;
use analysis::types::verify;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref ABSTRACT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Abstract, None), children: Vec::new() }
    };
    static ref BOOLEAN: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Boolean, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref NULL: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Null, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref VOID: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Void, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
}

// TODO: cleanup by splitting into entrance point and recursion fn
pub fn block(node: &mut ASTNode,
             modifiers: &Vec<ASTNode>,
             current: &ClassOrInterfaceEnvironment,
             kinds: &Vec<ClassOrInterfaceEnvironment>,
             globals: &Vec<VariableEnvironment>)
             -> Result<Vec<Type>, String> {
    let node = match node.clone().token.lexeme {
        Some(ref l) if l == "BlockStatements" => node.flatten().clone(),
        Some(ref l) if l == "Block" => {
            return match nonblock(&mut node.clone(),
                                  modifiers,
                                  current,
                                  kinds,
                                  globals,
                                  &mut Vec::new()) {
                Ok(rts) => {
                    let mut return_types: Vec<Type> = Vec::new();
                    for (rt, is_ret) in rts {
                        if !return_types.is_empty() {
                            // TODO: too naive
                            return Err(format!("unreachable code in block::Block"));
                        } else if !is_ret || rt == *VOID {
                            continue;
                        }
                        return_types.push(rt);
                    }
                    Ok(return_types)
                }
                Err(e) => Err(e),
            };
        }
        _ => {
            ASTNode {
                token: Token::new(TokenKind::NonTerminal, Some("BlockStatements")),
                children: vec![node.clone()],
            }
        }
    };

    let mut locals = Vec::new();
    let mut return_types = Vec::new();
    for child in &node.children {
        match nonblock(&mut child.clone(),
                       modifiers,
                       current,
                       kinds,
                       globals,
                       &mut locals) {
            Ok(rts) => {
                for (rt, is_ret) in rts {
                    if !return_types.is_empty() {
                        // TODO: too naive
                        return Err(format!("unreachable code in block"));
                    } else if !is_ret || rt == *VOID {
                        continue;
                    }
                    return_types.push(rt);
                }
            }
            Err(e) => return Err(e),
        }
    }

    if return_types.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(return_types.clone())
    }
}

// TODO hack: return type sucks. Currently, its a vector of Types and whether
// they're actually returned.
pub fn nonblock(node: &mut ASTNode,
                modifiers: &Vec<ASTNode>,
                current: &ClassOrInterfaceEnvironment,
                kinds: &Vec<ClassOrInterfaceEnvironment>,
                globals: &Vec<VariableEnvironment>,
                locals: &mut Vec<VariableEnvironment>)
                -> Result<Vec<(Type, bool)>, String> {
    match node.token.lexeme {
        // TODO: check accesses of protected fields, methods, and constructors are in
        // subtype or same package
        // TODO: check static/non-static accesses to fields and methods
        // TODO: resolve all non-static field and method usages
        // TODO: resolve all names except the above
        // TODO: check expressions are correctly types (no narrowing conversions)
        // TODO: ensure all expressions are resolved
        // TODO: handle UnaryExpression (Minus, Not)
        Some(ref l) if l == "ArrayCreationExpression" => {
            // TODO: does this even?
            // TODO: ACE -> child1 may be expr
            let mut kind = node.children[0].clone();
            kind.flatten();

            match lookup::class::in_env(&kind, current, kinds) {
                Ok(ref k) if k.modifiers.contains(&*ABSTRACT) => {
                    Err(format!("instantiated abstract class {}", k.name))
                }
                Ok(ref k) if k.kind == ClassOrInterface::INTERFACE => {
                    Err(format!("instantiated interface {}", k.name))
                }
                Ok(k) => Ok(vec![(Type::new(k.clone()), false)]),
                Err(e) => Err(e),
            }
        }
        Some(ref l) if l == "ClassInstanceCreationExpression" => {
            // TODO: calling resolve::expression::go here is mostly a hack, since it
            // does the type lookup accidentally
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match resolve::expression::go(&node, modifiers, current, kinds, &mut block_globals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            let mut kind = node.children[0].clone();
            kind.flatten();
            match lookup::class::in_env(&kind, current, kinds) {
                Ok(ref k) if k.modifiers.contains(&*ABSTRACT) => {
                    Err(format!("instantiated abstract class {}", k.name))
                }
                Ok(ref k) if k.kind == ClassOrInterface::INTERFACE => {
                    Err(format!("instantiated interface {}", k.name))
                }
                Ok(k) => Ok(vec![(Type::new(k.clone()), false)]),
                Err(e) => Err(e),
            }
        }
        // TODO: look into TokenKind::Assignment vs "Assignment"
        Some(ref l) if l == "Assignment" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match resolve::expression::go(&node, modifiers, current, kinds, &mut block_globals) {
                Ok(t) => Ok(vec![(t.clone(), false)]),
                Err(e) => return Err(e),
            }
        }
        Some(ref l) if l == "Block" && node.children.len() == 3 => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match block(&mut node.children[1],
                        modifiers,
                        current,
                        kinds,
                        &mut block_globals) {
                Ok(rts) => {
                    let mut return_types = Vec::new();
                    for rt in rts {
                        return_types.push((rt, true));
                    }
                    Ok(return_types)
                }
                Err(e) => Err(e),
            }
        }
        Some(ref l) if l == "Block" => Ok(vec![(VOID.clone(), false)]),
        Some(ref l) if l == "ForStatement" || l == "ForStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());

            let mut block_locals = Vec::new();

            let mut init = node.children[2].clone();
            match nonblock(&mut init,
                           modifiers,
                           current,
                           kinds,
                           &block_globals,
                           &mut block_locals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            let mut cond_idx = 3;
            if node.children[2].token.kind != TokenKind::Semicolon {
                cond_idx += 1;
            }
            let mut cond = node.children[cond_idx].clone();
            match nonblock(&mut cond,
                           modifiers,
                           current,
                           kinds,
                           &block_globals,
                           &mut block_locals) {
                Ok(ts) => {
                    if ts.len() != 1 {
                        return Err(format!("for condition has multiple types"));
                    }

                    if ts[0].0.kind.name.token.kind != TokenKind::Boolean {
                        return Err(format!("for condition is not a boolean"));
                    }

                    if let Some(value) = ts[0].0.kind.name.token.lexeme.clone() {
                        if value == "false".to_owned() {
                            return Err(format!("for statement condition is false"));
                        }
                    }
                }
                Err(e) => return Err(e),
            }

            // Update statement is always 2 children before last child. If there
            // is not update statement, this will be a semicolon.
            let mut update = node.children[node.children.len() - 3].clone();
            if update.token.kind != TokenKind::Semicolon {
                match nonblock(&mut update,
                               modifiers,
                               current,
                               kinds,
                               &block_globals,
                               &mut block_locals) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }

            let mut block = node.children.last().unwrap().clone();
            match nonblock(&mut block,
                           modifiers,
                           current,
                           kinds,
                           &block_globals,
                           &mut block_locals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            Ok(vec![(VOID.clone(), false)])
        }
        Some(ref l) if l == "IfStatement" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match resolve::expression::go(&node.children[2],
                                          modifiers,
                                          current,
                                          kinds,
                                          &mut block_globals) {
                Ok(ref t) if t == &*BOOLEAN => (),
                Ok(_) => return Err(format!("condition {} is not boolean", node.children[2])),
                Err(e) => return Err(e),
            }

            match nonblock(&mut node.children[4],
                           modifiers,
                           current,
                           kinds,
                           &block_globals,
                           &mut Vec::new()) {
                Ok(rts) => {
                    let mut return_types: Vec<Type> = Vec::new();
                    for (rt, is_ret) in rts {
                        if !return_types.is_empty() {
                            // TODO: too naive
                            return Err(format!("unreachable code in IfStatement"));
                        } else if !is_ret || rt == *VOID {
                            continue;
                        }
                        return_types.push(rt);
                    }
                }
                Err(e) => return Err(e),
            }

            Ok(vec![(VOID.clone(), false)])
        }
        Some(ref l) if l == "WhileStatement" || l == "WhileStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match resolve::expression::go(&node.children[2],
                                          modifiers,
                                          current,
                                          kinds,
                                          &mut block_globals) {
                Ok(ref t) if t == &*BOOLEAN => {
                    if let Some(value) = t.kind.name.token.lexeme.clone() {
                        if value == "false".to_owned() {
                            return Err(format!("while statement condition is false"));
                        }
                    }
                }
                Ok(_) => return Err(format!("condition {} is not boolean", node.children[2])),
                Err(e) => return Err(e),
            }

            match nonblock(&mut node.children[4],
                           modifiers,
                           current,
                           kinds,
                           &block_globals,
                           &mut Vec::new()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            Ok(vec![(VOID.clone(), false)])
        }
        Some(ref l) if l == "IfElseStatement" || l == "IfElseStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match resolve::expression::go(&node.children[2],
                                          modifiers,
                                          current,
                                          kinds,
                                          &mut block_globals) {
                Ok(ref t) if t == &*BOOLEAN => (),
                Ok(_) => return Err(format!("condition {} is not boolean", node.children[2])),
                Err(e) => return Err(e),
            }

            let mut if_return_types: Vec<Type> = Vec::new();
            match nonblock(&mut node.children[4],
                           modifiers,
                           current,
                           kinds,
                           &block_globals,
                           &mut Vec::new()) {
                Ok(rts) => {
                    for (rt, is_ret) in rts {
                        if !if_return_types.is_empty() {
                            // TODO: too naive
                            return Err(format!("unreachable code in IfElseStatement::if"));
                        } else if !is_ret || rt == *VOID {
                            continue;
                        }
                        if_return_types.push(rt);
                    }
                }
                Err(e) => return Err(e),
            }

            let mut else_return_types: Vec<Type> = Vec::new();
            match nonblock(&mut node.children[6],
                           modifiers,
                           current,
                           kinds,
                           &block_globals,
                           &mut Vec::new()) {
                Ok(rts) => {
                    for (rt, is_ret) in rts {
                        if !else_return_types.is_empty() {
                            // TODO: too naive
                            return Err(format!("unreachable code in IfElseStatement::else"));
                        } else if !is_ret || rt == *VOID {
                            continue;
                        }
                        else_return_types.push(rt);
                    }
                }
                Err(e) => return Err(e),
            }

            if if_return_types.is_empty() && else_return_types.is_empty() {
                Ok(vec![(VOID.clone(), false)])
            } else if if_return_types.is_empty() != else_return_types.is_empty() {
                // TODO: if neq issues?
                Ok(vec![(VOID.clone(), false)])
            } else {
                let mut return_types: Vec<(Type, bool)> = Vec::new();
                for rt in if_return_types.iter().chain(else_return_types.iter()) {
                    let new = (rt.clone(), true);
                    if return_types.contains(&new) {
                        continue;
                    } else if !return_types.is_empty() {
                        return Err(format!("TODO: IfElseStatement with multiple return types"));
                    }

                    return_types.push(new);
                }
                Ok(return_types)
            }
        }
        Some(ref l) if l == "LocalVariableDeclaration" => {
            match verify::method::declaration::go(&node,
                                                  modifiers,
                                                  kinds,
                                                  current,
                                                  globals,
                                                  locals) {
                // TODO: what type is this?
                Ok(_) => Ok(vec![(VOID.clone(), false)]),
                Err(e) => Err(e),
            }
        }
        Some(ref l) if l == "MethodInvocation" => {
            // TODO: calling resolve::expression::go here is mostly a hack, since it
            // does the type lookup accidentally
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            let method = match resolve::expression::go(&node,
                                                       modifiers,
                                                       current,
                                                       kinds,
                                                       &mut block_globals) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

            if node.children.len() >= 5 {
                // Primary Dot Identifier ( Args )
                let primary = node.children[0].clone();
                match nonblock(&mut primary.clone(),
                               modifiers,
                               current,
                               kinds,
                               &globals,
                               &mut locals.clone()) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }

            // TODO: verify args are same as params of method

            Ok(vec![(method.clone(), false)])
        }
        Some(ref l) if l == "PrimaryNoNewArray" => {
            let mut expr = node.children[1].clone();
            nonblock(&mut expr,
                     modifiers,
                     current,
                     kinds,
                     &globals,
                     &mut locals.clone())
        }
        Some(ref l) if l == "ReturnStatement" => {
            if node.children.is_empty() {
                return Ok(vec![(NULL.clone(), true)]);
            }

            let mut expr = node.children[1].clone();
            match nonblock(&mut expr,
                           modifiers,
                           current,
                           kinds,
                           &globals,
                           &mut locals.clone()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match resolve::expression::go(&expr, modifiers, current, kinds, &mut block_globals) {
                Ok(rt) => Ok(vec![(rt, true)]),
                Err(e) => Err(e),
            }
        }
        _ => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match resolve::expression::go(node, modifiers, current, kinds, &mut block_globals) {
                Ok(t) => Ok(vec![(t.clone(), false)]),
                Err(e) => Err(e),
            }
        }
    }
}

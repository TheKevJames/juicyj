use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::resolve;
use analysis::types::verify;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref ABSTRACT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Abstract, None), children: Vec::new() }
    };
}

// TODO: globals should be split into fields, parameters instead of fields to_var hack
pub fn block(node: &mut ASTNode,
             current: &ClassOrInterfaceEnvironment,
             kinds: &Vec<ClassOrInterfaceEnvironment>,
             globals: &Vec<VariableEnvironment>)
             -> Result<(), String> {
    let node = match node.clone().token.lexeme {
        Some(ref l) if l == "BlockStatements" => node.flatten().clone(),
        Some(ref l) if l == "Block" => {
            return nonblock(&mut node.clone(), current, kinds, globals, &mut Vec::new())
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
        match nonblock(&mut child.clone(), current, kinds, globals, &mut locals) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

pub fn nonblock(node: &mut ASTNode,
                current: &ClassOrInterfaceEnvironment,
                kinds: &Vec<ClassOrInterfaceEnvironment>,
                globals: &Vec<VariableEnvironment>,
                locals: &mut Vec<VariableEnvironment>)
                -> Result<(), String> {
    match node.token.lexeme {
        // TODO: check accesses of protected fields, methods, and constructors are in
        // subtype or same package
        // TODO: check static/non-static accesses to fields and methods
        // TODO: resolve all non-static field and method usages
        // TODO: resolve all names except the above
        // TODO: check expressions are correctly types (no narrowing conversions)
        // TODO: ensure all expressions are resolved
        // TODO: handle UnaryExpression (Minus, Not)
        // TODO: verify CastExpression
        Some(ref l) if l == "ArrayCreationExpression" || l == "ClassInstanceCreationExpression" => {
            // TODO: ACE -> child1 may be expr, CICE -> child1 may be params
            let mut kind = node.children[0].clone();
            kind.flatten();
            match lookup::class::in_env(&kind, current, kinds) {
                Ok(ref k) if k.modifiers.contains(&*ABSTRACT) => {
                    Err(format!("instantiated abstract class {}", k.name))
                }
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        // TODO: look into TokenKind::Assignment vs "Assignment"
        Some(ref l) if l == "Assignment" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            match resolve::expression::go(&node, current, kinds, &block_globals) {
                Ok(_) => Ok(()),
                Err(e) => return Err(e),
            }
        }
        Some(ref l) if l == "Block" && node.children.len() == 3 => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());
            block(&mut node.children[1], current, kinds, &block_globals)
        }
        Some(ref l) if l == "Block" => Ok(()),
        Some(ref l) if l == "ForStatement" || l == "ForStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());

            let mut block_locals = Vec::new();

            let mut init = node.children[2].clone();
            match nonblock(&mut init, current, kinds, &block_globals, &mut block_locals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            let mut block = node.children.last().unwrap().clone();
            nonblock(&mut block,
                     current,
                     kinds,
                     &block_globals,
                     &mut block_locals)
        }
        Some(ref l) if l == "IfStatement" || l == "WhileStatement" ||
                       l == "WhileStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());

            nonblock(&mut node.children[4],
                     current,
                     kinds,
                     &block_globals,
                     &mut Vec::new())
        }
        Some(ref l) if l == "IfElseStatement" || l == "IfElseStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());

            match nonblock(&mut node.children[4],
                           current,
                           kinds,
                           &block_globals,
                           &mut Vec::new()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            nonblock(&mut node.children[6],
                     current,
                     kinds,
                     &block_globals,
                     &mut Vec::new())
        }
        Some(ref l) if l == "LocalVariableDeclaration" => {
            verify::method::declaration::go(&node, kinds, current, globals, locals)
        }
        Some(ref l) if l == "MethodInvocation" => {
            // TODO: calling resolve::expression::go here is mostly a hack, since it
            // does the type lookup accidentally
            let mut block_globals = globals.clone();
            block_globals.extend(locals.clone());

            match resolve::expression::go(&node, current, kinds, &block_globals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            if node.children.len() >= 5 {
                // Primary Dot Identifier ( Args )
                let primary = node.children[0].clone();
                match nonblock(&mut primary.clone(),
                               current,
                               kinds,
                               &globals,
                               &mut locals.clone()) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }

            // TODO: verify args are same as params of method

            Ok(())
        }
        Some(ref l) if l == "PrimaryNoNewArray" || l == "ReturnStatement" => {
            // TODO: ReturnStatement should verify child returns return_type
            let mut expr = node.children[1].clone();
            nonblock(&mut expr, current, kinds, &globals, &mut locals.clone())
        }
        // TODO: should be much more fine-grained
        // _ => Err(format!("could not verify statement {:?}", node)),
        _ => {
            match node.token.kind {
                TokenKind::Boolean | TokenKind::Byte | TokenKind::Char | TokenKind::Int |
                TokenKind::Short => Ok(()),
                TokenKind::Null | TokenKind::This => Ok(()),
                TokenKind::CharValue | TokenKind::NumValue | TokenKind::StrValue => Ok(()),
                TokenKind::True | TokenKind::False => Ok(()),
                _ => {
                    println!("defaulting on : {:?}", node);
                    let mut block_globals = globals.clone();
                    block_globals.extend(locals.clone());

                    match resolve::expression::go(node, current, kinds, &block_globals) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }
}

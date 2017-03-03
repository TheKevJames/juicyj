use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::check;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

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

pub fn verify_statement(node: &mut ASTNode,
                        current: &ClassOrInterfaceEnvironment,
                        kinds: &Vec<ClassOrInterfaceEnvironment>,
                        globals: &Vec<VariableEnvironment>,
                        locals: &mut Vec<VariableEnvironment>)
                        -> Result<(), String> {
    match node.token.lexeme {
        Some(ref l) if l == "ArrayCreationExpression" || l == "ClassInstanceCreationExpression" => {
            let kind = node.children[1].clone();
            check::verify(kind, current, kinds)
        }
        Some(ref l) if l == "Block" && node.children.len() == 3 => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            verify(&mut node.children[1], current, kinds, &block_globals)
        }
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
                    verify_statement(&mut primary.clone(), current, kinds, &globals, &mut locals.clone())
                }
                _ => Ok(()),
            }
        }
        Some(ref l) if l == "PrimaryNoNewArray" || l == "ReturnStatement" => {
            let mut expr = node.children[1].clone();
            verify_statement(&mut expr, current, kinds, &globals, &mut locals.clone())
        }
        // TODO: CastExpression
        _ => Ok(()),
    }
}

pub fn verify_declaration(kinds: &Vec<ClassOrInterfaceEnvironment>,
                          current: &ClassOrInterfaceEnvironment,
                          globals: &Vec<VariableEnvironment>,
                          locals: &mut Vec<VariableEnvironment>,
                          node: &ASTNode)
                          -> Result<(), String> {
    let new = VariableEnvironment {
        kind: node.children[0].clone(),
        name: match node.children[1].clone().token.kind {
            TokenKind::Assignment => node.children[1].clone().children[0].clone(),
            _ => node.children[1].clone(),
        },
        dim: false, // TODO: ArrayType?
    };

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

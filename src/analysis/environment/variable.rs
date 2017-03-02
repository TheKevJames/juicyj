use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use analysis::types::lookup;
use scanner::ASTNode;
use scanner::ASTNodeImport;
use scanner::TokenKind;

#[derive(Clone,Debug)]
pub struct VariableEnvironment {
    pub kind: ASTNode,
    pub name: ASTNode,
}

pub fn analyze_block(kinds: &Vec<ClassOrInterfaceEnvironment>,
                     imports: &Vec<ASTNodeImport>,
                     current: &mut ClassOrInterfaceEnvironment,
                     globals: &Vec<VariableEnvironment>,
                     node: &mut ASTNode)
                     -> Result<(), String> {
    let mut locals = Vec::new();

    let node = match node.clone().token.lexeme {
        Some(ref l) if l == "BlockStatements" => node.flatten(),
        _ => node,
    };
    for child in &node.children {
        match analyze_statement(kinds,
                                imports,
                                current,
                                globals,
                                &mut locals,
                                &mut child.clone()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

pub fn analyze_statement(kinds: &Vec<ClassOrInterfaceEnvironment>,
                         imports: &Vec<ASTNodeImport>,
                         current: &mut ClassOrInterfaceEnvironment,
                         globals: &Vec<VariableEnvironment>,
                         locals: &mut Vec<VariableEnvironment>,
                         node: &mut ASTNode)
                         -> Result<(), String> {
    match node.token.lexeme {
        Some(ref l) if l == "LocalVariableDeclaration" => {
            analyze_variable_declaration(kinds, imports, current, globals, locals, &node)
        }
        Some(ref l) if l == "Block" && node.children.len() == 3 => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            analyze_block(kinds,
                          imports,
                          current,
                          &block_globals,
                          &mut node.children[1])
        }
        Some(ref l) if l == "IfStatement" || l == "WhileStatement" ||
                       l == "WhileStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            analyze_statement(kinds,
                              imports,
                              current,
                              &block_globals,
                              &mut Vec::new(),
                              &mut node.children[4])
        }
        Some(ref l) if l == "IfElseStatement" || l == "IfElseStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            match analyze_statement(kinds,
                                    imports,
                                    current,
                                    &block_globals,
                                    &mut Vec::new(),
                                    &mut node.children[4]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            analyze_statement(kinds,
                              imports,
                              current,
                              &block_globals,
                              &mut Vec::new(),
                              &mut node.children[6])
        }
        Some(ref l) if l == "ForStatement" || l == "ForStatementNoShortIf" => {
            let mut block_globals = globals.clone();
            for local in locals {
                block_globals.push(local.clone());
            }
            let mut child = node.children.last().unwrap().clone();
            analyze_statement(kinds,
                              imports,
                              current,
                              &block_globals,
                              &mut Vec::new(),
                              &mut child)
        }
        _ => Ok(()),
    }
}

pub fn analyze_variable_declaration(kinds: &Vec<ClassOrInterfaceEnvironment>,
                                    imports: &Vec<ASTNodeImport>,
                                    current: &mut ClassOrInterfaceEnvironment,
                                    globals: &Vec<VariableEnvironment>,
                                    locals: &mut Vec<VariableEnvironment>,
                                    node: &ASTNode)
                                    -> Result<(), String> {
    let mut kind = node.children[0].clone();
    if let Some(l) = kind.clone().token.lexeme {
        if l == "ArrayType" {
            kind = kind.children[0].clone();
        }
    }
    if !vec![TokenKind::Boolean,
             TokenKind::Byte,
             TokenKind::Char,
             TokenKind::Int,
             TokenKind::Short,
             TokenKind::Void]
        .contains(&kind.token.kind) {
        println!("Variable has kind: {}", kind);
        match lookup(&kind, current, kinds, imports) {
            Ok(val) => {
                println!("Success! Found {:?}", val.name);
            }
            Err(e) => {
                println!("Error");
                println!("{}", e);
            }
        }
    }
    let name = match node.children[1].clone().token.kind {
        TokenKind::Assignment => node.children[1].clone().children[0].clone(),
        _ => node.children[1].clone(),
    };

    for global in globals {
        if global.name == name {
            return Err("cannot declare variables with same name as variable in outer scope"
                .to_owned());
        }
    }

    for local in locals.clone() {
        if local.name == name {
            return Err("cannot declare multiple variables with same name in same scope".to_owned());
        }
    }

    locals.push(VariableEnvironment {
        kind: kind,
        name: name,
    });

    Ok(())
}

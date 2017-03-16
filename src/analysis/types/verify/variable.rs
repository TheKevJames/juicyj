use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use scanner::ASTNode;
use scanner::TokenKind;

pub fn initialized(node: &ASTNode,
                   current: &ClassOrInterfaceEnvironment,
                   globals: &Vec<VariableEnvironment>)
                   -> Result<(), String> {
    if node.token.kind == TokenKind::Identifier ||
       (node.token.kind == TokenKind::NonTerminal &&
        node.clone().token.lexeme.unwrap_or("".to_owned()) == "Name") {
        for var in globals {
            if &var.name != node {
                continue;
            }

            return match var.initialized {
                false => {
                    Err(format!("using un-initialized variable {} in class {}",
                                var.name,
                                current.name))
                }
                true => Ok(()),
            };
        }

        return Ok(());
    }

    for child in &node.children {
        match initialized(&child, current, globals) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

#[derive(Clone,Debug)]
pub struct ConstructorEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub name: ASTNode,
    pub parameters: Vec<ASTNode>,
    pub body: ASTNode,
}

impl ConstructorEnvironment {
    pub fn new(name: ASTNode, body: ASTNode) -> ConstructorEnvironment {
        ConstructorEnvironment {
            modifiers: Vec::new(),
            name: name,
            parameters: Vec::new(),
            body: body,
        }
    }
}

pub fn analyze_constructor_declaration(current: &mut ClassOrInterfaceEnvironment,
                                       modifiers: &ASTNode,
                                       declarator: &ASTNode,
                                       body: &ASTNode)
                                       -> Result<(), String> {
    let mut new = ConstructorEnvironment::new(declarator.children[0].clone(), body.clone());

    for child in modifiers.clone().children {
        new.modifiers.push(child);
    }

    if declarator.children.len() == 4 {
        let mut params = declarator.children[2].clone();
        let params = match params.clone().token.lexeme {
            Some(ref l) if l == "ParameterList" => params.flatten().clone(),
            _ => {
                ASTNode {
                    token: Token::new(TokenKind::NonTerminal, Some("ParameterList")),
                    children: vec![params],
                }
            }
        };
        for param in &params.children {
            if param.token.kind == TokenKind::Comma {
                continue;
            }
            new.parameters.push(param.clone());
        }
    }

    for constructor in &current.constructors {
        if constructor.parameters == new.parameters {
            return Err("constructors must have unique signatures".to_owned());
        }
    }

    current.constructors.push(new);
    Ok(())
}

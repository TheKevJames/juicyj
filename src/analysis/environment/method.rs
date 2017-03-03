use std::fmt;

use analysis::environment::classorinterface::ClassOrInterface;
use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use analysis::environment::variable::VariableEnvironment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

#[derive(Clone,Debug)]
pub struct MethodEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub return_type: ASTNode,
    pub name: ASTNode,
    pub parameters: Vec<VariableEnvironment>,
    pub body: Option<ASTNode>,
}

impl MethodEnvironment {
    pub fn new(name: ASTNode, return_type: ASTNode) -> MethodEnvironment {
        MethodEnvironment {
            modifiers: Vec::new(),
            return_type: return_type,
            name: name,
            parameters: Vec::new(),
            body: None,
        }
    }
}

impl fmt::Display for MethodEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.name));
        if !&self.parameters.is_empty() {
            try!(write!(f, " ("));
        }
        for parameter in &self.parameters {
            try!(write!(f, " {}", parameter));
        }
        if !&self.parameters.is_empty() {
            try!(write!(f, " )"));
        }
        Ok(())
    }
}

pub fn analyze_abstract_method_declaration(current: &mut ClassOrInterfaceEnvironment,
                                           header: &ASTNode)
                                           -> Result<(), String> {
    let declarator = header.children[2].clone();

    let mut new = MethodEnvironment::new(declarator.children[0].clone(),
                                         header.children[1].clone());

    for child in header.children[0].clone().children {
        new.modifiers.push(child);
    }

    let modifier_abstract = ASTNode {
        token: Token::new(TokenKind::Abstract, None),
        children: Vec::new(),
    };
    if !new.modifiers.contains(&modifier_abstract) && current.kind == ClassOrInterface::INTERFACE {
        new.modifiers.push(modifier_abstract);
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
            new.parameters.push(VariableEnvironment::new(param.clone()));
        }
    }

    for method in current.methods.clone() {
        if method.name == new.name && method.parameters == new.parameters {
            return Err("methods must have unique signatures".to_owned());
        }
    }

    current.methods.push(new);
    Ok(())
}

pub fn analyze_method_declaration(current: &mut ClassOrInterfaceEnvironment,
                                  header: &ASTNode,
                                  body: &ASTNode)
                                  -> Result<(), String> {
    let declarator = header.children[2].clone();

    let mut new = MethodEnvironment::new(declarator.children[0].clone(),
                                         header.children[1].clone());

    for child in header.children[0].clone().children {
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
            new.parameters.push(VariableEnvironment::new(param.clone()));
        }
    }

    new.body = Some(body.clone());

    for method in current.methods.clone() {
        if method.name == new.name && method.parameters == new.parameters {
            return Err("methods must have unique signatures".to_owned());
        }
    }

    current.methods.push(new);
    Ok(())
}

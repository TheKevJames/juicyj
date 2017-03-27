use std::fmt;

use analysis::environment::classorinterface::ClassOrInterface;
use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use analysis::environment::variable::VariableEnvironment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref NATIVE: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Native, None), children: Vec::new() }
    };
}

#[derive(Clone,Debug)]
pub struct MethodEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub return_type: ASTNode,
    pub name: ASTNode,
    pub parameters: Vec<VariableEnvironment>,
    pub parent: Option<ASTNode>,
    pub body: Option<ASTNode>,
}

impl MethodEnvironment {
    pub fn new(name: ASTNode, return_type: ASTNode) -> MethodEnvironment {
        MethodEnvironment {
            modifiers: Vec::new(),
            return_type: return_type,
            name: name,
            parameters: Vec::new(),
            parent: None,
            body: None,
        }
    }

    pub fn to_label(&self, class_label: String) -> Result<String, String> {
        let mut label: Vec<String> = Vec::new();
        label.push("__".to_owned());
        if self.modifiers.contains(&*NATIVE) {
            label.push("NATIVE".to_owned());
        }

        label.push(class_label);
        label.push(".".to_owned());
        match self.name.to_label() {
            Ok(l) => label.push(l),
            Err(e) => return Err(e),
        }

        // TODO<codegen>: only when required? Also see methodinvocation.rs/build_method
        label.push("_".to_owned());
        for param in &self.parameters {
            match param.kind.to_param() {
                Ok(p) => label.push(p),
                Err(e) => return Err(e),
            }
        }
        label.push("_".to_owned());

        Ok(label.join(""))
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

            let mut param = VariableEnvironment::new(param.clone());
            param.initialized = true;
            new.parameters.push(param);
        }
    }

    for method in current.methods.clone() {
        if method.name == new.name && method.parameters == new.parameters {
            // TODO: check after inheritance?
            return Err("methods must have unique signatures".to_owned());
        }
    }

    current.methods.push(new);
    Ok(())
}

pub fn analyze_constructor_declaration(current: &mut ClassOrInterfaceEnvironment,
                                       modifiers: &ASTNode,
                                       declarator: &ASTNode,
                                       body: &ASTNode)
                                       -> Result<(), String> {
    let mut new = MethodEnvironment::new(declarator.children[0].clone(), current.name.clone());

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

            let mut param = VariableEnvironment::new(param.clone());
            param.initialized = true;
            new.parameters.push(param);
        }
    }

    new.body = Some(body.clone());

    for constructor in &current.constructors {
        if constructor.parameters == new.parameters {
            return Err("constructors must have unique signatures".to_owned());
        }
    }

    current.constructors.push(new);
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

            let mut param = VariableEnvironment::new(param.clone());
            param.initialized = true;
            new.parameters.push(param);
        }
    }

    new.body = Some(body.clone());

    for method in current.methods.clone() {
        if method.name == new.name && method.parameters == new.parameters {
            // TODO: check after inheritance?
            return Err("methods must have unique signatures".to_owned());
        }
    }

    current.methods.push(new);
    Ok(())
}

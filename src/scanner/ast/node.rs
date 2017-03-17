use std::fmt;

use error::ASTError;
use error::ErrorMessage;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::parser::ParseNode;

#[derive(Clone,Debug)]
/// An individual node in the abstract syntax tree.
pub struct ASTNode {
    /// the associated token object for this node
    pub token: Token,
    /// all children of this node, ordered left-to-right
    pub children: Vec<ASTNode>,
}

#[derive(Clone,Debug,PartialEq)]
/// An AST node denoting an import statement.
pub struct ASTNodeImport {
    /// the associated ASTNode, usually a <Name> NonTerminal
    pub import: ASTNode,
}

#[derive(PartialEq)]
pub struct ASTNodePackage {
    pub package: ASTNode,
}

impl ASTNode {
    /// Transforms a ParseNode to an ASTNode by simplifying tree structure
    /// wherever possible. Catches some syntax errors as the tree becomes simple
    /// enough to operate on.
    pub fn new(node: &ParseNode) -> Result<ASTNode, ASTError> {
        match node.token.kind {
            TokenKind::NonTerminal => {
                match node.token.lexeme {
                    Some(ref l) if node.children.len() == 4 && l == "CastExpression" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match ASTNode::new(&child) {
                                Ok(child) => children.push(child),
                                Err(e) => return Err(e),
                            }
                        }

                        if node.children[1].token.lexeme == Some("Expression".to_owned()) {
                            match children[1].token.kind {
                                TokenKind::Dot => (),
                                TokenKind::Identifier => (),
                                TokenKind::NonTerminal => {
                                    if children[1].token.lexeme != Some("Name".to_owned()) {
                                        return Err(ASTError::new(ErrorMessage::InvalidCast,
                                                                 &children[1]));
                                    }
                                }
                                _ => {
                                    return Err(ASTError::new(ErrorMessage::InvalidCast,
                                                             &children[1]));
                                }
                            }

                            let mut nodes: Vec<ParseNode> = Vec::new();
                            node.children[1].collect_child_lexeme("PrimaryNoNewArray", &mut nodes);
                            for n in &nodes {
                                if n.children.len() != 1 {
                                    return Err(ASTError::new(ErrorMessage::InvalidCast, n));
                                }
                            }
                        }

                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 3 && l == "ArrayCreationExpression" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match ASTNode::new(&node.children[1]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        match ASTNode::new(&node.children[2]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if l == "ClassInstanceCreationExpression" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match ASTNode::new(&node.children[1]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        if node.children.len() == 5 {
                            match ASTNode::new(&node.children[3]) {
                                Ok(child) => children.push(child),
                                Err(e) => return Err(e),
                            }
                        }
                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 3 &&
                                   (l.ends_with("Expression") || l == "VariableDeclarator") => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match ASTNode::new(&node.children[0]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        match ASTNode::new(&node.children[2]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        Ok(ASTNode {
                            token: node.children[1].token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 3 && l == "PrimaryNoNewArray" => {
                        ASTNode::new(&node.children[1])
                    }
                    Some(ref l) if node.children.len() == 3 && l == "QualifiedName" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match ASTNode::new(&child) {
                                Ok(child) => {
                                    match child.token.lexeme {
                                        Some(ref l) if l == "QualifiedName" || l == "Name" => {
                                            for grandkid in child.children {
                                                children.push(grandkid);
                                            }
                                        }
                                        _ => children.push(child),
                                    }
                                }
                                Err(e) => return Err(e),
                            }
                        }

                        Ok(ASTNode {
                            token: node.children[0].token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 2 && l == "UnaryExpression" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        children.push(ASTNode {
                            token: Token {
                                kind: TokenKind::NumValue,
                                lexeme: Some("0".to_owned()),
                            },
                            children: Vec::new(),
                        });
                        match ASTNode::new(&node.children[1]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }

                        Ok(ASTNode {
                            token: node.children[0].token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 2 && l == "UnaryNoSignExpression" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match ASTNode::new(&node.children[1]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }

                        Ok(ASTNode {
                            token: node.children[0].token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 2 && l == "AbstractMethodDeclaration" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match ASTNode::new(&node.children[0]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }

                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 2 && l == "ReturnStatement" => {
                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: Vec::new(),
                        })
                    }
                    Some(_) if node.children.len() == 2 &&
                               node.children[1].token.kind == TokenKind::Semicolon => {
                        ASTNode::new(&node.children[0])
                    }
                    // TODO: does this miss the following case?
                    // CastExpression:
                    //     ( Name Dim ) UnaryNoSignExpression

                    // parent of UnaryExpression
                    Some(ref l) if node.children.len() == 1 && l == "MultiplicativeExpression" => {
                        match ASTNode::new(&node.children[0]) {
                            Ok(node) => {
                                if node.token.kind == TokenKind::NumValue {
                                    match node.token.lexeme {
                                        Some(ref l) if l.parse().unwrap_or(0) >
                                                       2u64.pow(31) - 1 => {
                                            return Err(ASTError::new(ErrorMessage::IntOOB, &node));
                                        }
                                        _ => (),
                                    }
                                }

                                Ok(node)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Some(ref l) if node.children.len() == 1 &&
                                   (l == "Name" || l == "ArgumentList") => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match ASTNode::new(&node.children[0]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }

                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if l == "Modifiers" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match ASTNode::new(&child) {
                                Ok(child) => children.push(child),
                                Err(e) => return Err(e),
                            }
                        }
                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(_) if node.children.len() == 1 => ASTNode::new(&node.children[0]),
                    _ => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match ASTNode::new(&child) {
                                Ok(child) => children.push(child),
                                Err(e) => return Err(e),
                            }
                        }
                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                }
            }
            _ => {
                Ok(ASTNode {
                    token: node.token.clone(),
                    children: Vec::new(),
                })
            }
        }
    }

    /// Attempt to flatten the current node and children into a simpler tree.
    /// Useful for flattening any NonTerminal which can contain itself.
    pub fn flatten(&mut self) -> &ASTNode {
        if self.token.kind != TokenKind::NonTerminal {
            return &*self;
        }

        let flattenable = match self.token.lexeme {
            Some(ref l) => l.clone(),
            _ => return &*self,
        };

        self.children = self.get_flattened_children(&flattenable);
        &*self
    }

    fn get_flattened_children(&self, flattenable: &String) -> Vec<ASTNode> {
        let mut children = Vec::new();
        for child in &self.children {
            if &child.clone().token.lexeme.unwrap_or("".to_owned()) == flattenable {
                children.extend(child.get_flattened_children(flattenable));
            } else {
                children.push(child.clone());
            }
        }

        children
    }

    /// Used for performing type comparisions, eg. true.is_same_type(false), so
    /// we can compare them.
    ///
    // Really, this should be on Type under PartialEq, but I fucked up and have
    // Type::PartialEq called for doing comparisons of method names and stuff...
    // Nasty methods, just nasty.
    pub fn is_same_type(&self, other: &ASTNode) -> bool {
        for (child, other_child) in self.children.iter().zip(other.children.iter()) {
            if !child.is_same_type(other_child) {
                return false;
            }
        }

        if self.token == other.token {
            return true;
        }

        // weirdness follows
        if self.token.kind != other.token.kind {
            return false;
        }

        let primitives = vec![TokenKind::Boolean, TokenKind::Int];
        primitives.contains(&self.token.kind)
    }

    /// Convenience function for recursive ASTNode printing. Should be accessed
    /// with the standard print command (ie. `fmt::Display`).
    pub fn print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        match indent {
            0 => try!(write!(f, "{:width$}{}", "", self.token, width = indent)),
            _ => try!(write!(f, "{:width$}{}", "\n", self.token, width = indent)),
        }
        for child in &self.children {
            try!(child.print(f, indent + 2));
        }
        Ok(())
    }
}

impl ASTNodeImport {
    /// Creates an ASTNodeImport from a parse node of form: `import { a.b.[c*] }`
    /// End result is `<Name> { a . b . [c*] }`
    pub fn new(node: &ParseNode) -> Result<ASTNodeImport, ASTError> {
        let mut names: Vec<Token> = Vec::new();
        node.collect_child_kinds(&vec![&TokenKind::Dot, &TokenKind::Identifier, &TokenKind::Star],
                                 &mut names);

        let mut import = ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("Name")),
            children: Vec::new(),
        };
        for token in &names {
            import.children.push(ASTNode {
                token: token.clone(),
                children: Vec::new(),
            });
        }

        Ok(ASTNodeImport { import: import })
    }
}

impl ASTNodePackage {
    pub fn new(node: &ParseNode) -> Result<ASTNodePackage, ASTError> {
        let mut names: Vec<Token> = Vec::new();
        node.children[1]
            .collect_child_kinds(&vec![&TokenKind::Dot, &TokenKind::Identifier], &mut names);

        let mut package = ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("Name")),
            children: Vec::new(),
        };
        for token in &names {
            package.children.push(ASTNode {
                token: token.clone(),
                children: Vec::new(),
            });
        }

        Ok(ASTNodePackage { package: package })
    }
}

impl PartialEq for ASTNode {
    fn eq(&self, other: &ASTNode) -> bool {
        if self.token == other.token && self.children == other.children {
            return true;
        }

        // TODO: remove this child stuff
        // Note that a bunch of stuff relies on it, so... do it anyway.
        if self.children.len() == 1 &&
           self.clone().token.lexeme.unwrap_or("".to_owned()) == "Name" {
            if &self.children[0] == other {
                return true;
            }
        }

        if other.children.len() == 1 &&
           other.clone().token.lexeme.unwrap_or("".to_owned()) == "Name" {
            if self == &other.children[0] {
                return true;
            }
        }

        false
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f, 0)
    }
}

impl fmt::Display for ASTNodeImport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", &self.import)
    }
}

impl fmt::Display for ASTNodePackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", &self.package)
    }
}

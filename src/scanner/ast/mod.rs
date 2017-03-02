mod node;

use std::fmt;

use error::ASTError;
use error::ErrorMessage;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::parser::ParseTree;

pub use self::node::ASTNodeImport;
pub use self::node::ASTNode;
use self::node::ASTNodePackage;

/// An abstract syntax tree generated at the end of the scanning step.
pub struct AST {
    /// the file from which this AST was generated
    pub filename: String,
    /// the package hierarchy for this file -- defaults to "unnamed"
    pub package: ASTNodePackage,
    /// the list of import statements in this file
    pub imports: Vec<ASTNodeImport>,
    /// pointer to the root node of the AST for this file
    pub root: Option<ASTNode>,
    /// canonical name of the definied class or interface in this tree --
    /// effectively "package + name"
    pub canonical: ASTNode,
}

impl AST {
    /// Transforms a ParseTree to an AST by through three operations: capturing
    /// the package declaration as an ASTNodePackage, capturing the import
    /// statements as ASTNodeImports, and performing a recursive build on the
    /// remaining tree into ASTNodes.
    pub fn new(filename: &str, tree: &ParseTree) -> Result<AST, ASTError> {
        let token_name = Token::new(TokenKind::NonTerminal, Some("Name"));

        let mut imports = Vec::new();
        let mut package = ASTNodePackage {
            package: ASTNode {
                token: token_name.clone(),
                children: Vec::new(),
            },
        };
        let mut root = None;
        let mut name = None;
        for child in &tree.root.children {
            match child.token.lexeme {
                Some(ref l) if l == "PackageDeclaration" => {
                    package = match ASTNodePackage::new(&child) {
                        Ok(i) => i,
                        Err(e) => return Err(e),
                    };
                }
                Some(ref l) if l == "ImportDeclarations" => {
                    let mut statements = Vec::new();
                    child.collect_child_lexeme("ImportDeclaration", &mut statements);

                    for child in statements {
                        match ASTNodeImport::new(&child) {
                            Ok(i) => {
                                if !imports.contains(&i) {
                                    imports.push(i);
                                }
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
                Some(ref l) if l == "TypeDeclaration" => {
                    root = match ASTNode::new(&child) {
                        Ok(i) => {
                            // TODO: check for semicolons? range check?
                            name = Some(i.children[2].clone().token);
                            Some(i)
                        }
                        Err(e) => return Err(e),
                    };
                }
                _ => return Err(ASTError::new(ErrorMessage::InvalidRootChild, child)),
            }
        }


        imports.push(ASTNodeImport {
            import: ASTNode {
                token: token_name.clone(),
                children: vec![ASTNode {
                                   token: Token::new(TokenKind::Identifier, Some("java")),
                                   children: Vec::new(),
                               },
                               ASTNode {
                                   token: Token::new(TokenKind::Dot, None),
                                   children: Vec::new(),
                               },
                               ASTNode {
                                   token: Token::new(TokenKind::Identifier, Some("lang")),
                                   children: Vec::new(),
                               },
                               ASTNode {
                                   token: Token::new(TokenKind::Dot, None),
                                   children: Vec::new(),
                               },
                               ASTNode {
                                   token: Token::new(TokenKind::Star, None),
                                   children: Vec::new(),
                               }],
            },
        });

        let mut canonical = package.package.clone();
        match name {
            Some(ref n) => {
                canonical.children.push(ASTNode {
                    token: Token::new(TokenKind::Dot, None),
                    children: Vec::new(),
                });
                canonical.children.push(ASTNode {
                    token: n.clone(),
                    children: Vec::new(),
                });
            }
            _ => return Err(ASTError::new(ErrorMessage::MissingName, &root.unwrap())),
        };

        Ok(AST {
            filename: filename.to_owned(),
            imports: imports,
            package: package,
            root: root,
            canonical: canonical,
        })
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Package: <{}>", self.package));

        if !self.imports.is_empty() {
            try!(writeln!(f, "Imports:"));
        }
        for i in &self.imports {
            try!(writeln!(f, "{:2}", i));
        }

        write!(f, "{}", self.root.clone().unwrap())
    }
}

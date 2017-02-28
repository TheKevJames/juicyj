mod node;

use std::fmt;

use error::ASTError;
use error::ErrorMessage;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::parser::ParseTree;

use self::node::ASTNodeImport;
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
}

impl AST {
    /// Transforms a ParseTree to an AST by through three operations: capturing
    /// the package declaration as an ASTNodePackage, capturing the import
    /// statements as ASTNodeImports, and performing a recursive build on the
    /// remaining tree into ASTNodes.
    pub fn new(filename: &str, tree: &ParseTree) -> Result<AST, ASTError> {
        let mut imports = Vec::new();
        let mut package = ASTNodePackage {
            package: vec![Token {
                              kind: TokenKind::Identifier,
                              lexeme: Some("unnamed".to_owned()),
                          }],
        };
        let mut root = None;
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
                        imports.push(match ASTNodeImport::new(&child) {
                            Ok(i) => i,
                            Err(e) => return Err(e),
                        });
                    }
                }
                Some(ref l) if l == "TypeDeclarations" => {
                    root = match ASTNode::new(&child) {
                        Ok(i) => Some(i),
                        Err(e) => return Err(e),
                    };
                }
                _ => return Err(ASTError::new(ErrorMessage::InvalidRootChild, child)),
            }
        }

        Ok(AST {
            filename: filename.to_owned(),
            imports: imports,
            package: package,
            root: root,
        })
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Package: {}", self.package));

        if !self.imports.is_empty() {
            try!(writeln!(f, "Imports:"));
        }
        for i in &self.imports {
            try!(writeln!(f, "{:2}", i));
        }

        write!(f, "{}", self.root.clone().unwrap())
    }
}

mod node;

use std::fmt;

use error::ASTError;
use error::ErrorMessage;
use scanner::parser::ParseTree;

use self::node::ASTNodeImport;
pub use self::node::ASTNode;
use self::node::ASTNodePackage;

/// An abstract syntax tree generated at the end of the scanning step.
pub struct AST {
    /// the (optional) package hierarchy for this file
    pub package: Option<ASTNodePackage>,
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
    pub fn new(tree: &ParseTree) -> Result<AST, ASTError> {
        let mut imports = Vec::new();
        let mut package = None;
        let mut root = None;
        for child in &tree.root.children {
            match child.token.lexeme {
                Some(ref l) if l == "PackageDeclaration" => {
                    package = match ASTNodePackage::new(&child) {
                        Ok(i) => Some(i),
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
            imports: imports,
            package: package,
            root: root,
        })
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.package {
            Some(ref p) => try!(writeln!(f, "Package: {}", p)),
            None => try!(writeln!(f, "[no package declaration]")),
        }

        if !self.imports.is_empty() {
            try!(writeln!(f, "Imports:"));
        }
        for i in &self.imports {
            try!(writeln!(f, "{:2}", i));
        }

        write!(f, "{}", self.root.clone().unwrap())
    }
}

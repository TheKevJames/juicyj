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
    /// name of the definied class or interface in this tree
    pub name: Option<Token>,
    /// canonical name of the definied class or interface in this tree --
    /// effectively "package + name"
    pub canonical: Vec<Token>,
}

impl AST {
    /// Transforms a ParseTree to an AST by through three operations: capturing
    /// the package declaration as an ASTNodePackage, capturing the import
    /// statements as ASTNodeImports, and performing a recursive build on the
    /// remaining tree into ASTNodes.
    pub fn new(filename: &str, tree: &ParseTree) -> Result<AST, ASTError> {
        let mut imports = Vec::new();
        let mut package = ASTNodePackage { package: Vec::new() };
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

        if package.package !=
           vec![Token::new(TokenKind::Identifier, Some("java")),
                Token::new(TokenKind::Identifier, Some("lang"))] {
            imports.push(ASTNodeImport {
                import: vec![Token::new(TokenKind::Identifier, Some("java")),
                             Token::new(TokenKind::Identifier, Some("lang")),
                             Token::new(TokenKind::Star, None)],
            });
        }

        let mut canonical = package.package.clone();
        let cname = match name {
            Some(ref n) => n.clone(),
            _ => return Err(ASTError::new(ErrorMessage::MissingName, &root.unwrap())),
        };
        canonical.push(cname);

        Ok(AST {
            filename: filename.to_owned(),
            imports: imports,
            package: package,
            root: root,
            name: name,
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

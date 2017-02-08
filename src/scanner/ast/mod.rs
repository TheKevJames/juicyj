mod node;

use std::fmt;

use scanner::common::error;
use scanner::parser::Node;
use scanner::parser::Tree;

use self::node::ASTNodeImport;
use self::node::ASTNode;
use self::node::ASTNodePackage;

pub struct AST {
    pub package: Option<ASTNodePackage>,
    pub imports: Vec<ASTNodeImport>,
    pub root: Option<ASTNode>,
}

impl AST {
    pub fn new(parse_tree: &Tree) -> Result<AST, error::ASTError> {
        let mut imports = Vec::new();
        let mut package = None;
        let mut root = None;
        for child in &parse_tree.root.children {
            match child.token.lexeme {
                Some(ref l) if l == "PackageDeclaration" => {
                    package = match ASTNodePackage::new(&child) {
                        Ok(i) => Some(i),
                        Err(e) => return Err(e),
                    };
                }
                Some(ref l) if l == "ImportDeclarations" => {
                    let mut statements: Vec<Node> = Vec::new();
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
                _ => return Err(error::ASTError { message: error::INVALID_ROOT_CHILD }),
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

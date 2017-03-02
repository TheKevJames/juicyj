use std::fmt;

use analysis::environment::constructor::ConstructorEnvironment;
use analysis::environment::field::FieldEnvironment;
use analysis::environment::method::MethodEnvironment;
use scanner::ASTNode;
use scanner::ASTNodeImport;

#[derive(Clone,Debug,PartialEq)]
pub enum ClassOrInterface {
    CLASS,
    INTERFACE,
}

#[derive(Clone,Debug)]
pub struct ClassOrInterfaceEnvironment {
    pub constructors: Vec<ConstructorEnvironment>,
    pub extends: Vec<ASTNode>,
    pub fields: Vec<FieldEnvironment>,
    pub implements: Vec<ASTNode>,
    pub imports: Vec<ASTNodeImport>,
    pub methods: Vec<MethodEnvironment>,
    pub modifiers: Vec<ASTNode>,
    pub name: ASTNode,
    pub kind: ClassOrInterface,
}

impl ClassOrInterfaceEnvironment {
    pub fn new(name: ASTNode, kind: ClassOrInterface) -> ClassOrInterfaceEnvironment {
        ClassOrInterfaceEnvironment {
            constructors: Vec::new(),
            extends: Vec::new(),
            fields: Vec::new(),
            implements: Vec::new(),
            imports: Vec::new(),
            kind: kind,
            methods: Vec::new(),
            modifiers: Vec::new(),
            name: name,
        }
    }
}

impl fmt::Display for ClassOrInterfaceEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

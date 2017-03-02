use analysis::environment::constructor::ConstructorEnvironment;
use analysis::environment::field::FieldEnvironment;
use analysis::environment::method::MethodEnvironment;
use scanner::ASTNode;
use scanner::Token;

#[derive(Clone,Debug,PartialEq)]
pub enum ClassOrInterface {
    CLASS,
    INTERFACE,
}

#[derive(Clone,Debug)]
pub struct ClassOrInterfaceEnvironment {
    pub constructors: Vec<ConstructorEnvironment>,
    pub extends: Vec<Vec<Token>>,
    pub fields: Vec<FieldEnvironment>,
    pub implements: Vec<Vec<Token>>,
    pub methods: Vec<MethodEnvironment>,
    pub modifiers: Vec<ASTNode>,
    pub name: Vec<Token>,
    pub kind: ClassOrInterface,
}

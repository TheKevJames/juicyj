use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup::array;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref ARRAYTYPE: Token = Token::new(TokenKind::NonTerminal, Some("ArrayType"));
}

pub fn go(node: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    match resolve::expression::go(&node.children[1], current, kinds, globals) {
        // CastExpression has 5 children iff it contains a DimExpr
        Ok(ref x) if node.children.len() == 5 => {
            let kind = ASTNode {
                token: ARRAYTYPE.clone(),
                children: vec![x.kind.name.clone()],
            };
            Ok(Type::new(array::create(&kind)))
        }
        x => x,
    }
}

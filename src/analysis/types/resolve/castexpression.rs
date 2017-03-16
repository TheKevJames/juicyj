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
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &mut Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let expr = node.children.last().unwrap().clone();
    let rhs = match resolve::expression::go(&expr, modifiers, current, kinds, globals) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let lhs =
        match resolve::expression::go(&node.children[1], modifiers, current, kinds, globals) {
            // CastExpression has 5 children iff it contains a DimExpr
            Ok(ref t) if node.children.len() == 5 => {
                let kind = ASTNode {
                    token: ARRAYTYPE.clone(),
                    children: vec![t.kind.name.clone()],
                };
                Type::new(array::create(&kind))
            }
            Ok(t) => t,
            Err(e) => return Err(e),
        };

    lhs.apply_cast(&rhs, current, kinds)
}

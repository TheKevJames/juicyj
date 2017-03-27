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

pub fn go(mut node: &mut ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &mut Vec<VariableEnvironment>)
          -> Result<Type, String> {
    if node.children[1].clone().token.lexeme.unwrap() == "DimExpr" {
        match resolve::expression::go(&mut node.children[1].children[1], modifiers, current, kinds, globals) {
            Ok(ref idx) if idx.is_coercible_to_int() => (),
            Ok(idx) => return Err(format!("got invalid index type {:?}", idx.kind.name)),
            Err(e) => return Err(e),
        }
    }

    match resolve::expression::go(&mut node.children[0], modifiers, current, kinds, globals) {
        Ok(x) => {
            let kind = ASTNode {
                token: ARRAYTYPE.clone(),
                children: vec![x.kind.name.clone()],
            };
            Ok(Type::new(array::create(&kind)))
        }
        Err(e) => return Err(e),
    }
}

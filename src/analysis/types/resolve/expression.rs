use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::TokenKind;

pub fn go(node: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    match node.token.lexeme {
        Some(ref l) if l == "ArrayAccess" => {
            resolve::arrayaccess::go(node, current, kinds, globals)
        }
        Some(ref l) if l == "ArrayCreationExpression" => {
            resolve::arraycreationexpression::go(node, current, kinds, globals)
        }
        Some(ref l) if l == "ArrayType" => {
            Ok(Type::new(ClassOrInterfaceEnvironment::new(node.clone(), ClassOrInterface::CLASS)))
        }
        Some(ref l) if l == "Assignment" => resolve::assignment::go(node, current, kinds, globals),
        Some(ref l) if l == "CastExpression" => {
            resolve::castexpression::go(node, current, kinds, globals)
        }
        Some(ref l) if l == "ClassInstanceCreationExpression" => {
            resolve::classinstancecreationexpression::go(node, current, kinds)
        }
        Some(ref l) if l == "FieldAccess" => {
            resolve::fieldaccess::go(node, current, kinds, globals)
        }
        Some(ref l) if l == "MethodInvocation" => {
            resolve::methodinvocation::go(node, current, kinds, globals)
        }
        Some(ref l) if l == "Name" => resolve::name::go(node, current, kinds, globals),
        _ => {
            match node.token.kind {
                TokenKind::And | TokenKind::BitAnd | TokenKind::Or | TokenKind::BitOr |
                TokenKind::BitXor => {
                    resolve::comparison::twoarg_boolean(node, current, kinds, globals)
                }
                TokenKind::Not => {
                    resolve::comparison::onearg_boolean(node, current, kinds, globals)
                }
                TokenKind::Equality |
                TokenKind::NotEqual |
                TokenKind::LessThan |
                TokenKind::LessThanOrEqual |
                TokenKind::GreaterThan |
                TokenKind::GreaterThanOrEqual => {
                    resolve::comparison::twoarg(node, current, kinds, globals)
                }
                TokenKind::Instanceof => {
                    resolve::comparison::twoarg_instanceof(node, current, kinds, globals)
                }
                TokenKind::FSlash | TokenKind::Minus | TokenKind::Percent | TokenKind::Plus |
                TokenKind::Star => resolve::math::go(node, current, kinds, globals),
                TokenKind::Boolean | TokenKind::Byte | TokenKind::Char | TokenKind::Int |
                TokenKind::Null | TokenKind::Short => resolve::primitive::go(node),
                TokenKind::This => Ok(Type::new(current.clone())),
                _ => Err(format!("could not resolve expression {:?}", node)),
            }
        }
    }
}

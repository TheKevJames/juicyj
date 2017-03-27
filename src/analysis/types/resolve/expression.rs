use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref NULL: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Null, None), children: Vec::new() }
    };
}

pub fn go(mut node: &mut ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &mut Vec<VariableEnvironment>)
          -> Result<Type, String> {
    match node.clone().token.lexeme {
        Some(ref l) if l == "Argument" => {
            go(&mut node.children[1], modifiers, current, kinds, globals)
        }
        Some(ref l) if l == "ArrayAccess" => {
            resolve::arrayaccess::go(node, modifiers, current, kinds, globals)
        }
        Some(ref l) if l == "ArrayCreationExpression" => {
            resolve::arraycreationexpression::go(node, modifiers, current, kinds, globals)
        }
        Some(ref l) if l == "ArrayType" => {
            Ok(Type::new(ClassOrInterfaceEnvironment::new(node.clone(), ClassOrInterface::CLASS)))
        }
        Some(ref l) if l == "Assignment" => {
            resolve::assignment::go(node, modifiers, current, kinds, globals)
        }
        Some(ref l) if l == "CastExpression" => {
            resolve::castexpression::go(node, modifiers, current, kinds, globals)
        }
        Some(ref l) if l == "ClassInstanceCreationExpression" => {
            resolve::classinstancecreationexpression::go(node, modifiers, current, kinds, globals)
        }
        Some(ref l) if l == "FieldAccess" => {
            resolve::fieldaccess::go(node, modifiers, current, kinds, globals)
        }
        Some(ref l) if l == "MethodInvocation" => {
            resolve::methodinvocation::go(&mut node, modifiers, current, kinds, globals)
        }
        Some(ref l) if l == "Name" => resolve::name::go(node, modifiers, current, kinds, globals),
        _ => {
            match node.token.kind {
                TokenKind::And | TokenKind::BitAnd | TokenKind::Or | TokenKind::BitOr |
                TokenKind::BitXor => {
                    resolve::comparison::twoarg_boolean(node, modifiers, current, kinds, globals)
                }
                TokenKind::Not => {
                    resolve::comparison::onearg_boolean(node, modifiers, current, kinds, globals)
                }
                TokenKind::Equality |
                TokenKind::NotEqual |
                TokenKind::LessThan |
                TokenKind::LessThanOrEqual |
                TokenKind::GreaterThan |
                TokenKind::GreaterThanOrEqual => {
                    resolve::comparison::twoarg(node, modifiers, current, kinds, globals)
                }
                TokenKind::Instanceof => {
                    resolve::comparison::twoarg_instanceof(node, modifiers, current, kinds, globals)
                }
                TokenKind::FSlash | TokenKind::Minus | TokenKind::Percent | TokenKind::Plus |
                TokenKind::Star => resolve::math::go(node, modifiers, current, kinds, globals),
                TokenKind::Boolean | TokenKind::Byte | TokenKind::Char | TokenKind::CharValue |
                TokenKind::False | TokenKind::Int | TokenKind::Null | TokenKind::NumValue |
                TokenKind::Short | TokenKind::StrValue | TokenKind::True => {
                    resolve::primitive::go(node)
                }
                TokenKind::This => Ok(Type::new(current.clone())),
                // Naked returns and empty statements resolve to Null
                TokenKind::Return => resolve::primitive::go(&NULL.clone()),
                TokenKind::Semicolon => resolve::primitive::go(&NULL.clone()),
                _ => Err(format!("could not resolve expression {:?}", node)),
            }
        }
    }
}

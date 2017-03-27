use scanner::ASTNode;
use scanner::TokenKind;

use super::arrayaccess;
use super::arraycreationexpression;
use super::assignment;
use super::booleanoperation;
use super::booleanvalue;
use super::castexpression;
use super::charvalue;
use super::classinstancecreationexpression;
use super::comparison;
use super::fieldaccess;
use super::forstatement;
use super::ifelsestatement;
use super::ifstatement;
use super::instanceof;
use super::localvariabledeclaration;
use super::math;
use super::methodinvocation;
use super::name;
use super::not;
use super::nullvalue;
use super::numvalue;
use super::returnstatement;
use super::strvalue;
use super::this;
use super::whilestatement;

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    match node.token.kind {
        TokenKind::NonTerminal => {
            match node.token.lexeme {
                Some(ref l) if l == "Argument" => {
                    go(&node.children[1],
                       label,
                       &mut text,
                       &mut externs,
                       &mut bss,
                       &mut data)
                }
                Some(ref l) if l == "ArrayAccess" => {
                    arrayaccess::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "ArrayCreationExpression" => {
                    arraycreationexpression::go(&node,
                                                label,
                                                &mut text,
                                                &mut externs,
                                                &mut bss,
                                                &mut data)
                }
                Some(ref l) if l == "Assignment" => {
                    assignment::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "Block" && node.children.len() == 3 => {
                    go(&node.children[1],
                       label,
                       &mut text,
                       &mut externs,
                       &mut bss,
                       &mut data)
                }
                Some(ref l) if l == "BlockStatements" => {
                    for child in &node.children {
                        match go(&child, label, &mut text, &mut externs, &mut bss, &mut data) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(())
                }
                Some(ref l) if l == "CastExpression" => {
                    castexpression::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "ClassInstanceCreationExpression" => {
                    classinstancecreationexpression::go(&node,
                                                        label,
                                                        &mut text,
                                                        &mut externs,
                                                        &mut bss,
                                                        &mut data)
                }
                Some(ref l) if l == "FieldAccess" => {
                    fieldaccess::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "ForStatement" || l == "ForStatementNoShortIf" => {
                    forstatement::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "IfElseStatement" || l == "IfElseStatementNoShortIf" => {
                    ifelsestatement::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "IfStatement" => {
                    ifstatement::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "LocalVariableDeclaration" => {
                    localvariabledeclaration::go(&node,
                                                 label,
                                                 &mut text,
                                                 &mut externs,
                                                 &mut bss,
                                                 &mut data)
                }
                Some(ref l) if l == "MethodInvocation" => {
                    methodinvocation::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "Name" => {
                    name::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "ReturnStatement" => {
                    returnstatement::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "WhileStatement" || l == "WhileStatementNoShortIf" => {
                    whilestatement::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
                }

                Some(ref l) if l == "Block" => Ok(()),
                _ => Err(format!("attempted to generate code for {:?}", node)),
            }
        }
        TokenKind::And | TokenKind::BitAnd | TokenKind::Or | TokenKind::BitOr |
        TokenKind::BitXor => {
            booleanoperation::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
        }
        TokenKind::CharValue => charvalue::go(&node, &mut text, &mut data),
        TokenKind::Equality |
        TokenKind::NotEqual |
        TokenKind::LessThan |
        TokenKind::LessThanOrEqual |
        TokenKind::GreaterThan |
        TokenKind::GreaterThanOrEqual => {
            comparison::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
        }
        TokenKind::False | TokenKind::True => booleanvalue::go(&node, &mut text),
        TokenKind::Identifier => {
            name::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
        }
        TokenKind::Instanceof => {
            instanceof::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
        }
        TokenKind::FSlash | TokenKind::Minus | TokenKind::Percent | TokenKind::Plus |
        TokenKind::Star => math::go(&node, label, &mut text, &mut externs, &mut bss, &mut data),
        TokenKind::Not => not::go(&node, label, &mut text, &mut externs, &mut bss, &mut data),
        TokenKind::Null => {
            nullvalue::go(&node, label, &mut text, &mut externs, &mut bss, &mut data)
        }
        TokenKind::NumValue => numvalue::go(&node, &mut text),
        TokenKind::StrValue => strvalue::go(&node, &mut text, &mut data),
        TokenKind::This => this::go(&node, label, &mut text, &mut externs, &mut bss, &mut data),

        // TODO<codegen>: prune AST
        TokenKind::Boolean | TokenKind::Char | TokenKind::Byte | TokenKind::Int |
        TokenKind::Short => Err(format!("attempted to generate code for unpruned {:?}", node)),

        _ => Err(format!("attempted to generate code for {:?}", node)),
    }
}

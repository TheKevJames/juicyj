use scanner::ASTNode;
use scanner::TokenKind;

use super::arrayaccess;
use super::arraycreationexpression;
use super::assignment;
use super::booleanvalue;
use super::castexpression;
use super::charvalue;
use super::classinstancecreationexpression;
use super::equality;
use super::fieldaccess;
use super::forstatement;
use super::ifstatement;
use super::ifelsestatement;
use super::localvariabledeclaration;
use super::math;
use super::methodinvocation;
use super::name;
use super::nullvalue;
use super::numvalue;
use super::returnstatement;
use super::strvalue;
use super::this;
use super::whilestatement;

pub fn go(node: &ASTNode,
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
                       &mut text,
                       &mut externs,
                       &mut bss,
                       &mut data)
                }
                Some(ref l) if l == "ArrayAccess" => {
                    arrayaccess::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "ArrayCreationExpression" => {
                    arraycreationexpression::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "Assignment" => {
                    assignment::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "Block" && node.children.len() == 3 => {
                    go(&node.children[1],
                       &mut text,
                       &mut externs,
                       &mut bss,
                       &mut data)
                }
                Some(ref l) if l == "BlockStatements" => {
                    for child in &node.children {
                        match go(&child, &mut text, &mut externs, &mut bss, &mut data) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(())
                }
                Some(ref l) if l == "CastExpression" => {
                    castexpression::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "ClassInstanceCreationExpression" => {
                    classinstancecreationexpression::go(&node,
                                                        &mut text,
                                                        &mut externs,
                                                        &mut bss,
                                                        &mut data)
                }
                Some(ref l) if l == "FieldAccess" => {
                    fieldaccess::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "ForStatement" => {
                    forstatement::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "IfElseStatement" => {
                    ifelsestatement::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "IfStatement" => {
                    ifstatement::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "LocalVariableDeclaration" => {
                    localvariabledeclaration::go(&node,
                                                 &mut text,
                                                 &mut externs,
                                                 &mut bss,
                                                 &mut data)
                }
                Some(ref l) if l == "MethodInvocation" => {
                    methodinvocation::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "Name" => {
                    name::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "ReturnStatement" => {
                    returnstatement::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }
                Some(ref l) if l == "WhileStatement" => {
                    whilestatement::go(&node, &mut text, &mut externs, &mut bss, &mut data)
                }

                Some(ref l) if l == "Block" => Ok(()),
                _ => {
                    Err(format!("TODO<codegen>: body statement (lexeme) {:?}",
                                node.token.lexeme))
                }
            }
        }
        TokenKind::CharValue => charvalue::go(&node, &mut text, &mut externs, &mut bss, &mut data),
        TokenKind::Equality => equality::go(&node, &mut text, &mut externs, &mut bss, &mut data),
        TokenKind::False | TokenKind::True => {
            booleanvalue::go(&node, &mut text, &mut externs, &mut bss, &mut data)
        }
        TokenKind::Minus | TokenKind::Plus => {
            math::go(&node, &mut text, &mut externs, &mut bss, &mut data)
        }
        TokenKind::Null => nullvalue::go(&node, &mut text, &mut externs, &mut bss, &mut data),
        TokenKind::NumValue => numvalue::go(&node, &mut text, &mut externs, &mut bss, &mut data),
        TokenKind::StrValue => strvalue::go(&node, &mut text, &mut externs, &mut bss, &mut data),
        TokenKind::This => this::go(&node, &mut text, &mut externs, &mut bss, &mut data),
        _ => Err(format!("TODO<codegen>: body statement (kind) {:?}", node.token.kind)),
    }
}

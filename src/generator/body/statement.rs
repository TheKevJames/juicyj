use scanner::ASTNode;
use scanner::TokenKind;

use super::arraycreationexpression;
use super::assignment;
use super::booleanvalue;
use super::forstatement;
use super::ifstatement;
use super::ifelsestatement;
use super::localvariabledeclaration;
use super::methodinvocation;
use super::numvalue;
use super::returnstatement;
use super::whilestatement;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>) {
    match node.token.kind {
        TokenKind::NonTerminal => {
            match node.token.lexeme {
                Some(ref l) if l == "ArrayCreationExpression" => {
                    arraycreationexpression::go(&node, &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "Assignment" => {
                    assignment::go(&node, &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "Block" && node.children.len() == 3 => {
                    go(&node.children[1], &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "BlockStatements" => {
                    go(&node.children[0], &mut text, &mut bss, &mut data);
                    if node.children.len() > 1 {
                        go(&node.children[1], &mut text, &mut bss, &mut data);
                    }
                }
                Some(ref l) if l == "ForStatement" => {
                    forstatement::go(&node, &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "IfStatement" => {
                    ifstatement::go(&node, &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "IfElseStatement" => {
                    ifelsestatement::go(&node, &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "LocalVariableDeclaration" => {
                    localvariabledeclaration::go(&node, &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "MethodInvocation" => {
                    methodinvocation::go(&node, &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "ReturnStatement" => {
                    returnstatement::go(&node, &mut text, &mut bss, &mut data);
                }
                Some(ref l) if l == "WhileStatement" => {
                    whilestatement::go(&node, &mut text, &mut bss, &mut data);
                }

                Some(ref l) if l == "Block" => (),
                _ => {
                    println!("TODO:");
                    println!("{:?}", node.token.lexeme);
                }
            }
        }
        TokenKind::False | TokenKind::True => booleanvalue::go(&node, &mut text, &mut bss, &mut data),
        TokenKind::NumValue => numvalue::go(&node, &mut text, &mut bss, &mut data),
        _ => {
            println!("TODO:");
            println!("{:?}", node.token.kind);
        }
    }
}

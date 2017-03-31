use std::collections::HashMap;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use super::statement;

lazy_static! {
    static ref STRINGARRAYINSTANTIATION: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("ArrayCreationExpression")),
            children: vec![
                ASTNode {
                    token: Token::new(TokenKind::NonTerminal, Some("TODO")),
                    children: Vec::new(),
                },
                ASTNode {
                    token: Token::new(TokenKind::NonTerminal, Some("DimExpr")),
                    children: vec![
                        ASTNode {
                            token: Token::new(TokenKind::LBracket, None),
                            children: Vec::new(),
                        },
                    ],
                },
            ],
        }
    };
    static ref STRINGINSTANTIATION: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("ClassInstanceCreationExpression")),
            children: vec![
                ASTNode {
                    token: Token::new(TokenKind::NonTerminal, Some("Name")),
                    children: vec![
                        ASTNode {
                            token: Token::new(TokenKind::Identifier, Some("java")),
                            children: Vec::new(),
                        },
                        ASTNode {
                            token: Token::new(TokenKind::Dot, None),
                            children: Vec::new(),
                        },
                        ASTNode {
                            token: Token::new(TokenKind::Identifier, Some("lang")),
                            children: Vec::new(),
                        },
                        ASTNode {
                            token: Token::new(TokenKind::Dot, None),
                            children: Vec::new(),
                        },
                        ASTNode {
                            token: Token::new(TokenKind::Identifier, Some("String")),
                            children: Vec::new(),
                        },
                        ASTNode {
                            token: Token::new(TokenKind::Dot, None),
                            children: Vec::new(),
                        },
                        ASTNode {
                            token: Token::new(TokenKind::Identifier, Some("String")),
                            children: Vec::new(),
                        },
                    ],
                },
                ASTNode {
                    token: Token::new(TokenKind::NonTerminal, Some("ParameterList")),
                    children: Vec::new(),
                },
            ],
        }
    };
}

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          fields: &HashMap<String, Vec<(String, String)>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<(String, String)>,
          mut data: &mut Vec<String>)
          -> Result<Option<String>, String> {
    if node.token.lexeme.is_none() {
        return Err(format!("StrValue {:?} has no value", node));
    }

    let chars = node.clone().token.lexeme.unwrap();

    text.push(format!("  ; str '{}'", chars));

    let mut charinstantiation = STRINGARRAYINSTANTIATION.clone();
    charinstantiation.children[1].children.push(ASTNode {
        token: Token {
            kind: TokenKind::NumValue,
            lexeme: Some(chars.len().to_string()),
        },
        children: Vec::new(),
    });

    // create chars array
    match statement::go(&charinstantiation,
                        class_label,
                        label,
                        fields,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    text.push(format!("{} {}", Instr::PUSH, Reg::ESI));
    text.push("".to_owned());

    // create String object
    let instantiation = STRINGINSTANTIATION.clone();
    match statement::go(&instantiation,
                        class_label,
                        label,
                        fields,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // populate with char array
    text.push(format!("{} {}", Instr::POP, Reg::EDI));
    text.push(format!("{} [{}+4], {}", Instr::MOV, Reg::ESI, Reg::EDI));

    for (idx, ch) in chars.chars().enumerate() {
        text.push(format!("{} dword [{}+4*{}], {:?}",
                          Instr::MOV,
                          Reg::EDI,
                          idx + 1,
                          ch));
    }

    // TODO<codegen>: kind is string
    Ok(None)
}

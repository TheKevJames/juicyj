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
                            token: Token::new(TokenKind::LBrace, None),
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
    let kind = match node.children[0].to_param() {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    let name = match node.children[1].children[0].to_label() {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    text.push(format!("  ; {} {} = x", kind, name));

    let variable = match node.children[1].children[0].to_label() {
        Ok(l) => format!("{}.{}", label, l),
        Err(e) => return Err(e),
    };
    let vkind = match node.children[0].to_label() {
        Ok(k) => k,
        Err(e) => return Err(e),
    };
    bss.push((variable.clone(), vkind.clone()));

    // allocate 4 bytes for lhs
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, "4"));

    text.push(format!("{} {}", Instr::PUSH, Reg::EBX));
    externs.push(format!("{} {}", Instr::EXTERN, "__malloc"));
    text.push(format!("{} {}", Instr::CALL, "__malloc"));
    text.push(format!("{} {}", Instr::POP, Reg::EBX));

    text.push(format!("{} [{}], {}", Instr::MOV, variable, Reg::EAX));
    text.push("".to_owned());

    if kind == "java.lang.String" {
        // implicitly calls constructor
        // TODO<codegen>: figure out the right way to do this
        let mut instantiation = STRINGINSTANTIATION.clone();

        let arg = node.children[1].children[1].clone();
        if arg.token.kind == TokenKind::StrValue {
            let argvalue = arg.token.lexeme.unwrap_or("".to_owned());
            let size = argvalue.len();

            let mut charinstantiation = STRINGARRAYINSTANTIATION.clone();
            charinstantiation.children[1].children.push(ASTNode {
                token: Token {
                    kind: TokenKind::NumValue,
                    lexeme: Some(size.to_string()),
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

            text.push(format!("{} {}", Instr::POP, Reg::EDI));
            text.push(format!("{} [{}+4], {}", Instr::MOV, Reg::ESI, Reg::EDI));

            for (idx, ch) in argvalue.chars().enumerate() {
                text.push(format!("{} dword [{}+4*{}], {:?}",
                                  Instr::MOV,
                                  Reg::EDI,
                                  idx + 1,
                                  ch));
            }
        } else {
            instantiation.children[1].children.push(arg);

            // create String object
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
        }
    } else {
        // resolve rhs and store in lhs
        match statement::go(&node.children[1].children[1],
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
    }

    text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EDI, variable));
    text.push(format!("{} [{}], {}", Instr::MOV, Reg::EDI, Reg::EAX));
    text.push("".to_owned());

    // TODO<codegen>: kind is either lhs kind or null
    Ok(None)
}

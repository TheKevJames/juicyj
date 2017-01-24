use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;

use common::Token;
use common::TokenKind;

pub struct Lexer<'src> {
    current: Option<char>,
    src: Peekable<Chars<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &str) -> Lexer {
        let mut c = src.chars().peekable();
        Lexer {
            current: c.next(),
            src: c,
        }
    }

    fn consume_token(&mut self) {
        self.current = self.src.next();
    }

    fn next_boolean(&mut self) -> Token {
        match self.current {
            Some('&') => {
                self.consume_token();

                if self.current == Some('&') {
                    self.consume_token();
                    return Token {
                        kind: TokenKind::And,
                        lexeme: None,
                    };
                }

                return Token {
                    kind: TokenKind::BitAnd,
                    lexeme: None,
                };
            }
            Some('|') => {
                self.consume_token();

                if self.current == Some('|') {
                    self.consume_token();
                    return Token {
                        kind: TokenKind::Or,
                        lexeme: None,
                    };
                }

                return Token {
                    kind: TokenKind::BitOr,
                    lexeme: None,
                };
            }
            _ => panic!("next_boolean called on illegal value."),
        }
    }

    fn next_comparison(&mut self) -> Token {
        match self.current {
            Some('=') => {
                self.consume_token();

                if self.current == Some('=') {
                    self.consume_token();
                    return Token {
                        kind: TokenKind::Equality,
                        lexeme: None,
                    };
                }

                return Token {
                    kind: TokenKind::Assignment,
                    lexeme: None,
                };
            }
            Some('>') => {
                self.consume_token();

                if self.current == Some('=') {
                    self.consume_token();
                    return Token {
                        kind: TokenKind::GreaterThanOrEqual,
                        lexeme: None,
                    };
                }

                return Token {
                    kind: TokenKind::GreaterThan,
                    lexeme: None,
                };
            }
            Some('<') => {
                self.consume_token();

                if self.current == Some('=') {
                    self.consume_token();
                    return Token {
                        kind: TokenKind::LessThanOrEqual,
                        lexeme: None,
                    };
                }

                return Token {
                    kind: TokenKind::LessThan,
                    lexeme: None,
                };
            }
            Some('!') => {
                self.consume_token();

                if self.current == Some('=') {
                    self.consume_token();
                    return Token {
                        kind: TokenKind::NotEqual,
                        lexeme: None,
                    };
                }

                return Token {
                    kind: TokenKind::Not,
                    lexeme: None,
                };
            }
            _ => panic!("next_comparison called on illegal value."),
        }
    }

    fn next_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(c) = self.current {
            if !c.is_alphabetic() {
                break;
            }

            identifier.push(c);
            self.consume_token();
        }

        let kind = match identifier.as_str() {
            "import" => TokenKind::Import,
            "package" => TokenKind::Package,

            "private" => TokenKind::Private,
            "protected" => TokenKind::Protected,
            "public" => TokenKind::Public,

            "abstract" => TokenKind::Abstract,
            "extends" => TokenKind::Extends,
            "implements" => TokenKind::Implements,
            "interface" => TokenKind::Interface,
            "final" => TokenKind::Final,
            "native" => TokenKind::Native,
            "static" => TokenKind::Static,

            "boolean" => TokenKind::Boolean,
            "byte" => TokenKind::Byte,
            "char" => TokenKind::Char,
            "int" => TokenKind::Int,
            "short" => TokenKind::Short,
            "String" => TokenKind::Str,
            "void" => TokenKind::Void,

            "false" => TokenKind::False,
            "true" => TokenKind::True,

            "class" => TokenKind::Class,
            "delete" => TokenKind::Delete,
            "instanceof" => TokenKind::Instanceof,
            "new" => TokenKind::New,
            "Object" => TokenKind::Object,
            "this" => TokenKind::This,

            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "if" => TokenKind::If,
            "return" => TokenKind::Return,
            "while" => TokenKind::While,

            _ => {
                return Token {
                    kind: TokenKind::Identifier,
                    lexeme: Some(identifier),
                }
            }
        };

        Token {
            kind: kind,
            lexeme: None,
        }
    }

    fn next_char(&mut self) -> Token {
        self.consume_token();

        let mut identifier = String::new();
        while let Some(c) = self.current {
            if c == '\'' {
                self.consume_token();
                break;
            }

            if c == '\\' {
                // TODO
                self.consume_token();
                self.consume_token();
                continue;
            }

            identifier.push(c);
            self.consume_token();
        }

        if identifier.len() != 1 {
            panic!("next_char given multiple characters.")
        }

        Token {
            kind: TokenKind::CharValue,
            lexeme: Some(identifier),
        }
    }

    fn next_number(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(c) = self.current {
            if !c.is_digit(10) {
                break;
            }

            identifier.push(c);
            self.consume_token();
        }

        Token {
            kind: TokenKind::NumValue,
            lexeme: Some(identifier),
        }
    }

    fn next_string(&mut self) -> Token {
        self.consume_token();

        let mut identifier = String::new();
        while let Some(c) = self.current {
            if c == '"' {
                self.consume_token();
                break;
            }

            if c == '\\' {
                // TODO
                self.consume_token();
                self.consume_token();
                continue;
            }

            identifier.push(c);
            self.consume_token();
        }

        Token {
            kind: TokenKind::StrValue,
            lexeme: Some(identifier),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_comments();

        let kind = match self.current {
            Some('{') => TokenKind::LBrace,
            Some('}') => TokenKind::RBrace,
            Some('[') => TokenKind::LBracket,
            Some(']') => TokenKind::RBracket,
            Some('(') => TokenKind::LParen,
            Some(')') => TokenKind::RParen,

            Some('.') => TokenKind::Dot,
            Some('/') => TokenKind::FSlash,
            Some('-') => TokenKind::Minus,
            Some('%') => TokenKind::Percent,
            Some('+') => TokenKind::Plus,
            Some('*') => TokenKind::Star,

            Some(';') => TokenKind::Semicolon,

            Some('&') => return Some(self.next_boolean()),
            Some('|') => return Some(self.next_boolean()),

            Some('=') => return Some(self.next_comparison()),
            Some('<') => return Some(self.next_comparison()),
            Some('>') => return Some(self.next_comparison()),
            Some('!') => return Some(self.next_comparison()),

            Some('\'') => return Some(self.next_char()),
            Some('"') => return Some(self.next_string()),
            Some(digit) if digit.is_digit(10) => return Some(self.next_number()),
            Some(ident) if ident.is_alphabetic() => return Some(self.next_identifier()),

            Some(c) => panic!("unparseable token: {}", c),
            _ => return None,
        };

        self.consume_token();
        Some(Token {
            kind: kind,
            lexeme: None,
        })
    }

    fn skip_comments(&mut self) {
        while let Some(c) = self.current {
            if c.is_whitespace() {
                self.consume_token();
                continue;
            }

            if c == '/' {
                if self.src.peek() == Some(&'*') {
                    self.consume_token();
                    self.consume_token();

                    while let Some(c) = self.current {
                        if c == '*' && self.src.peek() == Some(&'/') {
                            break;
                        }

                        self.consume_token();
                    }

                    self.consume_token();
                    self.consume_token();
                    continue;
                }

                if self.src.peek() == Some(&'/') {
                    while let Some(c) = self.current {
                        if c == '\n' {
                            break;
                        }

                        self.consume_token();
                    }

                    self.consume_token();
                    continue;
                }
            }

            break;
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

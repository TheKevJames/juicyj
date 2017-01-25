use std::iter::Chain;
use std::iter::Iterator;
use std::iter::Peekable;
use std::option::IntoIter;
use std::str::Chars;
use std::str::Split;

use common::Token;
use common::TokenKind;
use lexer::error;
use lexer::identifier;

pub struct Lexer<'file, 'src> {
    current: Option<char>,
    file: &'file str,
    index_character: u32,
    index_line: u32,
    line: Option<&'src str>,
    line_iter: Peekable<Chain<Chars<'src>, IntoIter<char>>>,
    lines: Peekable<Split<'src, char>>,
}

impl<'file, 'src> Lexer<'file, 'src> {
    pub fn new(file: &'file str, src: &'src str) -> Lexer<'file, 'src> {
        let mut lines = src.split('\n').peekable();
        let line = lines.next();
        let mut line_iter = match line {
            Some(l) => l.chars().chain(Some('\n')).peekable(),
            None => "".chars().chain(None).peekable(),
        };

        Lexer {
            current: line_iter.next(),
            file: file,
            index_character: 0,
            index_line: 1,
            line: line,
            line_iter: line_iter,
            lines: lines,
        }
    }

    fn consume_token(&mut self) {
        self.current = match self.line_iter.next() {
            Some(ch) => {
                self.index_character += 1;
                Some(ch)
            }
            None => {
                // skip empty lines
                loop {
                    self.index_line += 1;

                    self.line = self.lines.next();
                    self.line_iter = match self.line {
                        Some("") => continue,
                        Some(line) => line.chars().chain(Some('\n')).peekable(),
                        None => "".chars().chain(None).peekable(),
                    };
                    break;
                }
                self.index_character = 0;

                match self.line_iter.next() {
                    Some(ch) => {
                        self.index_character += 1;
                        Some(ch)
                    }
                    None => None,
                }
            }
        };
    }

    fn peek(&mut self) -> Option<char> {
        match self.line_iter.peek() {
            Some(ch) => Some(*ch),
            None => {
                match self.lines.peek() {
                    Some(line) => line.chars().next(),
                    None => None,
                }
            }
        }
    }

    fn do_ahead(&mut self,
                current_kind: TokenKind,
                ahead_char: char,
                ahead_kind: TokenKind)
                -> Result<Token, error::LexerError> {
        self.consume_token();

        if self.current == Some(ahead_char) {
            self.consume_token();
            return Ok(Token {
                kind: ahead_kind,
                lexeme: None,
            });
        }

        return Ok(Token {
            kind: current_kind,
            lexeme: None,
        });
    }

    fn next_char(&mut self) -> Result<Token, error::LexerError> {
        let mut char_length = 1;

        self.consume_token();

        let mut identifier = String::new();
        while let Some(c) = self.current {
            if c == '\'' {
                self.consume_token();
                break;
            }

            if c == '\n' {
                return Err(error::LexerError {
                    file: self.file.to_owned(),
                    index: self.index_character,
                    line: self.line.unwrap_or("").to_owned(),
                    line_number: self.index_line,
                    message: &"next_char got newline",
                });
            }

            if c == '\\' {
                identifier.push(c);
                self.consume_token();

                match self.current {
                    Some(digit0) if ('0' <= digit0 && digit0 <= '9') => {
                        identifier.push(digit0);
                        self.consume_token();

                        match self.current {
                            Some(digit1) if ('0' <= digit1 && digit1 <= '9') => {
                                identifier.push(digit1);
                                self.consume_token();

                                match self.current {
                                    Some(digit2) if ('0' <= digit2 && digit2 <= '9') => {
                                        identifier.push(digit2);
                                        self.consume_token();

                                        // only \[0-3][0-9][0-9] is valid octal
                                        match digit0 {
                                            '0'...'3' => {
                                                char_length = 4;
                                                continue;
                                            }
                                            _ => {
                                                panic!("next_char got invalid octal {}",
                                                       identifier);
                                            }
                                        }
                                    }
                                    Some('\'') => {
                                        char_length = 3;
                                        self.consume_token();
                                        break;
                                    }
                                    _ => {
                                        panic!("next_char got invalid octal {}",
                                               self.current.unwrap_or('?'))
                                    }
                                }
                            }
                            Some('\'') => {
                                char_length = 2;
                                self.consume_token();
                                break;
                            }
                            _ => {
                                panic!("next_char for invalid octal {}",
                                       self.current.unwrap_or('?'))
                            }
                        }
                    }
                    Some(next) if (next == 't' || next == 'b' || next == 'n' || next == 'r' ||
                                   next == 'f' ||
                                   next == '\'' || next == '"' ||
                                   next == '\\') => {
                        char_length = 2;

                        identifier.push(next);
                        self.consume_token();
                        continue;
                    }
                    _ => {
                        panic!("next_char got invalid escape char {}",
                               self.current.unwrap_or('?'))
                    }
                }
            }

            identifier.push(c);
            self.consume_token();
        }

        if identifier.len() != char_length {
            panic!("next_char given multiple characters.")
        }

        Ok(Token {
            kind: TokenKind::CharValue,
            lexeme: Some(identifier),
        })
    }

    fn next_identifier(&mut self) -> Result<Token, error::LexerError> {
        let mut identifier = String::new();
        while let Some(c) = self.current {
            if !identifier::valid_continuation(c) {
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
                return Ok(Token {
                    kind: TokenKind::Identifier,
                    lexeme: Some(identifier),
                })
            }
        };

        Ok(Token {
            kind: kind,
            lexeme: None,
        })
    }

    fn next_number(&mut self) -> Result<Token, error::LexerError> {
        let mut identifier = String::new();
        while let Some(c) = self.current {
            if !c.is_digit(10) {
                break;
            }

            identifier.push(c);
            self.consume_token();
        }

        Ok(Token {
            kind: TokenKind::NumValue,
            lexeme: Some(identifier),
        })
    }

    fn next_string(&mut self) -> Result<Token, error::LexerError> {
        self.consume_token();

        let mut identifier = String::new();
        while let Some(c) = self.current {
            if c == '"' {
                self.consume_token();
                break;
            }

            if c == '\n' {
                panic!("next_string got newline.")
            }

            if c == '\\' {
                identifier.push(c);
                self.consume_token();

                match self.current {
                    Some(next) if (next == 't' || next == 'b' || next == 'n' || next == 'r' ||
                                   next == 'f' || next == '\\' ||
                                   next == '"' || next == '\'' ||
                                   ('0' <= next && next <= '9')) => {
                        identifier.push(next);
                        self.consume_token();
                        continue;
                    }
                    _ => {
                        panic!("next_string got invalid escape char {}",
                               self.current.unwrap_or('?'))
                    }
                }
            }

            identifier.push(c);
            self.consume_token();
        }

        Ok(Token {
            kind: TokenKind::StrValue,
            lexeme: Some(identifier),
        })
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_comments();

        let token = match self.current {
            Some('{') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::LBrace,
                    lexeme: None,
                })
            }
            Some('}') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::RBrace,
                    lexeme: None,
                })
            }
            Some('[') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::LBracket,
                    lexeme: None,
                })
            }
            Some(']') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::RBracket,
                    lexeme: None,
                })
            }
            Some('(') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::LParen,
                    lexeme: None,
                })
            }
            Some(')') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::RParen,
                    lexeme: None,
                })
            }

            Some('.') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::Dot,
                    lexeme: None,
                })
            }
            Some('/') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::FSlash,
                    lexeme: None,
                })
            }
            Some('-') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::Minus,
                    lexeme: None,
                })
            }
            Some('%') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::Percent,
                    lexeme: None,
                })
            }
            Some('+') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::Plus,
                    lexeme: None,
                })
            }
            Some('*') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::Star,
                    lexeme: None,
                })
            }

            Some(';') => {
                self.consume_token();
                Ok(Token {
                    kind: TokenKind::Semicolon,
                    lexeme: None,
                })
            }

            Some('&') => self.do_ahead(TokenKind::BitAnd, '&', TokenKind::And),
            Some('|') => self.do_ahead(TokenKind::BitOr, '|', TokenKind::Or),
            Some('=') => self.do_ahead(TokenKind::Assignment, '=', TokenKind::Equality),
            Some('<') => self.do_ahead(TokenKind::LessThan, '=', TokenKind::LessThanOrEqual),
            Some('>') => self.do_ahead(TokenKind::GreaterThan, '=', TokenKind::GreaterThanOrEqual),
            Some('!') => self.do_ahead(TokenKind::Not, '=', TokenKind::NotEqual),

            Some('\'') => self.next_char(),
            Some('"') => self.next_string(),
            Some(d) if d.is_digit(10) => self.next_number(),
            Some(c) if identifier::valid_start(c) => self.next_identifier(),

            // TODO: don't lose your blanket
            Some(c) => panic!("unparseable token: {}", c),
            _ => return None,
        };

        let token = match token {
            Ok(k) => k,
            Err(k) => {
                println!("{}", k);
                panic!(k);
            }
        };

        Some(token)
    }

    fn skip_comments(&mut self) {
        while let Some(c) = self.current {
            if c.is_whitespace() {
                self.consume_token();
                continue;
            }

            if c == '/' {
                if self.peek() == Some('*') {
                    self.consume_token();
                    self.consume_token();

                    while let Some(c) = self.current {
                        if c == '*' && self.peek() == Some('/') {
                            break;
                        }

                        self.consume_token();
                    }

                    self.consume_token();
                    self.consume_token();
                    continue;
                }

                if self.peek() == Some('/') {
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

impl<'file, 'src> Iterator for Lexer<'file, 'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

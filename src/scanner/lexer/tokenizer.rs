use std::iter::Chain;
use std::iter::Iterator;
use std::iter::Peekable;
use std::option::IntoIter;
use std::str::Chars;
use std::str::Split;

use scanner::common::error;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::lexer::identifier;

#[derive(Clone)]
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

    fn consume(&mut self) {
        self.current = match self.line_iter.next() {
            Some(ch) if ch == '\t' => {
                self.index_character += 4;
                Some(ch)
            }
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
                    Some(ch) if ch == '\t' => {
                        self.index_character += 4;
                        Some(ch)
                    }
                    Some(ch) => {
                        self.index_character += 1;
                        Some(ch)
                    }
                    None => None,
                }
            }
        };
    }

    fn error(&self, message: error::ErrorMessage) -> error::LexerError {
        error::LexerError {
            file: self.file.to_owned(),
            index: self.index_character,
            line: self.line.unwrap_or("").to_owned(),
            line_number: self.index_line,
            message: message,
        }
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
                aheads_char: Option<Vec<(char, Option<TokenKind>)>>,
                aheads_str: Option<Vec<(&str, Option<TokenKind>)>>)
                -> Result<Token, error::LexerError> {
        self.consume();

        for (ahead_char, ahead_kind) in aheads_char.unwrap_or(Vec::new()) {
            if self.current == Some(ahead_char) {
                self.consume();
                return match ahead_kind {
                    Some(TokenKind::AssignmentAddition) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::AssignmentAnd) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::AssignmentDivision) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::AssignmentModulus) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::AssignmentMultiplication) => {
                        Err(self.error(error::INVALID_TOKEN))
                    }
                    Some(TokenKind::AssignmentOr) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::AssignmentSubtraction) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::AssignmentXor) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::Decrement) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::Increment) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::LShift) => Err(self.error(error::INVALID_TOKEN)),
                    Some(TokenKind::RShift) => Err(self.error(error::INVALID_TOKEN)),
                    Some(kind) => {
                        Ok(Token {
                            kind: kind,
                            lexeme: None,
                        })
                    }
                    None => Err(self.error(error::INVALID_TOKEN)),
                };
            }
        }

        for (ahead_str, ahead_kind) in aheads_str.unwrap_or(Vec::new()) {
            let mut ahead_chars = ahead_str.chars();
            if self.current != ahead_chars.next() {
                continue;
            }
            if self.peek() == ahead_chars.next() {
                continue;
            }
            let last = ahead_chars.next();
            if last != None {
                // This only works because all len-4's have len-3s as prefixes.
                // Itherwise, I'd need to figure out a peek_twice. Ew.
                self.consume();
                if last == self.peek() {
                    continue;
                }
            }

            let num_ahead = if last == None { 2 } else { 3 };
            for _ in 0..num_ahead {
                self.consume();
            }
            return match ahead_kind {
                Some(TokenKind::AssignmentLShift) => Err(self.error(error::INVALID_TOKEN)),
                Some(TokenKind::AssignmentRRShift) => Err(self.error(error::INVALID_TOKEN)),
                Some(TokenKind::AssignmentRShift) => Err(self.error(error::INVALID_TOKEN)),
                Some(TokenKind::RRShift) => Err(self.error(error::INVALID_TOKEN)),
                Some(kind) => {
                    Ok(Token {
                        kind: kind,
                        lexeme: None,
                    })
                }
                None => Err(self.error(error::INVALID_TOKEN)),
            };
        }

        return Ok(Token {
            kind: current_kind,
            lexeme: None,
        });
    }

    fn next_char(&mut self) -> Result<Token, error::LexerError> {
        let mut char_length = 1;

        self.consume();

        let mut identifier = String::new();
        while let Some(c) = self.current {
            if c == '\'' {
                self.consume();
                break;
            }

            if c == '\n' {
                self.consume();
                return Err(self.error(error::CHAR_NEWLINE));
            }

            if c == '\\' {
                identifier.push(c);
                self.consume();

                match self.current {
                    Some(digit0) if ('0' <= digit0 && digit0 <= '7') => {
                        identifier.push(digit0);
                        self.consume();

                        match self.current {
                            Some(digit1) if ('0' <= digit1 && digit1 <= '7') => {
                                identifier.push(digit1);
                                self.consume();

                                match self.current {
                                    Some(digit2) if ('0' <= digit2 && digit2 <= '7') => {
                                        identifier.push(digit2);
                                        self.consume();

                                        // only \[0-3][0-7][0-7] is valid octal
                                        match digit0 {
                                            '0'...'3' => {
                                                char_length = 4;
                                                continue;
                                            }
                                            _ => return Err(self.error(error::INVALID_OCTAL)),
                                        }
                                    }
                                    Some('\'') => {
                                        char_length = 3;
                                        self.consume();
                                        break;
                                    }
                                    _ => {
                                        return Err(self.error(error::CHAR_TOO_LONG_OCTAL));
                                    }
                                }
                            }
                            Some('\'') => {
                                char_length = 2;
                                self.consume();
                                break;
                            }
                            _ => {
                                return Err(self.error(error::CHAR_TOO_LONG_OCTAL));
                            }
                        }
                    }
                    Some(next) if (next == 't' || next == 'b' || next == 'n' || next == 'r' ||
                                   next == 'f' ||
                                   next == '\'' || next == '"' ||
                                   next == '\\') => {
                        char_length = 2;

                        identifier.push(next);
                        self.consume();
                        continue;
                    }
                    _ => return Err(self.error(error::INVALID_ESCAPE)),
                }
            }

            identifier.push(c);
            self.consume();
        }

        if identifier.len() != char_length {
            return Err(self.error(error::CHAR_TOO_LONG));
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
            self.consume();
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
            "void" => TokenKind::Void,

            "false" => TokenKind::False,
            "true" => TokenKind::True,
            "null" => TokenKind::Null,

            // NOTE <1>
            // "Cloneable" => TokenKind::Cloneable,
            // "Integer" => TokenKind::Integer,
            // "Number" => TokenKind::Number,
            // "Object" => TokenKind::Object,
            // "String" => TokenKind::Str,
            "class" => TokenKind::Class,
            "delete" => TokenKind::Delete,
            "instanceof" => TokenKind::Instanceof,
            "new" => TokenKind::New,
            "this" => TokenKind::This,

            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "if" => TokenKind::If,
            "return" => TokenKind::Return,
            "while" => TokenKind::While,

            // see Je_1_Identifiers_Goto.java
            "break" => TokenKind::Break,
            "case" => TokenKind::Case,
            "catch" => TokenKind::Catch,
            "continue" => TokenKind::Continue,
            "default" => TokenKind::Default,
            "do" => TokenKind::Do,
            "double" => TokenKind::Double,
            "finally" => TokenKind::Finally,
            "float" => TokenKind::Float,
            "goto" => TokenKind::Goto,
            "long" => TokenKind::Long,
            "strictfp" => TokenKind::Strictfp,
            "super" => TokenKind::Super,
            "switch" => TokenKind::Switch,
            "synchronized" => TokenKind::Synchronized,
            "throw" => TokenKind::Throw,
            "throws" => TokenKind::Throws,
            "transient" => TokenKind::Transient,
            "try" => TokenKind::Try,
            "volatile" => TokenKind::Volatile,
            _ => {
                return Ok(Token {
                    kind: TokenKind::Identifier,
                    lexeme: Some(identifier),
                })
            }
        };

        // match kind {
        //     // TODO: no reason these can't be variable names
        //     TokenKind::Break => Err(self.error(error::INVALID_TOKEN)),
        //     // etc
        //     kind => {
        //         Ok(Token {
        //             kind: kind,
        //             lexeme: None,
        //         })
        //     }
        // }
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
            self.consume();
        }

        Ok(Token {
            kind: TokenKind::NumValue,
            lexeme: Some(identifier),
        })
    }

    fn next_string(&mut self) -> Result<Token, error::LexerError> {
        self.consume();

        let mut identifier = String::new();
        while let Some(c) = self.current {
            if c == '"' {
                self.consume();
                break;
            }

            if c == '\n' {
                self.consume();
                return Err(self.error(error::STRING_NEWLINE));
            }

            if c == '\\' {
                identifier.push(c);
                self.consume();

                match self.current {
                    Some(next) if (next == 't' || next == 'b' || next == 'n' || next == 'r' ||
                                   next == 'f' || next == '\\' ||
                                   next == '"' || next == '\'' ||
                                   ('0' <= next && next <= '7')) => {
                        identifier.push(next);
                        self.consume();
                        continue;
                    }
                    _ => return Err(self.error(error::INVALID_ESCAPE)),
                }
            }

            identifier.push(c);
            self.consume();
        }

        Ok(Token {
            kind: TokenKind::StrValue,
            lexeme: Some(identifier),
        })
    }

    fn next_token(&mut self) -> Option<Result<Token, error::LexerError>> {
        self.skip_comments();

        let kind = match self.current {
            Some('{') => Some(TokenKind::LBrace),
            Some('}') => Some(TokenKind::RBrace),
            Some('[') => Some(TokenKind::LBracket),
            Some(']') => Some(TokenKind::RBracket),
            Some('(') => Some(TokenKind::LParen),
            Some(')') => Some(TokenKind::RParen),

            Some(',') => Some(TokenKind::Comma),
            Some('.') => Some(TokenKind::Dot),
            Some(';') => Some(TokenKind::Semicolon),

            Some(':') => Some(TokenKind::Colon),
            Some('~') => Some(TokenKind::Complement),
            Some('?') => Some(TokenKind::Question),

            _ => None,
        };
        match kind {
            Some(TokenKind::Colon) => {
                self.consume();
                return Some(Err(self.error(error::INVALID_TOKEN)));
            }
            Some(TokenKind::Complement) => {
                self.consume();
                return Some(Err(self.error(error::INVALID_TOKEN)));
            }
            Some(TokenKind::Question) => {
                self.consume();
                return Some(Err(self.error(error::INVALID_TOKEN)));
            }
            Some(kind) => {
                self.consume();
                return Some(Ok(Token {
                    kind: kind,
                    lexeme: None,
                }));
            }
            _ => {}
        }

        match self.current {
            Some('/') => {
                Some(self.do_ahead(TokenKind::FSlash,
                                   Some(vec![('=', Some(TokenKind::AssignmentDivision))]),
                                   None))
            }
            Some('-') => {
                Some(self.do_ahead(TokenKind::Minus,
                                   Some(vec![('=', Some(TokenKind::AssignmentSubtraction)),
                                             ('-', Some(TokenKind::Decrement))]),
                                   None))
            }
            Some('%') => {
                Some(self.do_ahead(TokenKind::Percent,
                                   Some(vec![('=', Some(TokenKind::AssignmentModulus))]),
                                   None))
            }
            Some('+') => {
                Some(self.do_ahead(TokenKind::Plus,
                                   Some(vec![('=', Some(TokenKind::AssignmentAddition)),
                                             ('+', Some(TokenKind::Increment))]),
                                   None))
            }
            Some('*') => {
                Some(self.do_ahead(TokenKind::Star,
                                   Some(vec![('=', Some(TokenKind::AssignmentMultiplication))]),
                                   None))
            }

            Some('&') => {
                Some(self.do_ahead(TokenKind::BitAnd,
                                   Some(vec![('&', Some(TokenKind::And)),
                                             ('=', Some(TokenKind::AssignmentAnd))]),
                                   None))
            }
            Some('|') => {
                Some(self.do_ahead(TokenKind::BitOr,
                                   Some(vec![('|', Some(TokenKind::Or)),
                                             ('=', Some(TokenKind::AssignmentOr))]),
                                   None))
            }
            Some('^') => {
                Some(self.do_ahead(TokenKind::BitXor,
                                   Some(vec![('=', Some(TokenKind::AssignmentXor))]),
                                   None))
            }

            Some('=') => {
                Some(self.do_ahead(TokenKind::Assignment,
                                   Some(vec![('=', Some(TokenKind::Equality))]),
                                   None))
            }
            Some('<') => {
                Some(self.do_ahead(TokenKind::LessThan,
                                   Some(vec![('=', Some(TokenKind::LessThanOrEqual)),
                                             ('<', Some(TokenKind::LShift))]),
                                   Some(vec![("<=", Some(TokenKind::AssignmentLShift))])))
            }
            Some('>') => {
                Some(self.do_ahead(TokenKind::GreaterThan,
                                   Some(vec![('=', Some(TokenKind::GreaterThanOrEqual)),
                                             ('>', Some(TokenKind::RShift))]),
                                   Some(vec![(">=", Some(TokenKind::AssignmentRShift)),
                                             (">>=", Some(TokenKind::AssignmentRRShift)),
                                             (">>", Some(TokenKind::RRShift))])))
            }
            Some('!') => {
                Some(self.do_ahead(TokenKind::Not,
                                   Some(vec![('=', Some(TokenKind::NotEqual))]),
                                   None))
            }

            Some('\'') => Some(self.next_char()),
            Some('"') => Some(self.next_string()),
            Some(d) if d.is_digit(10) => Some(self.next_number()),
            Some(c) if identifier::valid_start(c) => Some(self.next_identifier()),

            Some(_) => {
                self.consume();
                Some(Err(self.error(error::INVALID_TOKEN)))
            }
            _ => None,
        }
    }

    fn skip_comments(&mut self) {
        while let Some(c) = self.current {
            if c.is_whitespace() {
                self.consume();
                continue;
            }

            if c == '/' {
                if self.peek() == Some('*') {
                    self.consume();
                    self.consume();

                    while let Some(c) = self.current {
                        if c == '*' && self.peek() == Some('/') {
                            break;
                        }

                        self.consume();
                    }

                    self.consume();
                    self.consume();
                    continue;
                }

                if self.peek() == Some('/') {
                    while let Some(c) = self.current {
                        if c == '\n' {
                            break;
                        }

                        self.consume();
                    }

                    self.consume();
                    continue;
                }
            }

            break;
        }
    }
}

impl<'file, 'src> Iterator for Lexer<'file, 'src> {
    type Item = Result<Token, error::LexerError>;

    fn next(&mut self) -> Option<Result<Token, error::LexerError>> {
        self.next_token()
    }
}

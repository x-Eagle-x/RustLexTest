#![allow(dead_code)]
#![allow(unused_assignments)]

const BAD_LEXING: i32 = 0x01;

pub enum TokenKind {
    Dummy,

    NumLiteral,
    StrLiteral,
    Identifier,

    OperatorPlus,
    OperatorMinus,
    OperatorDivide,
    OperatorMultiply,

    KwFunction,
    KwVariable
}

pub struct Token {
    raw: String,
    kind: TokenKind
}

pub struct Lexer {
    index: usize,
    file_index: usize,
    input: Vec<(String, String)>,

    line: usize,
    position: usize,

    pub tokens: Vec<Token>
}

fn is_identifier(what: char) -> bool {
    what.is_alphanumeric() || what == '_'
}

fn literal_can_proceed(what: char) -> bool {
    match what {
        ' ' | '\n' | '\t' | '\0' => true,
        '+' | '-' | '/' | '*' => true,
        _ => false
    }
}

fn match_identifier(identifier: String) -> TokenKind {
    match identifier.as_str() {
        "fn" => TokenKind::KwFunction,
        "var" => TokenKind::KwVariable,
        _ => TokenKind::Identifier
    }
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {index: 0, file_index: 0, input: vec![], line: 1, position: 1, tokens: vec![]}
    }

    fn error(&self, message: &str) {
        eprintln!("Lexing error (F: {} L: {}, C: {}): {}.", self.input[self.file_index].0, self.line, self.position, message);
        std::process::exit(BAD_LEXING);
    }

    fn get_token(&mut self) -> Token {
        let mut kind: TokenKind = TokenKind::Dummy;
        let char = self.current_char();

        match char {
            '+' => kind = TokenKind::OperatorPlus,
            '-' => kind = TokenKind::OperatorMinus,
            '/' => kind = TokenKind::OperatorDivide,
            '*' => kind = TokenKind::OperatorMultiply,

            '0'..='9' => {
                kind = TokenKind::NumLiteral;
                let mut raw_number: String = char.to_string();

                while self.temp_peek_char(1).is_numeric() {
                    raw_number.push(self.peek_char(1));
                }

                if !literal_can_proceed(self.temp_peek_char(1)) {
                    self.error("illegal character(s) found after number literal");
                }

                return Token {raw: raw_number, kind: kind};
            },

            '"' => {
                kind = TokenKind::StrLiteral;
                let mut string_literal: String = String::default();

                while self.temp_peek_char(1) != '"' {
                    match self.temp_peek_char(1) {
                        '\t' => self.error("use '\\t' instead of raw character"),
                        '\n' => self.error("missing terminating character (use '\\n' instead of raw character)"),
                        '\0' => self.error("missing terminating character"),
                        '\\' => {
                            string_literal.push(match self.peek_char(2) {
                                'r' => '\r',
                                'n' => '\n',
                                't' => '\t',
                                '\\' => '\\',

                                ' ' | '\t' | '\n' | '\0' => {self.error("missing escape sequence"); '\0'},
                                _ => {self.error("unrecognizeable escape sequence"); '\0' /* dummy value */}
                            });
                        }
                        _ => string_literal.push(self.peek_char(1))
                    }
                }
                self.peek_char(1);

                if !literal_can_proceed(self.temp_peek_char(1)) {
                    self.error("illegal character(s) found after string literal");
                }

                return Token {raw: string_literal, kind: TokenKind::StrLiteral};
            }

            '_' | 'a'..='z' | 'A'..='Z' => {
                let mut raw_token: String = char.to_string();
                while is_identifier(self.temp_peek_char(1)) {
                    raw_token.push(self.peek_char(1));
                }

                if !literal_can_proceed(self.temp_peek_char(1)) {
                    self.error("illegal character(s) found after identifier");
                }

                kind = match_identifier(raw_token.clone());
                return Token {raw: raw_token, kind: kind};
            }

            _ => self.error("unrecognizeable token"),
        }

        Token {raw: char.to_string(), kind: kind}
    }

    fn peek_char(&mut self, peek: usize) -> char {
        if self.index + peek >= self.input[self.file_index].1.len() {
            return '\0';
        }
        self.index += peek;
        self.position += 1;
        self.input[self.file_index].1.as_bytes()[self.index] as char
    }

    fn temp_peek_char(&self, peek: usize) -> char {
        if self.index + peek >= self.input[self.file_index].1.len() {
            return '\0';
        }
        self.input[self.file_index].1.as_bytes()[self.index + peek] as char
    }

    fn current_char(&self) -> char {
        self.input[self.file_index].1.as_bytes()[self.index] as char
    }

    pub fn lex(&mut self) {
        for _ in self.input.clone() {
            self.index = 0;

            while self.index < self.input[self.file_index].1.len() {
                if self.current_char() == '\n' {
                    self.line += 1;
                    self.position = 1;
                }
                else if !self.current_char().is_whitespace() {
                    let token = self.get_token();
                    self.tokens.push(token);
                }
                
                self.position += 1;
                self.index += 1;
            }

            self.file_index += 1;
        }
    }

    pub fn feed(&mut self, input: String) {
        self.input[self.file_index].1.push_str(input.as_str());
    }

    pub fn feed_file(&mut self, input: (String, String)) {
        self.input.push(input);
    }
}

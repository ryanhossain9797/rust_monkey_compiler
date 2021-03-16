use std::char;

use crate::token::Token;

// static KEYWORDS: HashMap<&'static str, Token> = HashMap::new();

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.as_bytes()[self.read_position] as char);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.read_position] as char)
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        use self::Token::*;
        let token = match self.ch {
            Some(token) => match token {
                '0'..='9' => INT(token.to_string().parse::<i32>().unwrap()),
                '=' => match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        EQ
                    }
                    _ => ASSIGN,
                },
                ';' => SEMICOLON,
                ',' => COMMA,
                '(' => LPAREN,
                ')' => RPAREN,
                '{' => LBRACE,
                '}' => RBRACE,
                '+' => PLUS,
                '-' => MINUS,
                '*' => ASTERISK,
                '/' => SLASH,
                '!' => match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        NOTEQ
                    }
                    _ => BANG,
                },
                '<' => LT,
                '>' => GT,
                _ => {
                    if self.is_letter() {
                        self.read_identifier()
                    } else {
                        ILLEGAL
                    }
                }
            },
            None => EOF,
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            let val = ch as u8;
            if [b' ', b'\n', b'\t', b'\r'].contains(&val) {
                self.read_char();
            } else {
                break;
            }
        }
    }

    //identifier or keyword
    fn is_letter(&self) -> bool {
        let val = self.ch.unwrap() as u8;
        (b'a'..=b'z').contains(&val) || (b'A'..=b'Z').contains(&val) || val == b'_'
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while self.ch.is_some() {
            if self.is_letter() {
                self.read_char();
            } else {
                break;
            }
        }
        self.keyword_or_ident(position)
    }

    fn keyword_or_ident(&mut self, from: usize) -> Token {
        use self::Token::{ELSE, FALSE, FUNCTION, IDENT, IF, LET, RETURN, TRUE};
        match &self.input[from..self.position] {
            "fn" => FUNCTION,
            "let" => LET,
            "true" => TRUE,
            "false" => FALSE,
            "if" => IF,
            "else" => ELSE,
            "return" => RETURN,
            ident => IDENT(ident.to_string()),
        }
    }
}

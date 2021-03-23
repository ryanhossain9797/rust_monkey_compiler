#![allow(clippy::upper_case_acronyms)]

use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String), // add, foobar, x, y, ...
    INT(i32),      // 1343456

    // Operators
    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    ASTERISK, // *
    SLASH,    // /
    BANG,     // !
    LT,       // <
    GT,       // >
    EQ,       // ==
    NOTEQ,    // !=

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::ILLEGAL => "ILLEGAL".to_string(),
                Token::EOF => "EOF".to_string(),
                Token::IDENT(ident) => format!("IDENT {}", ident),
                Token::INT(integer) => format!("INT {}", integer),
                Token::ASSIGN => "=".to_string(),
                Token::PLUS => "+".to_string(),
                Token::MINUS => "-".to_string(),
                Token::ASTERISK => "*".to_string(),
                Token::SLASH => "/".to_string(),
                Token::BANG => "!".to_string(),
                Token::LT => "<".to_string(),
                Token::GT => ">".to_string(),
                Token::EQ => "==".to_string(),
                Token::NOTEQ => "!=".to_string(),
                Token::COMMA => ",".to_string(),
                Token::SEMICOLON => ";".to_string(),
                Token::LPAREN => "(".to_string(),
                Token::RPAREN => ")".to_string(),
                Token::LBRACE => "{".to_string(),
                Token::RBRACE => "}".to_string(),
                Token::FUNCTION => "FUNCTION".to_string(),
                Token::LET => "LET".to_string(),
                Token::TRUE => "TRUE".to_string(),
                Token::FALSE => "FALSE".to_string(),
                Token::IF => "IF".to_string(),
                Token::ELSE => "ELSE".to_string(),
                Token::RETURN => "RETURN".to_string(),
            }
        )
    }
}

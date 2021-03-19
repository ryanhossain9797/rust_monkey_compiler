use super::*;

pub struct Parser {
    l: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

use super::token::*;
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Expression for Identifier {}

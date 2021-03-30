use super::*;
pub struct Parser {
    l: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(l: lexer::Lexer) -> Self {
        let mut parser = Parser {
            l,
            cur_token: token::Token::ILLEGAL,
            peek_token: token::Token::ILLEGAL,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.cur_token != token::Token::EOF {
            println!("parsing statement");
            let maybe_statement = self.parse_statement();
            if let Some(statement) = maybe_statement {
                println!("got a statement");
                program.statements.push(statement);
            }
            self.next_token();
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match &self.cur_token {
            Token::LET => {
                println!("got let statement token");
                match self.parse_let_statement() {
                    Some(statement) => Some(Box::new(statement)),
                    None => None,
                }
            }
            token => {
                println!("got token: {}", token.to_string());
                None
            }
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let ident_token = self.cur_token.clone();

        if !self.expect_peek(Token::IDENT("".to_string())) {
            return None;
        }

        let name_token = self.cur_token.clone();

        if !self.expect_peek(Token::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(Token::SEMICOLON) {
            self.next_token();
        }
        Some(LetStatement {
            token: ident_token,
            name: Identifier {
                token: name_token.clone(),
                value: match name_token {
                    Token::IDENT(x) => x,
                    _ => return None,
                },
            },
            value: None,
        })
    }

    pub fn cur_token_is(&self, token: Token) -> bool {
        match (&self.cur_token, &token) {
            (Token::IDENT(_), Token::IDENT(_)) => true,
            (Token::INT(_), Token::INT(_)) => true,
            (cur_token, token) => cur_token == token,
        }
    }

    pub fn peek_token_is(&self, token: Token) -> bool {
        match (&self.peek_token, &token) {
            (Token::IDENT(_), Token::IDENT(_)) => true,
            (Token::INT(_), Token::INT(_)) => true,
            (peek_token, token) => peek_token == token,
        }
    }

    pub fn expect_peek(&mut self, token: Token) -> bool {
        match self.peek_token_is(token) {
            true => {
                self.next_token();
                true
            }
            false => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        let input = "let x = 5;
        let y = 10;
        let foobar = 838383;"
            .to_string();

        let l = Lexer::new(input);
        let mut parser = Parser::new(l);

        let program = parser.parse_program();
        if program.statements.len() != 3 {
            panic!(program.statements.len());
        } else {
            let tests = ["x", "y", "foobar"];

            for (index, test) in tests.iter().enumerate() {
                let statement = &program.statements[index];
                test_let_statement(statement, test)
            }
        }
    }

    fn test_let_statement(statement: &Box<dyn Statement>, expected_identifier: &str) {
        if &statement.token_literal() != "LET" {
            panic!("token literal is not let, {}", statement.token_literal());
        }
        let let_statement = match statement.as_any().downcast_ref::<LetStatement>() {
            Some(statement) => statement,
            None => panic!("Not a let statement"),
        };

        if (&let_statement.name.value != expected_identifier) {
            panic!(
                "expected identifier doesn't match {}, {}",
                let_statement.name.value, expected_identifier
            )
        }

        if let_statement.name.value != expected_identifier {
            panic!(
                "statement name not {}, got {}",
                expected_identifier,
                let_statement.token_literal()
            )
        }
    }
}

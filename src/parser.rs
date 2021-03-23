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
            let maybe_statement = self.parse_statement();
            if let Some(statement) = maybe_statement {
                program.statements.push(statement);
            }
            self.next_token();
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token {
            Token::LET => None, //self.parse_let_statement(), RESUME HERE
            _ => None,
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
            panic!(
                "program has incorrect number of statements: {}",
                program.statements.len()
            );
        } else {
            let tests = ["x", "y", "foobar"];

            for (index, test) in tests.iter().enumerate() {
                let statement = &program.statements[index];
                assert!(test_statement(statement, test))
            }
        }

        panic!();
    }
    fn test_statement(statement: &Box<dyn Statement>, expected_identifier: &str) -> bool {
        true
    }
}

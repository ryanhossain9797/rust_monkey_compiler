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

    pub fn parse_program(&self) -> Option<Program> {
        None
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
        let parser = Parser::new(l);

        let maybe_program = parser.parse_program();

        match maybe_program {
            None => panic!("program returned null"),
            Some(program) => {
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
            }
        }

        panic!();
    }
    fn test_statement(statement: &Box<dyn Statement>, expected_identifier: &str) -> bool {
        true
    }
}

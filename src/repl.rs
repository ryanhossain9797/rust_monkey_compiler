use super::{Lexer, Token};
use std::io::{Read, Stdin, Stdout, Write};

const PROMPT: &str = ">>";

pub async fn start(input: Stdin, mut output: Stdout) -> anyhow::Result<()> {
    print!("{} ", PROMPT);
    output.flush()?;

    let mut code = String::new();

    input.read_line(&mut code)?;

    let mut lexer = Lexer::new(code);

    loop {
        let token = lexer.next_token();
        match token {
            Token::EOF => return Ok(()),
            token => {
                println!("{}", token);
            }
        }
    }
}

mod lexer;
mod repl;
mod token;

use lexer::*;
use repl::*;
use token::*;

#[async_std::main]
async fn main() {
    let mut lexer = lexer::Lexer::new(
        "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x * y;
        };
        
        <!5>

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;

        let result = add(five, ten);"
            .to_string(),
    );

    loop {
        let token = lexer.next_token();
        match token {
            Token::EOF => {
                break;
            }
            token => {
                println!("{}", token);
            }
        }
    }
}

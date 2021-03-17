mod lexer;
mod repl;
mod token;

use lexer::*;
use repl::*;
use std::io::{stdin, stdout};
use token::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    println!("Type in commands\nGo to new line and press CTRL-D to run");

    start(stdin(), stdout()).await
}

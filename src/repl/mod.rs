use crate::{lexer::Lexer, token::Token};
use std::{
    error::Error,
    io::{self, Write},
};

const PROMPT: &str = ">>";

pub fn start() -> Result<(), Box<dyn Error>> {
    io::stdin().lines().for_each(|line| {
        print!("{}", PROMPT);
        io::stdout().flush().expect("it should've been flushed");
        if let Ok(part) = line {
            let lex = Lexer::new(part.into());
            for tok in lex {
                println!("{:?}", tok);
                if tok == Token::EOF {
                    break;
                }
            }
        }
    });
    Ok(())
}

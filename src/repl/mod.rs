use crate::{lexer::Lexer, token::Token};
use std::{
    io::{self, Write},
    rc::Rc,
};

const PROMPT: &str = ">>";

pub fn start() {
    let mut input = String::new();
    loop {
        print!("{} ", PROMPT);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.is_empty() {
                    break;
                }
                let lex = Lexer::new(Rc::from(input.as_str()));
                for tok in lex {
                    if tok != Token::EOF {
                        println!("{:?}", tok);
                    }
                }
                input.clear();
            }
            Err(_) => break,
        }
    }
}

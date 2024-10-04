use crate::lexer::Lexer;
use std::{error::Error, io, rc::Rc};

const PROMPT: &str = ">>";

pub fn start() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", PROMPT);
    io::stdin().read_line(&mut input)?;
    while !input.is_empty() {
        print!("{}", PROMPT);
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let lex = Lexer::new(Rc::from(input));
        for tok in lex {
            println!("{:?}", tok);
        }
    }
    Ok(())
}

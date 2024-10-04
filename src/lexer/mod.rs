use core::str;
use std::rc::Rc;
mod tests;

use super::token::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lexer {
    input: Rc<str>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ch != 0 {
            Some(self.next_token())
        } else {
            None
        }
    }
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(input: Rc<str>) -> Self {
        let mut lex = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lex.read_char();
        return lex;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn lookup_ident(&mut self) -> Token {
        return match self.read_identifier() {
            "let" => Token::LET,
            "fn" => Token::FUNCTION,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
            other => Token::IDENT(String::from(other)),
        };
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEQ
                } else {
                    Token::BANG
                }
            }
            b'/' => Token::SLASH,
            b'*' => Token::ASTERISK,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0 => Token::EOF,
            other => {
                if self.ch.is_ascii_alphabetic() {
                    return self.lookup_ident();
                } else if self.ch.is_ascii_digit() {
                    return Token::INT(self.read_number());
                } else {
                    Token::ILLEGAL(other)
                }
            }
        };
        self.read_char();
        return tok;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from(&self.input[position..self.position]);
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        return &self.input[position..self.position];
    }
}

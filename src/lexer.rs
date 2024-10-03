use core::str;

use super::token::Token;

#[allow(dead_code)]
#[derive(Debug)]
struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

#[allow(dead_code)]
impl Lexer {
    fn new(input: Vec<u8>) -> Self {
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
            self.ch = self.input[self.read_position];
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
        let ident = self.read_identifier();
        return match ident {
            "let" => Token::LET,
            "fn" => Token::FUNCTION,
            other => Token::IDENT(other),
        };
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'=' => Token::ASSIGN,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'!' => Token::BANG,
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

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let input = &self.input[position..self.position];
        return str::from_utf8(input).expect("string from u8 to succeed");
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        let input = &self.input[position..self.position];
        return str::from_utf8(input).expect("string from u8 to succeed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let code = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
        "#;
        let input = code.bytes().collect();
        let tests = [
            Token::LET,
            Token::IDENT("five"),
            Token::ASSIGN,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten"),
            Token::ASSIGN,
            Token::INT("10"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add"),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x"),
            Token::COMMA,
            Token::IDENT("y"),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x"),
            Token::PLUS,
            Token::IDENT("y"),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result"),
            Token::ASSIGN,
            Token::IDENT("add"),
            Token::LPAREN,
            Token::IDENT("five"),
            Token::COMMA,
            Token::IDENT("ten"),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::INT("5"),
            Token::LT,
            Token::INT("10"),
            Token::GT,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lex = Lexer::new(input);
        for test in tests {
            let tok = lex.next_token();
            assert_eq!(tok, test);
        }
    }
}

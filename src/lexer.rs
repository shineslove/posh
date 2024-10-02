use super::token::Token;

#[allow(dead_code)]
#[derive(Debug)]
struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

#[allow(dead_code)]
impl Lexer {
    fn new(input: String) -> Self {
        return Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position)
                .expect("should be here")
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
    fn lookup_ident(&mut self) -> Token {
        return match self.read_identifier() {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            other => Token::IDENT(other),
        };
    }
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        return match self.ch {
            '=' => Token::ASSIGN,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '+' => Token::PLUS,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '\0' => Token::EOF,
            other => {
                if self.is_letter() {
                    return self.lookup_ident();
                } else {
                    Token::ILLEGAL(other)
                }
            }
        };
    }
    fn is_letter(&mut self) -> bool {
        return 'a' <= self.ch && self.ch <= 'z'
            || 'A' <= self.ch && self.ch <= 'Z'
            || self.ch == '_';
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        return &self.input[position..self.position];
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
        "#;
        let input = String::from(code);
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
            Token::IDENT("fn"),
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
            Token::LBRACE,
            Token::IDENT("five"),
            Token::COMMA,
            Token::IDENT("ten"),
            Token::RPAREN,
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

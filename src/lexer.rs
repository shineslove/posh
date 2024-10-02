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
    fn next_token(&mut self) -> Token {
        self.read_char();
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
            _ => Token::ILLEGAL(self.ch),
        };
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
        ];
        let mut lex = Lexer::new(input);
        for test in tests {
            let tok = lex.next_token();
            assert_eq!(tok, test);
        }
    }
}

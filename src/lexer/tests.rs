#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
               return true; 
            } else {
               return false;
            }
            10 == 10;
            10 != 9;
        "#;
        let tests = [
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::INT(String::from("5")),
            Token::LT,
            Token::INT(String::from("10")),
            Token::GT,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(String::from("5")),
            Token::LT,
            Token::INT(String::from("10")),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(String::from("10")),
            Token::EQ,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::INT(String::from("10")),
            Token::NotEQ,
            Token::INT(String::from("9")),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lex = Lexer::new(Rc::from(input));
        for test in tests {
            let tok = lex.next_token();
            assert_eq!(tok, test);
        }
    }
}

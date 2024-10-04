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
            Token::IDENT("five".into()),
            Token::ASSIGN,
            Token::INT("5".into()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".into()),
            Token::ASSIGN,
            Token::INT("10".into()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".into()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".into()),
            Token::COMMA,
            Token::IDENT("y".into()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".into()),
            Token::PLUS,
            Token::IDENT("y".into()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".into()),
            Token::ASSIGN,
            Token::IDENT("add".into()),
            Token::LPAREN,
            Token::IDENT("five".into()),
            Token::COMMA,
            Token::IDENT("ten".into()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".into()),
            Token::SEMICOLON,
            Token::INT("5".into()),
            Token::LT,
            Token::INT("10".into()),
            Token::GT,
            Token::INT("5".into()),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5".into()),
            Token::LT,
            Token::INT("10".into()),
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
            Token::INT("10".into()),
            Token::EQ,
            Token::INT("10".into()),
            Token::SEMICOLON,
            Token::INT("10".into()),
            Token::NotEQ,
            Token::INT("9".into()),
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

#[derive(Debug, PartialEq)]
pub enum Token {
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    ILLEGAL(char),
    IDENT(char),
    INT(char),
    EOF,
}

#[derive(Debug)]
pub enum Token<'a> {
    ILLEGAL(&'a str),
    IDENT(&'a str),
    INT(&'a str),
    EOF,
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
}

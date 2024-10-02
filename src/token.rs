#[derive(Debug, PartialEq)]
pub enum Token<'t> {
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
    IDENT(&'t str),
    INT(&'t str),
    EOF,
}

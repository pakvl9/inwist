use crate::base::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    ID,
    INT,
    FLOAT,
    STR,
    BOOL,
    NONE,

    LET,
    FUNC,
    STRUCT,

    IF,
    ELSE,
    SWITCH,
    CASE,
    AND,
    OR,
    NOT,
    FOR,
    LOOP,
    BREAK,
    WHILE,
    GOTO,
    
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERSENT,
    COLON,
    EQ,
    EQEQ,
    NOTEQ,
    SEMI,
    DOT,
    LT,
    GT,
    LTEQ,
    GTEQ,

    LPAR,
    RPAR,
    LBRACE,
    RBRACE,
    LBRAKET,
    RBRAKET,

    EOF,
    UNK,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub typet: TokenType,
    pub span:  Span,
}
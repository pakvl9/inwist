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
    AND,
    OR,
    NOT,
    FOR,
    LOOP,
    WHILE,
    
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
    LBR,
    RBR,

    EOF,
    UNK,
}

pub struct Pos {
    pub file_id: u32,
    pub offset:  u32,
}

pub struct Span {
    pub start: Pos,
    pub end:   Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub typet: TokenType,
    pub span:  Span,
}
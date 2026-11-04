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
    
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERSENT,
    COLON,
    EQ,
    SEMI,
    DOT,

    LPAR,
    RPAR,
    LBR,
    RBR,

    EOF,
    UNC,
}

pub struct Token {
    pub typet: TokenType,
    pub len: usize,
    pub line: i32,
    pub col: i32,
    pub data: &'a str,
    // Срез строки, как вариант сырой указатель data: *const u8
}
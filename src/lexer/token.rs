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
    UNC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub typet: TokenType,
    // длина не нужна str уже имеет длину
    // pub len: usize,
    pub line: i32,
    pub col:  i32,
    pub data: &'a str,
    // Срез строки, как вариант сырой указатель data: *const u8
}
use super::token::*;
use super::file_reg::*;
use std::collections::HashMap;

pub struct Lexer {
    file:   File,
    pos:    usize,
    line:   usize,
    col:    usize,

    keyws:  HashMap<&'static str, TokenType>,
    ops:    HashMap<&'static str, TokenType>,
    opsdbl: HashMap<&'static str, TokenType>,
}

impl Lexer {
    pub fn lex_init(file: File) -> Self {
        let keyws = HashMap::from([
            ("let",    TokenType::LET),
            ("func",   TokenType::FUNC),
            ("struct", TokenType::STRUCT),
            ("if",     TokenType::IF),
            ("else",   TokenType::ELSE),
            ("switch", TokenType::SWITCH),
            ("case",   TokenType::CASE),
            ("and",    TokenType::AND),
            ("or",     TokenType::OR),
            ("true",   TokenType::BOOL),
            ("false",  TokenType::BOOL),
            ("for",    TokenType::FOR),
            ("loop",   TokenType::LOOP),
            ("goto",   TokenType::GOTO),
            ("break",  TokenType::BREAK),
            ("while",  TokenType::WHILE),
            ("none",   TokenType::NONE),
        ]);

        let ops = HashMap::from([
            ("!", TokenType::NOT),
            ("+", TokenType::PLUS),
            ("-", TokenType::MINUS),
            ("*", TokenType::STAR),
            ("/", TokenType::SLASH),
            ("%", TokenType::PERSENT),
            (":", TokenType::COLON),
            ("=", TokenType::EQ),
            ("<", TokenType::LT),
            (">", TokenType::GT),
            (";", TokenType::SEMI),
            (".", TokenType::DOT),
            ("(", TokenType::LPAR),
            (")", TokenType::RPAR),
            ("{", TokenType::LBRACE),
            ("}", TokenType::RBRACE),
            ("[", TokenType::LBRAKET),
            ("]", TokenType::RBRAKET),
        ]);

        let opsdbl = HashMap::from([
            ("==", TokenType::EQEQ),
            ("!=", TokenType::NOTEQ),
            ("<=", TokenType::LTEQ),
            (">=", TokenType::GTEQ),
        ]);

        Self {
            file: file,
            pos:  0,
            line: 1,
            col:  1,

            keyws,
            ops,
            opsdbl,
        }
    }

    fn peek(&self) -> Option<char>{
        self.file.src()[self.pos..].chars().next()
    }
    fn peek_nth(&self, n: usize) -> Option<char> {
        self.file.src()[self.pos..].chars().nth(n)
    }
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();

        if ch == '\n' {
            self.line += 1;
            self.col  =  1;
        } else {
            self.col  += 1;
        }

        Some(ch)
    }
    fn rest(&self) -> &'a str {
        &self.file.src()[self.pos..]
    }

    fn skip_ign(&mut self) -> Result<(), String> {
        loop {
            while self.cursor.peek().is_some_and(|c| c.is_whitespace()) {
                self.cursor.advance();
            }

            if self.cursor.rest().starts_with("//") {
                while let Some(c) = self.cursor.peek() {
                    if c == '\n' {
                        break
                    }

                    self.cursor.advance();
                }

                continue;
            }

            if self.cursor.rest().starts_with("/*") {
                let l_st = self.cursor.line;
                let c_st = self.cursor.col;

                self.cursor.advance();self.cursor.advance();

                while !self.cursor.rest().starts_with("*/") {
                    if self.cursor.advance().is_none() {
                        return Err(format!("Unclosed comment at {l_st}:{c_st}"));
                    }
                }

                self.cursor.advance();self.cursor.advance();

                continue;
            }
            
            break
        }

        Ok(())
    }

    pub fn lex_next(&mut self) -> Result<Token<'a>, String> {
        self.skip_ign()?;
    }
}
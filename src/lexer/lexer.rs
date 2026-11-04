use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Lexer<'a> {
    rest: &'a str,
    // string_view только оставшейся части файла 
    line: usize,
    col:  usize,

    keyws: HashMap<&'static str, TokenType>,
    ops: HashMap<&'static str, TokenType>,
    opsdbl: HashMap<&'static str, TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn lex_init(data: &'a str) -> Self {
        let keyws = HashMap::from([
            ("let",    TokenType::LET),
            ("func",   TokenType::FUNC),
            ("struct", TokenType::STRUCT),
            ("if",     TokenType::IF),
            ("else",   TokenType::ELSE),
            ("and",    TokenType::AND),
            ("or",     TokenType::OR),
            ("true",   TokenType::BOOL),
            ("false",  TokenType::BOOL),
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
            ("{", TokenType::LBR),
            ("}", TokenType::RBR),
        ]);

        let opsdbl = HashMap::from([
            ("==", TokenType::EQEQ),
            ("!=", TokenType::NOTEQ),
            ("<=", TokenType::LTEQ),
            (">=", TokenType::GTEQ),
        ]);

        Self {
            rest: data,
            line: 1,
            col:  1,

            keyws,
            ops,
            opsdbl,
        }
    }

    fn peek(&self) -> Option<char>{
        self.rest.chars().next()
    }
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;

        self.rest = &self.rest[ch.len_utf8()..];
        // перемещение начала среза на длину символа в байтах utf8

        if ch == '\n' {
            self.line += 1;
            self.col  =  1;
        } else {
            self.col  += 1;
        }

        Some(ch)
    }

    fn skip_ign(&mut self) -> Result<(), String> {
        loop {
            while self.peek().is_some_and(|c| c.is_whitespace()) {
                self.advance();
            }

            if self.rest.starts_with("//") {
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break
                    }

                    self.advance();
                }

                continue;
            }

            if self.rest.starts_with("/*") {
                let l_st = self.line;
                let c_st = self.col;

                self.advance();self.advance();

                while !self.rest.starts_with("*/") {
                    if self.advance().is_none() {
                        return Err(format!("Unclosed comment at {l_st}:{c_st}"));
                    }
                }

                self.advance();self.advance();

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
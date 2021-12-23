use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
}

#[allow(dead_code)]
pub struct Scanner {
    input: BufReader<File>,
    read_position: u64,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new(input: BufReader<File>) -> Scanner {
        Scanner {
            input,
            read_position: 0,
        }
    }
    fn get_token(ch: char) -> Option<Token> {
        match ch {
            '{' => Some(Token::LBRACE),
            '}' => Some(Token::RBRACE),
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            _ => None,
        }
    }
    pub fn scan(mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut line = String::new();
        while self.input.read_line(&mut line).unwrap_or(0) > 0 {
            for (i, ch) in line.chars().enumerate() {
                match Scanner::get_token(ch) {
                    Some(token) => tokens.push(token),
                    None => {}
                }
            }
            line.clear();
        }
        tokens
    }
}

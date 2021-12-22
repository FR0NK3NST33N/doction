use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Tokens {
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
    pub fn scan(mut self) -> Vec<Tokens> {
        let mut line = String::new();
        while self.input.read_line(&mut line).unwrap_or(0) > 0 {
            for (i, ch) in line.chars().enumerate() {
                println!("{} {}", i, ch)
            }
            line.clear();
        }
        vec![Tokens::LBRACE]
    }
}

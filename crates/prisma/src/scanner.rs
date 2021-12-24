use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{Iterator, Peekable};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    ATTRIBUTE(String),
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
    fn is_identifier(ch: Option<char>) {}
    fn get_token<I: Iterator<Item = char>>(ch: char, chars: &mut Peekable<I>) -> Option<Token> {
        match ch {
            '{' => Some(Token::LBRACE),
            '}' => Some(Token::RBRACE),
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            '@' => {
                let mut values: Vec<char> = chars
                    .by_ref()
                    .take_while(|ch| !ch.is_whitespace() && !(ch == &'('))
                    .collect();
                let mut at = vec!['@'];
                at.append(&mut values);

                return Some(Token::ATTRIBUTE(at.into_iter().collect()));
            }
            _ => None,
        }
    }
    pub fn scan(mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut line = String::new();
        while self.input.read_line(&mut line).unwrap_or(0) > 0 {
            let mut iter = line.chars().peekable();
            let mut ch = iter.next();
            while ch != None {
                match ch {
                    Some(val) => match Scanner::get_token(val, &mut iter) {
                        Some(token) => tokens.push(token),
                        None => {}
                    },
                    None => {}
                }
                ch = iter.next();
            }
            // for (i, ch) in line.chars().enumerate() {
            //     match Scanner::get_token(ch) {
            //         Some(token) => tokens.push(token),
            //         None => {}
            //     }
            // }
            line.clear();
        }
        tokens
    }
}

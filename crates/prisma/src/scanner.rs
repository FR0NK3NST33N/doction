use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{Iterator, Peekable};

#[derive(Debug, PartialEq)]
pub enum Token {
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COMMA,
    LPAREN,
    RPAREN,
    ATTRIBUTE(String),
    IDENT(String),
    MODEL,
    TYPE(String),
    EOF,
    DATASOURCE,
    GENERATOR,
    EQUAL,
    STRING(String),
}

pub struct Scanner {
    input: BufReader<File>,
}

impl Scanner {
    pub fn new(input: BufReader<File>) -> Scanner {
        Scanner { input }
    }
    fn get_token<I: Iterator<Item = char>>(ch: char, chars: &mut Peekable<I>) -> Option<Token> {
        match ch {
            '{' => Some(Token::LBRACE),
            '}' => Some(Token::RBRACE),
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            '=' => Some(Token::EQUAL),
            '[' => Some(Token::LBRACKET),
            ']' => Some(Token::RBRACKET),
            ',' => Some(Token::COMMA),
            '@' => {
                let mut attribute: Vec<char> = chars
                    .by_ref()
                    .take_while(|ch| !ch.is_whitespace() && !(ch == &'('))
                    .collect();
                let mut at = vec!['@'];
                at.append(&mut attribute);

                return Some(Token::ATTRIBUTE(at.into_iter().collect()));
            }
            '"' => {
                let string: Vec<char> = chars
                    .by_ref()
                    .take_while(|ch| (ch != &'"'))
                    .filter(|ch| ch != &'"')
                    .collect();

                return Some(Token::STRING(string.into_iter().collect()));
            }
            value if value.is_alphanumeric() => {
                let mut rest: Vec<char> = chars
                    .by_ref()
                    .take_while(|ch| !ch.is_whitespace() && !(ch == &'('))
                    .collect();
                let mut first = vec![value];
                first.append(&mut rest);
                let term: String = first.into_iter().collect();

                let token = if Scanner::is_keyword(&term) {
                    match Scanner::get_keyword_token(&term) {
                        Some(token) => token,
                        None => panic!("Keyword not found"),
                    }
                } else if Scanner::is_type(&term) {
                    Token::TYPE(term)
                } else {
                    Token::IDENT(term)
                };
                return Some(token);
            }
            _ => None,
        }
    }
    fn is_keyword(term: &str) -> bool {
        let keywords: Vec<&str> = vec!["model", "datasource", "generator"];
        return keywords.contains(&term);
    }
    fn get_keyword_token(keyword: &str) -> Option<Token> {
        match keyword {
            "model" => Some(Token::MODEL),
            "datasource" => Some(Token::DATASOURCE),
            "generator" => Some(Token::GENERATOR),
            _ => None,
        }
    }
    fn is_type(term: &str) -> bool {
        let types: Vec<&str> = vec![
            "String",
            "Boolean",
            "Int",
            "BigInt",
            "Float",
            "Decimal",
            "DateTime",
            "Json",
            "Bytes",
            "Unsupported",
        ];
        return types.contains(&term);
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
            line.clear();
        }
        tokens.push(Token::EOF);
        return tokens;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_model() {
        let model = String::from("model");
        let mut iter = model.chars().peekable();
        let ch = iter.next().unwrap();
        let result = Scanner::get_token(ch, &mut iter);
        assert_eq!(result, Some(Token::MODEL));
    }
    #[test]
    fn test_parse_ident() {
        let ident = String::from("email");
        let mut iter = ident.chars().peekable();
        let ch = iter.next().unwrap();
        let result = Scanner::get_token(ch, &mut iter);
        assert_eq!(result, Some(Token::IDENT(ident)));
    }
    #[test]
    fn test_parse_type() {
        let field_type = String::from("BigInt");
        let mut iter = field_type.chars().peekable();
        let ch = iter.next().unwrap();
        let result = Scanner::get_token(ch, &mut iter);
        assert_eq!(result, Some(Token::TYPE(field_type)));
    }
    #[test]
    fn test_parse_attribute() {
        let attribute = String::from("@default");
        let mut iter = attribute.chars().peekable();
        let ch = iter.next().unwrap();
        let result = Scanner::get_token(ch, &mut iter);
        assert_eq!(result, Some(Token::ATTRIBUTE(attribute)));
    }
    #[test]
    fn test_parse_string() {
        let string = String::from("\"some-string\"");
        let mut iter = string.chars().peekable();
        let ch = iter.next().unwrap();
        let result = Scanner::get_token(ch, &mut iter);
        assert_eq!(result, Some(Token::STRING(String::from("some-string"))));
    }
}

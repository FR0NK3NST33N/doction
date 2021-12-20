use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
pub struct Scanner<'a> {
    input: &'a BufReader<File>,
    read_position: u64,
}

#[allow(dead_code)]
impl Scanner<'_> {
    pub fn new(input: &BufReader<File>) -> Scanner {
        Scanner {
            input: input,
            read_position: 0,
        }
    }
}

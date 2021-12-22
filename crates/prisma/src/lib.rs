use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

mod scanner;
use scanner::Scanner;

pub fn find_schema_file() -> Result<String, String> {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.eq("schema.prisma") {
            return Result::Ok(entry.path().display().to_string());
        }
    }
    return Result::Err(String::from("No schema file found"));
}

pub fn read_schema_file(path: String) -> Result<BufReader<File>, String> {
    match File::open(path) {
        Ok(val) => {
            let buffered = BufReader::new(val);
            return Ok(buffered);
        }
        Err(error) => return Err(String::from(error.to_string())),
    }
}

pub fn parse_schema_file(file: BufReader<File>) -> Result<String, String> {
    let scanner = Scanner::new(file);
    let _tokens = scanner.scan();
    Ok(String::from("Success"))
}

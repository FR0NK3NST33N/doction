use std::fs::File;
use std::io::{BufReader, BufRead};
use walkdir::WalkDir;

pub fn find_schema_file() -> Result<String, String> {
    for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.eq("schema.prisma") {
            return Result::Ok(entry.path().display().to_string())
        }
    }
    return Result::Err(String::from("No schema file found"))
}

pub fn read_schema_file(path: String) -> Result<BufReader<File>, String> {
    match File::open(path) {
        Ok(val) => {
            let buffered = BufReader::new(val);
            return Ok(buffered)
        },
        Err(error) => {
            return Err(String::from(error.to_string()))
        }
    }
}

pub fn parse_schema_file(file: BufReader<File>) -> Result<String, String> {
    for line in file.lines() {
        match line {
            Ok(value) => {
                println!("{}", value)
            },
            Err(error) => {
                return Err(String::from(error.to_string()))
            }
        }
    }
    Ok(String::from("Success"))
}
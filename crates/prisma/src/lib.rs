use std::fs::File;
use std::io;
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

pub fn read_schema_file(path: String) -> io::Result<io::BufReader<File>> {
    let input = File::open(path)?; // ? assumes an Ok result
    let buffered = BufReader::new(input);
    return Ok(buffered)
}

pub fn parse_schema_file(file: io::BufReader<File>) -> io::Result<()> {
    for line in file.lines() {
        println!("{}", line?);
    }
    Ok(())
}
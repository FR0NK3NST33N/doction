fn main() {
    match notion::search() {
        Ok(()) => {
            println!("success!")
        }
        Err(error) => {
            println!("{}", error)
        }
    }
}

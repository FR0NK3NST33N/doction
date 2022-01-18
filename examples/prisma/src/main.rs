fn main() -> std::io::Result<()> {
    doction::doction_test();
    node::node_test();
    let result = prisma::find_schema_file();
    match result {
        Ok(val) => {
            println!("{}", val);
            match prisma::read_schema_file(val) {
                Ok(value) => {
                    let tokens = prisma::parse_schema_file(value);
                    println!("{:?}", tokens);
                }
                Err(error) => {
                    println!("{}", error)
                }
            }
            Ok(())
        }
        Err(err) => {
            println!("{}", err);
            Ok(()) // not handling error
        }
    }
}

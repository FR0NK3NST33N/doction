fn main() -> std::io::Result<()> {
    doction::doction_test();
    node::node_test();
    let result  = prisma::find_schema_file();
    match result {
        Ok(val) => {
            println!("{}", val);
            match prisma::read_schema_file(val) {
                Ok(value) => {
                    prisma::parse_schema_file(value).unwrap();
                },
                Err(error) => {
                    println!("{}", error)
                }
            }
            Ok(())
        },
        Err(err) => {
            println!("{}", err);
            Ok(()) // not handling error
        }
    }
}

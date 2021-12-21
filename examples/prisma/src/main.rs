extern crate doction;
extern crate node;
extern crate prisma;

fn main() -> std::io::Result<()> {
    doction::doction_test();
    node::node_test();
    let result = prisma::find_schema_file();
    match result {
        Ok(val) => {
            println!("{}", val);
            match prisma::read_schema_file(val) {
                Ok(mut value) => {
                    prisma::parse_schema_file(&mut value).unwrap();
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

extern crate doction;
extern crate node;
extern crate prisma;

fn main() -> std::io::Result<()> {
    doction::doction_test();
    node::node_test();
    let result  = prisma::find_schema_file();
    match result {
        Ok(val) => {
            // Use val here....
            println!("{}", val);
            let file_result = prisma::read_schema_file(val)?; // assuming result ok
            prisma::parse_schema_file(file_result); // not handling result
            Ok(())
        },
        Err(err) => {
            // Do something with the error if you want
            println!("{}", err);
            Ok(()) // not handling error
        }
    }
}

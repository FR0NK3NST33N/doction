use prisma;
use prisma::scanner::Token;

#[test]
fn test_basic_schema() {
    let path = prisma::find_schema_file_by_name("basic_schema.prisma").unwrap();
    let file = prisma::read_schema_file(path).unwrap();
    let tokens = prisma::parse_schema_file(file);
    assert_eq!(
        tokens,
        vec![
            Token::MODEL,
            Token::IDENT(String::from("User")),
            Token::LBRACE,
            Token::IDENT(String::from("id")),
            Token::TYPE(String::from("String")),
            Token::ATTRIBUTE(String::from("@id")),
            Token::ATTRIBUTE(String::from("@default")),
            Token::IDENT(String::from("cuid")),
            Token::RPAREN,
            Token::RPAREN,
            Token::IDENT(String::from("email")),
            Token::TYPE(String::from("String")),
            Token::ATTRIBUTE(String::from("@unique")),
            Token::IDENT(String::from("password")),
            Token::TYPE(String::from("String")),
            Token::RBRACE,
            Token::EOF
        ]
    )
}

#[test]
fn test_datasource_schema() {
    let path = prisma::find_schema_file_by_name("schema_and_datasource.prisma").unwrap();
    let file = prisma::read_schema_file(path).unwrap();
    let tokens = prisma::parse_schema_file(file);
    assert_eq!(
        tokens,
        vec![
            Token::DATASOURCE,
            Token::IDENT(String::from("db")),
            Token::LBRACE,
            Token::IDENT(String::from("provider")),
            Token::EQUAL,
            Token::STRING(String::from("postgresql")),
            Token::IDENT(String::from("url")),
            Token::EQUAL,
            Token::STRING(String::from("postgreqsl:://user:pass@localhost:5432/mydb?schema=public")),
            Token::RBRACE,
            Token::MODEL,
            Token::IDENT(String::from("User")),
            Token::LBRACE,
            Token::IDENT(String::from("id")),
            Token::TYPE(String::from("String")),
            Token::ATTRIBUTE(String::from("@id")),
            Token::ATTRIBUTE(String::from("@default")),
            Token::IDENT(String::from("cuid")),
            Token::RPAREN,
            Token::RPAREN,
            Token::IDENT(String::from("email")),
            Token::TYPE(String::from("String")),
            Token::ATTRIBUTE(String::from("@unique")),
            Token::IDENT(String::from("password")),
            Token::TYPE(String::from("String")),
            Token::RBRACE,
            Token::EOF
        ]
    )
}

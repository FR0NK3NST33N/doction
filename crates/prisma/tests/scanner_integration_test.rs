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

#[test]
fn test_datasource_generator_schema() {
    let path = prisma::find_schema_file_by_name("schema_and_datasource_and_generator.prisma").unwrap();
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
            Token::GENERATOR,
            Token::IDENT(String::from("client")),
            Token::LBRACE,
            Token::IDENT(String::from("provider")),
            Token::EQUAL,
            Token::STRING(String::from("prisma-client-js")),
            Token::IDENT(String::from("output")),
            Token::EQUAL,
            Token::STRING(String::from("node_modules/.prisma/client")),
            Token::IDENT(String::from("engineType")),
            Token::EQUAL,
            Token::STRING(String::from("library")),
            Token::IDENT(String::from("binaryTargets")),
            Token::EQUAL,
            Token::LBRACKET,
            Token::STRING(String::from("native")),
            Token::COMMA,
            Token::STRING(String::from("darwin")),
            Token::COMMA,
            Token::STRING(String::from("windows")),
            Token::RBRACKET,
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

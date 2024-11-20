
use std::fs;
use std::io;
use shex_ast::Schema;
use shex_compact::ShExParser;

pub fn read_and_parse_schema(file_path: &str) -> io::Result<Schema> {
    let str = fs::read_to_string(file_path)?;
    let schema = ShExParser::parse(&str, None).unwrap();
    Ok(schema)
}

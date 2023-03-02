pub mod ast;
pub use ast::*;

use srdf::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_build_test() {
        let foo = Schema {
            id: None,
            base: Some(Box::new(SIRI::from("hi"))),
            prefixes: Some(PrefixMap::new())
        };
        let foo_from_builder = 
            SchemaBuilder::new()
                         .set_base(SIRI::from("hi"))
                         .build();
        assert_eq!(foo.base.unwrap().to_str(),foo_from_builder.base.unwrap().to_str());
    }
}

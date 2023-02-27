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
            base: Some(IRI::from(String::from("hi"))),
            prefixes: Some(PrefixMap::new())
        };
        let foo_from_builder = 
            SchemaBuilder::new()
                         .setBase(IRI::from(String::from("hi")))
                         .build();
        assert_eq!(foo_from_builder,foo);
    }
}

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
            base: Some(Box::new(<MyIRI as IRI>::from("hi".to_string()))),
            prefixes: Some(PrefixMap::new())
        };
        let foo_from_builder = 
            SchemaBuilder::new()
                         .set_base(<MyIRI as IRI>::from("hi".to_string()))
                         .build();
        assert_eq!(2,2);
    }
}

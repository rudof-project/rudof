pub mod ast;
pub use ast::*;

use srdf::*;
use prefix_map::PrefixMap;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_build_test() {
        let foo = Schema {
            id: None,
            base: Some(Box::new(IriS::from_str("hi"))),
            prefixes: Some(PrefixMap::new())
        };
        let foo_from_builder = 
            SchemaBuilder::new()
                         .set_base(IriS::from_str("hi"))
                         .build();
        assert_eq!(foo.base.unwrap(),foo_from_builder.base.unwrap());
    }
}

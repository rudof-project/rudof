pub mod ast;
pub use ast::*;



#[cfg(test)]
mod tests {
    use super::*;
    use srdf::*;
    use prefix_map::PrefixMap;
    
    #[test]
    fn schema_build_test() {
        let foo = Schema {
            id: None,
            base: Some(Box::new(IriS::from_str("hi"))),
            prefixes: Some(PrefixMap::new())
        };
        let mut builder = SchemaBuilder::new();
        builder.set_base(IriS::from_str("hi"));
        let foo_from_builder = builder.build();
        assert_eq!(foo.base.unwrap(),foo_from_builder.base.unwrap());
    }
}

//! SHACL RDF
//!
//! Converts between SHACl AST and RDF
//!
#![deny(rust_2018_idioms)]

pub mod rdf_to_shacl;
pub mod shacl_to_rdf;

pub use rdf_to_shacl::*;
pub use shacl_to_rdf::*;
use rdf::rdf_core::FocusRDF;

pub fn parse_shacl_rdf<RDF>(
    rdf: RDF,
) -> Result<shacl_ast::Schema<RDF>, crate::shacl_parser_error::ShaclParserError>
where
    RDF: FocusRDF,
{
    let mut parser = crate::ShaclParser::new(rdf);
    let schema = parser.parse()?;
    Ok(schema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iri_s::iri;
    use shacl_ast::node_shape::NodeShape;
    use shacl_ast::shape::Shape;
    use shacl_ast::target::Target;
    use rdf::rdf_core::{RDFFormat, term::Object};
    use rdf::rdf_impl::{InMemoryGraph, ReaderMode};

    #[test]
    fn test_parse_shacl_rdf() {
        let graph = r#"
            @prefix sh: <http://www.w3.org/ns/shacl#> .
            @prefix : <http://example.org/> .

            :Shape a sh:NodeShape ;
                sh:targetClass :TargetClass .
        "#;

        let rdf =
            InMemoryGraph::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let schema = parse_shacl_rdf(rdf).unwrap();
        let shape = schema
            .get_shape(&Object::iri(iri!("http://example.org/Shape")))
            .unwrap();
        let expected_node_shape = NodeShape::new(Object::iri(iri!("http://example.org/Shape")))
            .with_targets(vec![Target::target_class(Object::iri(iri!(
                "http://example.org/TargetClass"
            )))]);
        let expected_shape = Shape::node_shape(expected_node_shape);
        assert_eq!(*shape, expected_shape);
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTComponent, ASTNodeShape, ASTShape};
    use crate::rdf::ShaclParser;
    use crate::types::Target;
    use rudof_iri::iri;
    use rudof_rdf::rdf_core::RDFFormat;
    use rudof_rdf::rdf_core::term::Object;
    use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};

    #[test]
    fn test_language_in() {
        let shape = r#"
            @prefix :    <http://example.org/> .
            @prefix sh:  <http://www.w3.org/ns/shacl#> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

            :TestShape a sh:NodeShape ;
                sh:targetNode "Hello"@en ;
                sh:languageIn ( "en" "fr" ) .
        "#;

        let shape_id = Object::iri(iri!("http://example.org/TestShape"));
        let graph = InMemoryGraph::from_str(shape, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
        let ast = ShaclParser::new(graph).parse().unwrap();
        let shape = match ast.get_shape(&shape_id).unwrap() {
            ASTShape::NodeShape(ns) => ns,
            _ => unreachable!(),
        };

        match shape.components().first().unwrap() {
            ASTComponent::LanguageIn(langs) => {
                assert_eq!(langs.len(), 2);
                assert_eq!(langs[0].as_str(), "en");
                assert_eq!(langs[1].as_str(), "fr");
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_shacl_rdf() {
        let graph = r#"
            @prefix sh: <http://www.w3.org/ns/shacl#> .
            @prefix : <http://example.org/> .

            :Shape a sh:NodeShape ;
                sh:targetClass :TargetClass .
        "#;
        let shape_id = Object::iri(iri!("http://example.org/Shape"));

        let rdf = InMemoryGraph::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let ast = ShaclParser::new(rdf).parse().unwrap();
        let shape = ast.get_shape(&shape_id).unwrap();
        let expected_node_shape = ASTNodeShape::new(shape_id)
            .with_targets(vec![Target::Class(Object::iri(iri!("http://example.org/TargetClass")))]);

        let expected_shape = ASTShape::node_shape(expected_node_shape);
        assert_eq!(*shape, expected_shape);
    }
}

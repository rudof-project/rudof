#[cfg(test)]
mod tests {
    use crate::ast::{ASTComponent, ASTNodeShape, ASTShape};
    use crate::rdf::ShaclParser;
    use crate::types::Target;
    use rudof_iri::iri;
    use rudof_rdf::rdf_core::term::Object;
    use rudof_rdf::rdf_core::{RDFFormat, SHACLPath};
    use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};

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
        let graph = OxigraphInMemory::from_str(shape, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
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

    /// Parsing `sh:subsetOf` moves the focus node to the path it points to, and
    /// the component parsers run in sequence over the same focus node. If the
    /// focus is not restored, every component declared after `sh:subsetOf` is
    /// parsed against the wrong node. The W3C test suite does not cover this.
    #[test]
    fn test_subset_of_keeps_the_focus_for_later_components() {
        let shape = r#"
            @prefix :    <http://example.org/> .
            @prefix sh:  <http://www.w3.org/ns/shacl#> .

            :TestShape a sh:NodeShape ;
                sh:property :TestShape-property1 .

            :TestShape-property1
                sh:path :property1 ;
                sh:subsetOf ( :property2 :property3 ) ;
                sh:minCount 1 .
        "#;

        let shape_id = Object::iri(iri!("http://example.org/TestShape-property1"));
        let graph = OxigraphInMemory::from_str(shape, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
        let ast = ShaclParser::new(graph).parse().unwrap();
        let shape = match ast.get_shape(&shape_id).unwrap() {
            ASTShape::PropertyShape(ps) => ps,
            _ => unreachable!(),
        };

        let subset_of = shape
            .components()
            .iter()
            .find_map(|c| match c {
                ASTComponent::SubsetOf(path) => Some(path),
                _ => None,
            })
            .expect("sh:subsetOf component not parsed");
        assert_eq!(
            *subset_of,
            SHACLPath::sequence(vec![
                SHACLPath::iri(iri!("http://example.org/property2")),
                SHACLPath::iri(iri!("http://example.org/property3")),
            ])
        );

        // The component declared after sh:subsetOf must still be parsed
        assert!(
            shape
                .components()
                .iter()
                .any(|c| matches!(c, ASTComponent::MinCount(1))),
            "components parsed after sh:subsetOf were lost: {:?}",
            shape.components()
        );
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

        let rdf = OxigraphInMemory::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let ast = ShaclParser::new(rdf).parse().unwrap();
        let shape = ast.get_shape(&shape_id).unwrap();
        let expected_node_shape = ASTNodeShape::new(shape_id)
            .with_targets(vec![Target::Class(Object::iri(iri!("http://example.org/TargetClass")))]);

        let expected_shape = ASTShape::node_shape(expected_node_shape);
        assert_eq!(*shape, expected_shape);
    }
}

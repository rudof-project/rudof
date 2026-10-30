#[cfg(test)]
mod tests {
    use crate::ast::{ASTComponent, ASTNodeExpr, ASTNodeShape, ASTShape};
    use crate::rdf::ShaclParser;
    use crate::types::Target;
    use rudof_iri::iri;
    use rudof_rdf::rdf_core::term::Object;
    use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
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

    fn node_shape_of(shape: &str, id: &str) -> ASTNodeShape {
        let graph = OxigraphInMemory::from_str(shape, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
        let ast = ShaclParser::new(graph).parse().unwrap();

        match ast.get_shape(&Object::iri(iri!(id))).unwrap() {
            ASTShape::NodeShape(ns) => (**ns).clone(),
            _ => unreachable!(),
        }
    }

    /// A plain boolean for `sh:deactivated` is the literal expression case of a
    /// node expression.
    #[test]
    fn test_deactivated_boolean_is_a_literal_expression() {
        let shape = r#"
            @prefix :    <http://example.org/> .
            @prefix sh:  <http://www.w3.org/ns/shacl#> .

            :TestShape a sh:NodeShape ;
                sh:deactivated true .
        "#;

        let shape = node_shape_of(shape, "http://example.org/TestShape");

        match shape.components().first().unwrap() {
            ASTComponent::Deactivated(ASTNodeExpr::Literal(ConcreteLiteral::BooleanLiteral(value))) => {
                assert!(*value)
            },
            other => panic!("expected a literal expression, got {other}"),
        }
    }

    /// A composite node expression is recognized by the `shnex:` property its
    /// blank node is the subject of, recursively.
    #[test]
    fn test_deactivated_nested_node_expression() {
        let shape = r#"
            @prefix :      <http://example.org/> .
            @prefix sh:    <http://www.w3.org/ns/shacl#> .
            @prefix shnex: <http://www.w3.org/ns/shacl-node-expr#> .

            :TestShape a sh:NodeShape ;
                sh:deactivated [ shnex:exists [ shnex:pathValues :property1 ] ] .
        "#;

        let shape = node_shape_of(shape, "http://example.org/TestShape");

        match shape.components().first().unwrap() {
            ASTComponent::Deactivated(ASTNodeExpr::Exists(exists)) => match exists.as_ref() {
                ASTNodeExpr::PathValues { path, focus_node } => {
                    assert_eq!(*path, SHACLPath::iri(iri!("http://example.org/property1")));
                    assert!(focus_node.is_none());
                },
                other => panic!("expected a path values expression, got {other}"),
            },
            other => panic!("expected an exists expression, got {other}"),
        }
    }

    /// Parsing a node expression walks the focus node through the expression
    /// tree. If it is not restored, every component declared after
    /// `sh:deactivated` is parsed against the wrong node — the same hazard as
    /// `test_subset_of_keeps_the_focus_for_later_components`.
    #[test]
    fn test_deactivated_keeps_the_focus_for_later_components() {
        let shape = r#"
            @prefix :      <http://example.org/> .
            @prefix sh:    <http://www.w3.org/ns/shacl#> .
            @prefix shnex: <http://www.w3.org/ns/shacl-node-expr#> .

            :TestShape a sh:NodeShape ;
                sh:deactivated [ shnex:exists [ shnex:pathValues :property1 ] ] ;
                sh:minCount 1 .
        "#;

        let shape = node_shape_of(shape, "http://example.org/TestShape");

        assert!(
            shape
                .components()
                .iter()
                .any(|c| matches!(c, ASTComponent::MinCount(1))),
            "components parsed after sh:deactivated were lost: {:?}",
            shape.components()
        );
    }

    const PATH_PREFIXES: &str = "@prefix : <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n";

    fn parse_shapes(shapes: &str) -> Result<(), String> {
        let graph = OxigraphInMemory::from_str(shapes, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
        ShaclParser::new(graph).parse().map(|_| ()).map_err(|e| e.to_string())
    }

    #[test]
    fn long_alternative_path_is_parsed() {
        let preds: String = (0..5000).map(|i| format!(" :p{i}")).collect();
        parse_shapes(&format!(
            "{PATH_PREFIXES}:S a sh:PropertyShape ; sh:path [ sh:alternativePath ({preds} ) ] ."
        ))
        .unwrap();
    }

    #[test]
    fn shapes_share_one_budget_for_their_paths() {
        // _:l10 is ( _:l9 _:l9 ) down to _:l1 = ( :q :q ), 2047 steps, and 2000 shapes parse it again each
        let mut shapes = format!("{PATH_PREFIXES}_:l1 rdf:first :q ; rdf:rest ( :q ) .\n");
        for n in 2..=10 {
            let m = n - 1;
            shapes.push_str(&format!("_:l{n} rdf:first _:l{m} ; rdf:rest ( _:l{m} ) .\n"));
        }
        for i in 0..2000 {
            shapes.push_str(&format!(":S{i} a sh:PropertyShape ; sh:path _:l10 .\n"));
        }

        let err = parse_shapes(&shapes).unwrap_err();
        assert!(err.contains("steps plus"), "{err}");
    }
}

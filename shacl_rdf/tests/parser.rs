use iri_s::IriS;
use shacl_ast::component::Component;
use shacl_ast::shape::Shape;
use shacl_rdf::ShaclParser;
use srdf::Object;
use srdf::RDFFormat;
use srdf::ReaderMode;
use srdf::SRDFGraph;
use srdf::lang::Lang;

#[cfg(test)]
mod parser_tests {
    use super::*;

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

        let rdf_format = RDFFormat::Turtle;
        let reader_mode = ReaderMode::default();
        let shape_id: Object = IriS::new_unchecked("http://example.org/TestShape").into();

        let graph = SRDFGraph::from_str(shape, &rdf_format, None, &reader_mode).unwrap();
        let schema = ShaclParser::new(graph).parse().unwrap();
        let shape = match schema.get_shape(&shape_id).unwrap() {
            Shape::NodeShape(ns) => ns,
            _ => panic!("Shape is not a NodeShape"),
        };

        match shape.components().first().unwrap() {
            Component::LanguageIn { langs } => {
                assert_eq!(langs.len(), 2);
                assert_eq!(langs[0], Lang::new("en").unwrap());
                assert_eq!(langs[1], Lang::new("fr").unwrap());
            },
            _ => panic!("Shape has not a LanguageIn component"),
        }
    }
}

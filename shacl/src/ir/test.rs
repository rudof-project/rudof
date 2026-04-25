#[cfg(test)]
mod tests {
    use crate::ir::IRSchema;
    use crate::rdf::ShaclParser;
    use rudof_rdf::rdf_core::RDFFormat;
    use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
    use std::io::Cursor;

    const SCHEMA: &str = r#"
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix ex: <http://example.org/> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] .

        ex:PersonShape2 a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] .
    "#;

    fn load_schema(ast: &str) -> IRSchema {
        let rdf = InMemoryGraph::from_reader(
            &mut Cursor::new(ast),
            "String",
            &RDFFormat::Turtle,
            None,
            &ReaderMode::default(),
        )
        .unwrap();

        ShaclParser::new(rdf).parse().unwrap().try_into().unwrap()
    }

    #[test]
    fn test_schema_iterator() {
        let schema = load_schema(SCHEMA);
        let actual = schema.iter_with_targets().count();
        let expected = 2;
        assert_eq!(actual, expected);
    }
}

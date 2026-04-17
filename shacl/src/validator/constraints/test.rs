
#[cfg(test)]
mod tests {
    use rudof_rdf::rdf_core::RDFFormat;
    use rudof_rdf::rdf_impl::ReaderMode;
    use sparql_service::RdfData;
    use crate::ir::IRSchema;
    use crate::rdf::ShaclParser;
    use crate::validator::processor::{DataValidation, ShaclProcessor};
    use crate::validator::ShaclValidationMode;

    #[test]
    fn test_min_exclusive_native() {
        let graph = r#"
prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:MinInclusive a sh:NodeShape ;
  sh:targetClass :Node ;
  sh:property [
    sh:path :p ;
    sh:datatype xsd:double ;
    sh:minInclusive "0.0"^^xsd:double ;
    sh:minCount 1
 ] .

:ok1 a :Node; :p "0"^^xsd:double .
:ok2 a :Node; :p "10.5"^^xsd:double .
:ko1 a :Node; :p "-5.3"^^xsd:double .
:ko2 a :Node; :p "other" .
:ko3 a :Node; :p "other"^^xsd:double .
"#;

        let rdf = RdfData::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let mut validator: DataValidation = rdf.clone().into();
        let schema = ShaclParser::new(rdf).parse().unwrap();
        let schema_ir: IRSchema = schema.try_into().unwrap();
        let report = validator.validate(&schema_ir, &ShaclValidationMode::Native).unwrap();
        assert_eq!(report.results().len(), 5);
    }
}
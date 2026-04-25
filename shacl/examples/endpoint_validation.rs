#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(not(target_family = "wasm"))]
fn main() -> anyhow::Result<()> {
    use prefixmap::PrefixMap;
    use rudof_rdf::rdf_core::RDFFormat;
    use shacl::validator::ShaclValidationMode;
    use shacl::validator::processor::{EndpointValidation, ShaclProcessor};
    use shacl::validator::store::ShaclDataManager;
    use std::io::Cursor;

    let shacl = r#"
        @prefix ex:  <http://example.org/> .
        @prefix wd:  <http://www.wikidata.org/entity/> .
        @prefix wdt: <http://www.wikidata.org/prop/direct/> .
        @prefix sh:  <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:WikidataExampleShape
            a sh:NodeShape ;
            sh:targetNode wd:Q80 ;
            sh:property [
                sh:path     wdt:P1477 ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
                sh:datatype xsd:string ;
            ] .
    "#;

    let schema = ShaclDataManager::load(&mut Cursor::new(shacl), "Test", &RDFFormat::Turtle, None)?;

    let mut endpoint_validation = EndpointValidation::new("https://query.wikidata.org/sparql", &PrefixMap::default())?;

    let report = endpoint_validation.validate(&schema, &ShaclValidationMode::Native)?;

    println!("{report}");
    Ok(())
}

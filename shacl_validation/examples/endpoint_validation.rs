use std::io::Cursor;

use anyhow::*;
use prefixmap::PrefixMap;
use shacl_ir::schema::SchemaIR;
use shacl_validation::shacl_processor::EndpointValidation;
use shacl_validation::shacl_processor::ShaclProcessor as _;
use shacl_validation::shacl_processor::ShaclValidationMode;
use shacl_validation::store::ShaclDataManager;
use srdf::RDFFormat;

fn main() -> Result<()> {
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

    let schema: SchemaIR = ShaclDataManager::load(Cursor::new(shacl), RDFFormat::Turtle, None)?;

    let endpoint_validation = EndpointValidation::new(
        "https://query.wikidata.org/sparql",
        &PrefixMap::default(),
        ShaclValidationMode::Native,
    )?;

    let report = endpoint_validation.validate(&schema)?;

    println!("{report}");

    Ok(())
}

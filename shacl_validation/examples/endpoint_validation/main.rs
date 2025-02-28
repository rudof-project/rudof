use std::io::Cursor;

use prefixmap::PrefixMap;
use shacl_validation::shacl_processor::EndpointValidation;
use shacl_validation::shacl_processor::ShaclProcessor as _;
use shacl_validation::shacl_processor::ShaclValidationMode;
use shacl_validation::store::ShaclDataManager;
use srdf::RDFFormat;

fn main() {
    let shacl_schema_path = "./examples/endpoint_validation/wikidata.ttl";
    let shacl_schema = std::fs::read_to_string(shacl_schema_path).unwrap();
    let cursor = Cursor::new(shacl_schema);

    let schema = ShaclDataManager::load(cursor, RDFFormat::Turtle, None).unwrap();

    // Some turtle data, even with empty it's broken
    let endpoint = "https://query.wikidata.org/sparql";

    let endpoint_validation = EndpointValidation::new(
        &endpoint,
        &PrefixMap::default(),
        ShaclValidationMode::Native,
    )
    .unwrap();

    let report = endpoint_validation.validate(&schema).unwrap();

    println!("{:?}", report.results());
}

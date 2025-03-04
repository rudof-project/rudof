use shacl_ast::compiled::compiled_shacl_error::CompiledShaclError;
use shacl_ast::compiled::schema::CompiledSchema;
use shacl_ast::ShaclParser;
use shacl_validation::shacl_processor::DefaultShaclProcessor;
use srdf::RDFFormat;
use srdf::ReaderMode;
use srdf::SRDFGraph;
use srdf::SRDFSparql;
use thiserror::Error;

const SCHEMA: &str = r#"
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
            ] .
    "#;

fn main() -> Result<(), ExampleError> {
    let shapes_graph = SRDFGraph::from_str(SCHEMA, &RDFFormat::Turtle, None, &ReaderMode::Lax)?;
    let schema: CompiledSchema<_> = ShaclParser::new(shapes_graph).parse()?.try_into()?;
    let endpoint = SRDFSparql::wikidata()?;
    let report = DefaultShaclProcessor::new(endpoint).validate(&schema)?;
    println!("{report}"); // ValidationReport implements the Display trait (nice output)
    Ok(())
}

#[derive(Error, Debug)]
pub enum ExampleError {
    #[error("Error related to the creation of the Rdf graph")]
    SRDF(#[from] srdf::srdf_graph::srdfgraph_error::SRDFGraphError),

    #[error("Error related to the parsing of the SHACL schema")]
    ShaclParser(#[from] shacl_ast::shacl_parser_error::ShaclParserError),

    #[error("Error related to the validation of the SHACL schema against the RDF graph")]
    Validate(#[from] shacl_validation::validate_error::ValidateError),

    #[error("Error related to the connection to the SPARQL endpoint")]
    Endpoint(#[from] srdf::srdf_sparql::SRDFSparqlError),

    #[error("Error related to the compilation of the SHACL schema")]
    CompiledShacl(#[from] CompiledShaclError),
}

use crate::{Rudof, RudofConfig};

pub fn new(config: RudofConfig) -> Rudof {
    Rudof {
        version: env!("CARGO_PKG_VERSION").to_string(),
        config,
        data: None,
        shacl_shapes: None,
        shacl_shapes_ir: None,
        shacl_validation_results: None,
        shex_schema: None,
        shex_schema_ir: None,
        shex_validation_results: None,
        pg_schema: None,
        pg_schema_validation_results: None,
        shapemap: None,
        sparql_query: None,
        query_results: None,
        dctap: None,
        service_description: None,
        rdf_config: None,
    }
}

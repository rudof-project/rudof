use crate::Rudof;

pub fn reset_all(rudof: &mut Rudof) {
    rudof.data = None;
    rudof.shacl_shapes = None;
    rudof.shacl_validation_results = None;
    rudof.shex_schema = None;
    rudof.shex_schema_ir = None;
    rudof.shex_validator = None;
    rudof.shex_validation_results = None;
    #[cfg(feature = "pgschema")]
    {
        rudof.pg_schema = None;
        rudof.pg_schema_validation_results = None;
        rudof.typemap = None;
    }
    rudof.pg_db_connection = None;
    rudof.shapemap = None;
    rudof.sparql_query = None;
    rudof.query_results = None;
    #[cfg(feature = "dctap")]
    {
        rudof.dctap = None;
    }
    rudof.service_description = None;
    #[cfg(feature = "rdf-config")]
    {
        rudof.rdf_config = None;
    }
    rudof.map_state = None;
}

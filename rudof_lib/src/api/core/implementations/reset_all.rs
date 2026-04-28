use crate::Rudof;

pub fn reset_all(rudof: &mut Rudof) {
    rudof.data = None;
    rudof.shacl_shapes = None;
    rudof.shacl_validation_results = None;
    rudof.shex_schema = None;
    rudof.shex_schema_ir = None;
    rudof.shex_validation_results = None;
    rudof.pg_schema = None;
    rudof.pg_schema_validation_results = None;
    rudof.shapemap = None;
    rudof.query = None;
    rudof.query_results = None;
    rudof.dctap = None;
    rudof.service_description = None;
    rudof.rdf_config = None;
    rudof.map_state = None;
}

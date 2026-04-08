use crate::Rudof;

pub fn reset_pgschema_validation(rudof: &mut Rudof) {
    rudof.pg_schema = None;
    rudof.shapemap = None;
    rudof.pg_schema_validation_results = None;
}

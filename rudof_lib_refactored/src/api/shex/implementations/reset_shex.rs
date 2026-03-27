use crate::Rudof;

pub fn reset_shex(rudof: &mut Rudof) {
    rudof.shex_schema = None;
    rudof.shex_schema_ir = None;
    rudof.shapemap = None;
    rudof.shex_validation_results = None;
    rudof.shex_validator = None;
}

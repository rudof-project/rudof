use crate::Rudof;

pub fn reset_shex(rudof: &mut Rudof) {
    rudof.shex_schema = None;
    rudof.shapemap = None;
    rudof.shex_validation_results = None;
    rudof.shex_validator = None;
}

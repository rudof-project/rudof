use crate::Rudof;

pub fn reset_shacl_validation(rudof: &mut Rudof) {
    rudof.shacl_validation_results = None;
    rudof.shacl_shapes = None;
}

use crate::Rudof;

pub fn reset_shacl_schema(rudof: &mut Rudof) {
    rudof.shacl_shapes = None;
    rudof.shacl_shapes_ir = None;
}

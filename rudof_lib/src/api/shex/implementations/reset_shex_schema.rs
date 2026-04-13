use crate::Rudof;

pub fn reset_shex_schema(rudof: &mut Rudof) {
    rudof.shex_schema = None;
    rudof.shex_schema_ir = None;
    rudof.map_state = None;
}

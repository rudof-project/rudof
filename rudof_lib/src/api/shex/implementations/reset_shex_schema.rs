use crate::Rudof;

pub fn reset_shex_schema(rudof: &mut Rudof) {
    rudof.shex_schema = None;
    rudof.shex_schema_ir = None;
    // The compiled validator is built from the schema IR being cleared above, so it
    // must go with it — otherwise commands that only check `shex_validator` (e.g.
    // loading a shapemap) would keep succeeding against a schema that's supposedly gone.
    rudof.shex_validator = None;
    rudof.map_state = None;
}

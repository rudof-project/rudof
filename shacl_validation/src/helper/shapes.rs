use shacl_ast::{shape::Shape, Schema};
use srdf::RDFNode;

pub fn get_shape_ref(sref: &RDFNode, schema: &Schema) -> Option<&Shape> {
    schema.get_shape(sref)
}

pub fn get_shapes_ref(srefs: &[RDFNode], schema: &Schema) -> Vec<Option<&Shape>> {
    srefs
        .iter()
        .map(|sref| get_shape_ref(sref, schema))
        .collect()
}

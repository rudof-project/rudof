use shacl_ast::{shape::Shape, Schema};
use srdf::RDFNode;

pub fn get_shape_ref<'a>(sref: &'a RDFNode, schema: &'a Schema) -> Option<&'a Shape> {
    schema.get_shape(sref)
}

pub fn get_shapes_ref<'a>(srefs: &'a [RDFNode], schema: &'a Schema) -> Vec<Option<&'a Shape>> {
    srefs
        .iter()
        .map(|sref| get_shape_ref(sref, schema))
        .collect()
}

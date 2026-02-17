use crate::{
    compiled::compile_shape, compiled_shacl_error::CompiledShaclError, schema_ir::SchemaIR,
    shape_label_idx::ShapeLabelIdx,
};
use iri_s::IriS;
use rdf::rdf_core::{Rdf, SHACLPath};
use shacl_ast::{Schema, property_shape::PropertyShape};
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct ReifierInfo {
    reification_required: bool,
    reifier_shape: Vec<ShapeLabelIdx>,
    predicate: IriS,
}

impl ReifierInfo {
    pub fn reification_required(&self) -> bool {
        self.reification_required
    }

    pub fn reifier_shape(&self) -> &Vec<ShapeLabelIdx> {
        &self.reifier_shape
    }

    pub fn predicate(&self) -> &IriS {
        &self.predicate
    }

    pub fn get_reifier_info_property_shape<R: Rdf>(
        shape: &PropertyShape<R>,
        schema: &Schema<R>,
        schema_ir: &mut SchemaIR,
    ) -> Result<Option<Self>, Box<CompiledShaclError>> {
        if let Some(reifier_info) = shape.reifier_info() {
            let mut compiled_shapes = Vec::new();
            for shape_node in reifier_info.reifier_shape() {
                let compiled_shape = compile_shape(shape_node, schema, schema_ir)?;
                compiled_shapes.push(compiled_shape);
            }
            let path = shape.path();
            let predicate = match path {
                SHACLPath::Predicate { pred } => pred.clone(),
                other => {
                    return Err(Box::new(CompiledShaclError::InvalidReifierShapePath {
                        shape_id: Box::new(shape.id().clone()),
                        path: other.to_string(),
                    }));
                },
            };
            Ok(Some(Self {
                reification_required: reifier_info.reification_required(),
                reifier_shape: compiled_shapes,
                predicate,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Display for ReifierInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReifierInfo(reification_required: {}, reifier_shape count: {}, predicate: {})",
            self.reification_required,
            self.reifier_shape.len(),
            self.predicate
        )
    }
}

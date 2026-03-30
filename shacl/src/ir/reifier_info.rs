use crate::ast::{ASTPropertyShape, ASTSchema};
use crate::ir::error::IRError;
use crate::ir::schema::IRSchema;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use iri_s::IriS;
use rudof_rdf::rdf_core::SHACLPath;
use std::fmt::{Display, Formatter};

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

    pub fn get_reifier_info(
        shape: &ASTPropertyShape,
        ast: &ASTSchema,
        ir: &mut IRSchema,
    ) -> Result<Option<Self>, IRError> {
        if let Some(reifier_info) = shape.reifier_info() {
            let mut compiled_shapes = Vec::new();
            for shape in reifier_info.reifier_shape() {
                let idx = ir.register_shape(shape, None, ast)?;
                compiled_shapes.push(idx);
            }
            let predicate = match shape.path() {
                SHACLPath::Predicate { pred } => pred,
                other => {
                    return Err(IRError::InvalidReifierShapePath {
                        shape_id: Box::new(shape.id().clone()),
                        path: other.to_string(),
                    })
                }
            };
            Ok(Some(Self {
                reification_required: reifier_info.reification_required(),
                reifier_shape: compiled_shapes,
                predicate: predicate.clone(),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Display for ReifierInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReifierInfo(reification_required: {}, reifier_shape count: {}, predicate: {})",
            self.reification_required,
            self.reifier_shape.len(),
            self.predicate
        )
    }
}

use std::fmt::Display;

use crate::{
    formal_base_type::FormalBaseType, label_property_spec::LabelPropertySpec, pgs::PropertyGraphSchema,
    pgs_error::PgsError,
};

/// This is wrapper of LabelPropertySpec for edge labels, to distinguish them from node labels.
#[derive(Debug, Clone)]
pub struct EdgeLabelPropertySpec {
    label_property_spec: LabelPropertySpec,
}

impl EdgeLabelPropertySpec {
    pub fn new(label_property_spec: LabelPropertySpec) -> Self {
        EdgeLabelPropertySpec { label_property_spec }
    }

    pub fn semantics(&self, schema: &PropertyGraphSchema) -> Result<FormalBaseType, PgsError> {
        self.label_property_spec.semantics(schema)
    }
}

impl Display for EdgeLabelPropertySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label_property_spec)
    }
}

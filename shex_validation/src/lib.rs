use shapemap::ShapeMap;
use shex_ast::{CompiledSchema, SchemaJson, ShapeLabel};
use srdf::SRDF;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("ShapeLabel not found {shape_label:?} Labels: {existing_labels:?}")]
    LabelNotFoundError {
        shape_label: ShapeLabel,
        existing_labels: Vec<ShapeLabel>,
    },
    #[error("Error converting Json String: {str:?}")]
    FromJsonStr { str: String, err: String },
}

struct Validator {
    schema: CompiledSchema,
}

impl Validator {
    fn new(schema: CompiledSchema) -> Validator {
        Validator { schema: schema }
    }

    fn from_json_str(json_str: String) -> Result<Validator, ValidationError> {
        match serde_json::from_str::<SchemaJson>(json_str.as_str()) {
            Ok(schema_json) => {
                let schema = CompiledSchema::from_schema_json(schema_json)?;
                Ok(Validator::new(schema))
            }
            Err(e) => Err(ValidationError::FromJsonStr {
                str: json_str,
                err: e.to_string(),
            }),
        }
    }
}

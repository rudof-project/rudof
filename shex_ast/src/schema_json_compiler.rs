use std::collections::HashMap;

use crate::{ShapeLabel, ShapeLabelIdx, CompiledSchema, SchemaJson, CompiledSchemaError};
use crate::compiled_schema::ShapeExpr;

type CResult<T> = Result<T, CompiledSchemaError>;

pub struct SchemaJsonCompiler {
    shape_decls_counter: usize
}

impl SchemaJsonCompiler {

    pub fn new() -> SchemaJsonCompiler {
        SchemaJsonCompiler {
            shape_decls_counter: 0
        }
    }

    pub fn compile(&mut self, schema_json: &SchemaJson, compiled_schema: &mut CompiledSchema) -> CResult<()> {
        self.collect_shape_labels(schema_json, compiled_schema)?;
        Ok(())
    }

    pub fn collect_shape_labels(&mut self, schema_json: &SchemaJson, compiled_schema: &mut CompiledSchema) -> CResult<()> {
        match &schema_json.shapes {
            None => Ok(()),
            Some(sds) => {
              for sd in sds {
                let label = self.id_to_shape_label(sd.id.clone())?;
                compiled_schema.add_shape(label, ShapeExpr::Empty);
                self.shape_decls_counter += 1;
              }
              Ok(())
            }
        }
    }

    fn id_to_shape_label<'a>(&self, id: String) -> CResult<ShapeLabel> {
        let label = ShapeLabel::from_iri_str(id)?;
        Ok(label)
    }


}
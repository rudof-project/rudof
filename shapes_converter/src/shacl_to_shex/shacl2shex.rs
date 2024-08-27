use super::{Shacl2ShExConfig, Shacl2ShExError};
use shacl_ast::Schema;

pub struct Shacl2ShEx {
    config: Shacl2ShExConfig,
    current_shacl: Schema,
}

impl Shacl2ShEx {
    pub fn new(config: Shacl2ShExConfig) -> Shacl2ShEx {
        Shacl2ShEx {
            config,
            current_shacl: Schema::new(),
        }
    }

    pub fn current_shacl(&self) -> &Schema {
        &self.current_shacl
    }

    pub fn convert(&mut self, schema: &Schema) -> Result<(), Shacl2ShExError> {
        let _prefixmap = schema.prefix_map().without_rich_qualifying();
        for (_node, _shape) in schema.iter() {
            //TODO
        }
        Ok(())
    }
}

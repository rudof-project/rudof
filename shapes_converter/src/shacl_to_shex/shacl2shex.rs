use super::{Shacl2ShExConfig, Shacl2ShExError};
use shacl_ast::Schema as ShaclSchema;
use shex_ast::Schema as ShExSchema;

#[allow(dead_code)] // TODO: only for config...
pub struct Shacl2ShEx {
    config: Shacl2ShExConfig,
    current_shex: ShExSchema,
}

impl Shacl2ShEx {
    pub fn new(config: &Shacl2ShExConfig) -> Shacl2ShEx {
        Shacl2ShEx {
            config: config.clone(),
            current_shex: ShExSchema::new(),
        }
    }

    pub fn current_shex(&self) -> &ShExSchema {
        &self.current_shex
    }

    pub fn convert(&mut self, shacl_schema: &ShaclSchema) -> Result<(), Shacl2ShExError> {
        let _prefixmap = shacl_schema.prefix_map().without_rich_qualifying();
        for (_node, _shape) in shacl_schema.iter() {
            //TODO
        }
        Ok(())
    }
}

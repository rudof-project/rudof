use crate::field_generators::{FieldGenerator, GenerationContext};
use crate::Result;

pub struct TemporalGenerator;

impl FieldGenerator for TemporalGenerator {
    fn generate(&self, _context: &GenerationContext) -> Result<String> {
        Ok("2024-01-01".to_string())
    }

    fn name(&self) -> &str { "temporal" }
    fn supported_datatypes(&self) -> Vec<String> {
        vec!["http://www.w3.org/2001/XMLSchema#date".to_string()]
    }
}

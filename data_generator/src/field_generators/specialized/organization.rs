// Placeholder for other specialized generators

use crate::field_generators::{FieldGenerator, GenerationContext};
use crate::{impl_field_generator, Result};

pub struct OrganizationGenerator;

impl FieldGenerator for OrganizationGenerator {
    fn generate(&self, _context: &GenerationContext) -> Result<String> {
        Ok("Sample Organization Corp".to_string())
    }

    fn name(&self) -> &str { "organization" }
    fn supported_datatypes(&self) -> Vec<String> {
        vec!["http://www.w3.org/2001/XMLSchema#string".to_string()]
    }
}

pub struct AcademicGenerator;

impl FieldGenerator for AcademicGenerator {
    fn generate(&self, _context: &GenerationContext) -> Result<String> {
        Ok("Academic Course 101".to_string())
    }

    fn name(&self) -> &str { "academic" }
    fn supported_datatypes(&self) -> Vec<String> {
        vec!["http://www.w3.org/2001/XMLSchema#string".to_string()]
    }
}

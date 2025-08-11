use crate::field_generators::GenerationContext;
use crate::Result;

/// Trait for field value generators
pub trait FieldGenerator: Send + Sync {
    /// Generate a field value for the given context
    fn generate(&self, context: &GenerationContext) -> Result<String>;
    
    /// Get the name/identifier of this generator
    fn name(&self) -> &str;
    
    /// Get supported datatypes for this generator
    fn supported_datatypes(&self) -> Vec<String>;
    
    /// Validate that the generator can handle the given context
    fn can_handle(&self, context: &GenerationContext) -> bool {
        self.supported_datatypes().contains(&context.datatype)
    }
}

/// Factory trait for creating field generators
pub trait FieldGeneratorFactory: Send + Sync {
    /// Create a new instance of the field generator
    fn create(&self) -> Result<Box<dyn FieldGenerator>>;
    
    /// Get the name of the generator this factory creates
    fn generator_name(&self) -> &str;
}

/// Macro to help implement FieldGenerator for simple generators
#[macro_export]
macro_rules! impl_field_generator {
    ($struct_name:ident, $name:expr, $datatypes:expr) => {
        impl FieldGenerator for $struct_name {
            fn name(&self) -> &str {
                $name
            }
            
            fn supported_datatypes(&self) -> Vec<String> {
                $datatypes.iter().map(|s| s.to_string()).collect()
            }
        }
    };
}

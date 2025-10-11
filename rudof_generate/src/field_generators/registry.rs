use crate::field_generators::{FieldGenerator, FieldGeneratorFactory};
use crate::{DataGeneratorError, Result};
use std::collections::HashMap;

/// Registry for managing field generators
pub struct FieldGeneratorRegistry {
    generators: HashMap<String, Box<dyn FieldGenerator>>,
    factories: HashMap<String, Box<dyn FieldGeneratorFactory>>,
    datatype_mappings: HashMap<String, String>,
}

impl Default for FieldGeneratorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldGeneratorRegistry {
    pub fn new() -> Self {
        Self {
            generators: HashMap::new(),
            factories: HashMap::new(),
            datatype_mappings: HashMap::new(),
        }
    }

    /// Register a field generator instance
    pub fn register_generator(&mut self, generator: Box<dyn FieldGenerator>) {
        let name = generator.name().to_string();

        // Register mappings for supported datatypes
        for datatype in generator.supported_datatypes() {
            self.datatype_mappings.insert(datatype, name.clone());
        }

        self.generators.insert(name, generator);
    }

    /// Register a field generator factory
    pub fn register_factory(&mut self, factory: Box<dyn FieldGeneratorFactory>) {
        let name = factory.generator_name().to_string();
        self.factories.insert(name, factory);
    }

    /// Get a generator by name
    pub fn get_generator(&self, name: &str) -> Result<&dyn FieldGenerator> {
        self.generators
            .get(name)
            .map(|g| g.as_ref())
            .ok_or_else(|| {
                DataGeneratorError::FieldGeneration(format!("Generator '{name}' not found"))
            })
    }

    /// Get the default generator for a datatype
    pub fn get_default_generator(&self, datatype: &str) -> Result<&dyn FieldGenerator> {
        let generator_name = self.datatype_mappings.get(datatype).ok_or_else(|| {
            DataGeneratorError::FieldGeneration(format!(
                "No generator found for datatype '{datatype}'"
            ))
        })?;

        self.get_generator(generator_name)
    }

    /// Create a generator instance from a factory
    pub fn create_generator(&self, name: &str) -> Result<Box<dyn FieldGenerator>> {
        let factory = self.factories.get(name).ok_or_else(|| {
            DataGeneratorError::FieldGeneration(format!("Factory for generator '{name}' not found"))
        })?;

        factory.create()
    }

    /// Register all default generators
    pub fn register_default_generators(&mut self) -> Result<()> {
        use crate::field_generators::basic::*;
        use crate::field_generators::pattern::PatternGenerator;

        self.register_generator(Box::new(StringGenerator));
        self.register_generator(Box::new(IntegerGenerator));
        self.register_generator(Box::new(DecimalGenerator));
        self.register_generator(Box::new(BooleanGenerator));
        self.register_generator(Box::new(DateGenerator));
        self.register_generator(Box::new(DateTimeGenerator));
        self.register_generator(Box::new(UriGenerator));
        self.register_generator(Box::new(PatternGenerator));

        Ok(())
    }

    /// List all registered generators
    pub fn list_generators(&self) -> Vec<&str> {
        self.generators.keys().map(|s| s.as_str()).collect()
    }

    /// List all supported datatypes
    pub fn list_datatypes(&self) -> Vec<&str> {
        self.datatype_mappings.keys().map(|s| s.as_str()).collect()
    }
}

pub mod basic;
pub mod pattern;
pub mod registry;
pub mod specialized;
pub mod traits;

pub use registry::FieldGeneratorRegistry;
pub use traits::{FieldGenerator, FieldGeneratorFactory};

use crate::Result;
use crate::config::{DataQuality, FieldGeneratorConfig};
use serde_json::Value;
use std::collections::HashMap;

/// Context for field generation
#[derive(Debug, Clone)]
pub struct GenerationContext {
    /// Property IRI being generated
    pub property: String,
    /// Target datatype IRI
    pub datatype: String,
    /// Subject entity IRI
    pub subject: String,
    /// Additional context parameters
    pub parameters: HashMap<String, Value>,
    /// Data quality level
    pub quality: DataQuality,
    /// Locale for text generation
    pub locale: String,
}

impl GenerationContext {
    pub fn new(property: String, datatype: String, subject: String) -> Self {
        Self {
            property,
            datatype,
            subject,
            parameters: HashMap::new(),
            quality: DataQuality::Medium,
            locale: "en".to_string(),
        }
    }

    pub fn with_quality(mut self, quality: DataQuality) -> Self {
        self.quality = quality;
        self
    }

    pub fn with_locale(mut self, locale: String) -> Self {
        self.locale = locale;
        self
    }

    pub fn with_parameter<K: Into<String>, V: Into<Value>>(mut self, key: K, value: V) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }
}

/// Manager for coordinating field generation
pub struct FieldGenerationManager {
    registry: FieldGeneratorRegistry,
    config: FieldGeneratorConfig,
}

impl FieldGenerationManager {
    pub fn new(config: FieldGeneratorConfig) -> Result<Self> {
        let mut registry = FieldGeneratorRegistry::new();

        // Register default generators
        registry.register_default_generators()?;

        Ok(Self { registry, config })
    }

    /// Generate a field value based on the context
    pub fn generate_field(&self, context: &GenerationContext) -> Result<String> {
        // Try property-specific generator first
        if let Some(prop_config) = self.config.properties.get(&context.property) {
            let generator = self.registry.get_generator(&prop_config.generator)?;
            let mut gen_context = context.clone();

            // Merge property-specific parameters
            for (key, value) in &prop_config.parameters {
                gen_context.parameters.insert(key.clone(), value.clone());
            }

            return generator.generate(&gen_context);
        }

        // Try datatype-specific generator
        if let Some(datatype_config) = self.config.datatypes.get(&context.datatype) {
            let generator = self.registry.get_generator(&datatype_config.generator)?;
            let mut gen_context = context.clone();

            // Merge datatype-specific parameters
            for (key, value) in &datatype_config.parameters {
                gen_context.parameters.insert(key.clone(), value.clone());
            }

            return generator.generate(&gen_context);
        }

        // Fall back to default generator for datatype
        let generator = self.registry.get_default_generator(&context.datatype)?;
        generator.generate(context)
    }

    /// Generate multiple field values in parallel
    pub async fn generate_fields_parallel(
        &self,
        contexts: Vec<GenerationContext>,
    ) -> Result<Vec<String>> {
        use rayon::prelude::*;

        let results: Result<Vec<String>> = contexts
            .into_par_iter()
            .map(|context| self.generate_field(&context))
            .collect();

        results
    }
}

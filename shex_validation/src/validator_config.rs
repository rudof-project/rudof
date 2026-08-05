use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Deserializer, Serialize};
use shex_ast::ir::external_resolver::{ExternalShapeResolver, ExternalShapeResolverRegistry};
use shex_ast::shapemap::ShapemapConfig;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::{ShExConfig, ValidatorError};

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    /// Maximum numbers of validation steps
    pub(crate) max_steps: Option<usize>,

    /// Configuration of RDF data readers
    pub(crate) rdf_data: RdfDataConfig,

    /// Configuration of ShEx schemas
    pub(crate) shex: ShExConfig,

    /// Configuration of Shapemaps
    pub(crate) shapemap: ShapemapConfig,

    /// Whether to check the negation requirement (default: true)
    pub(crate) check_negation_requirement: bool,

    /// Width for pretty printing
    // TODO - This should be in rudof_lib
    pub(crate) width: usize,


    /// Resolvers consulted for EXTERNAL shape expressions. Defaults to a
    /// registry containing only `RejectAllExternalResolver`. Resolvers cannot
    /// be loaded from TOML — they must be installed programmatically via
    /// [`Self::with_external_resolver`].
    #[serde(skip, default)]
    pub(crate) external_resolvers: ExternalShapeResolverRegistry,
}

impl ValidatorConfig {
    pub fn new() -> Self {
        Self {
            max_steps: Self::default_max_steps(),
            width: Self::default_width(),
            rdf_data: Self::default_rdf_data(),
            shex: Self::default_shex(),
            shapemap: Self::default_shapemap(),
            check_negation_requirement: Self::default_check_negation_requirement(),
            external_resolvers: ExternalShapeResolverRegistry::default(),
        }
    }

    /// Obtain a `ValidatorConfig` from a path file in TOML format
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ValidatorConfig, ValidatorError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ValidatorError::ValidatorConfigFromPathError {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ValidatorError::ValidatorConfigFromPathError {
                path: path_name.clone(),
                error: e.to_string(),
            })?;

        let config: ValidatorConfig =
            toml::from_str(s.as_str()).map_err(|e| ValidatorError::ValidatorConfigTomlError {
                path: path_name.clone(),
                error: e.to_string(),
            })?;
        Ok(config)
    }

    pub fn with_max_steps(mut self, steps: Option<usize>) -> Self {
        self.max_steps = steps;
        self
    }

    pub fn with_rdf_data(mut self, cfg: RdfDataConfig) -> Self {
        self.rdf_data = cfg;
        self
    }

    pub fn with_shex(mut self, cfg: ShExConfig) -> Self {
        self.shex = cfg;
        self
    }

    pub fn with_shapemap(mut self, cfg: ShapemapConfig) -> Self {
        self.shapemap = cfg;
        self
    }

    pub fn with_check_negation_requirement(mut self, flag: bool) -> Self {
        self.check_negation_requirement = flag;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_external_shape_resolver_registry(mut self, r: ExternalShapeResolverRegistry) -> Self {
        self.external_resolvers = r;
        self
    }

    /// Prepend a resolver to the EXTERNAL-shape resolver chain. Returns the
    /// updated config for builder-style chaining.
    pub fn with_external_resolver<R: ExternalShapeResolver + 'static>(mut self, r: R) -> Self {
        self.external_resolvers = std::mem::take(&mut self.external_resolvers).with_resolver(r);
        self
    }

    pub fn with_external_resolver_arc(mut self, r: Arc<dyn ExternalShapeResolver>) -> Self {
        self.external_resolvers = std::mem::take(&mut self.external_resolvers).with_resolver_arc(r);
        self
    }
}

impl ValidatorConfig {
    pub fn max_steps(&self) -> Option<usize> {
        self.max_steps
    }

    pub fn rdf_data(&self) -> &RdfDataConfig {
        &self.rdf_data
    }

    pub fn shex(&self) -> &ShExConfig {
        &self.shex
    }

    pub fn shapemap(&self) -> &ShapemapConfig {
        &self.shapemap
    }

    pub fn check_negation_requirement(&self) -> bool {
        self.check_negation_requirement
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn external_resolvers(&self) -> &ExternalShapeResolverRegistry {
        &self.external_resolvers
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ValidatorConfig {
    #[inline] fn default_max_steps() -> Option<usize> { None }
    #[inline] fn default_rdf_data() -> RdfDataConfig { RdfDataConfig::new() }
    #[inline] fn default_shex() -> ShExConfig { ShExConfig::new() }
    #[inline] fn default_shapemap() -> ShapemapConfig { ShapemapConfig::new() }
    #[inline] fn default_check_negation_requirement() -> bool { true }
    #[inline] fn default_width() -> usize { 80 }
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self::new()
    }
}




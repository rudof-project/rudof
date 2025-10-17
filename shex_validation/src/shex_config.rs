use iri_s::IriS;
use serde::{Deserialize, Serialize};
use shex_ast::ShExFormat;
use srdf::RdfDataConfig;
use std::io::Read;
use std::path::Path;
use thiserror::Error;

/// ShEx configuration on main
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct ShExConfigMain {
    /// Show information about time
    pub show_time: Option<bool>,

    /// Specific ShEx configuration
    pub shex: Option<ShExConfig>,
}

impl ShExConfigMain {
    /// Obtain a `ShExConfigMain` from a path file in TOML format
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ShExConfigMain, ShExConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ShExConfigError::FromPathError {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ShExConfigError::FromPathError {
                path: path_name.clone(),
                error: e.to_string(),
            })?;
        let config: ShExConfigMain =
            toml::from_str(s.as_str()).map_err(|e| ShExConfigError::TomlError {
                path: path_name.clone(),
                error: e.to_string(),
            })?;
        Ok(config)
    }

    pub fn shex_config(&self) -> ShExConfig {
        match &self.shex {
            None => ShExConfig::default(),
            Some(sc) => sc.clone(),
        }
    }

    pub fn show_imports(&self) -> bool {
        match &self.shex {
            None => false,
            Some(sc) => sc.show_imports.unwrap_or(false),
        }
    }

    pub fn show_extends(&self) -> bool {
        match &self.shex {
            None => false,
            Some(sc) => sc.show_extends.unwrap_or(false),
        }
    }

    pub fn show_shapes(&self) -> bool {
        match &self.shex {
            None => false,
            Some(sc) => sc.show_shapes.unwrap_or(false),
        }
    }

    pub fn set_show_extends(&mut self, flag: bool) {
        match &mut self.shex {
            None => self.shex = Some(ShExConfig::default().with_show_extends(flag)),
            Some(sc) => sc.clone().set_show_extends(flag),
        }
    }
}

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ShExConfig {
    /// Show information about extended shapes
    pub show_extends: Option<bool>,

    /// Show information about schema imports
    pub show_imports: Option<bool>,

    /// Show information about shapes
    pub show_shapes: Option<bool>,

    /// Show dependencies
    pub show_dependencies: Option<bool>,

    /// Show ShEx Schema Internal Representation
    pub show_ir: Option<bool>,

    /// Default ShEx format
    pub shex_format: Option<ShExFormat>,

    /// Check if schema is well formed
    pub check_well_formed: Option<bool>,

    /// Information about RDF data config which is used for Schemas represented in RDF
    pub rdf_config_shex: Option<RdfDataConfig>,

    /// Default IRI to resolve relative IRIs
    pub base: Option<IriS>,
}

impl Default for ShExConfig {
    fn default() -> Self {
        Self {
            show_extends: Some(true),
            show_imports: Some(true),
            show_shapes: Some(true),
            show_dependencies: Some(true),
            show_ir: Some(true),
            check_well_formed: Some(true),
            rdf_config_shex: Some(RdfDataConfig::default()),
            shex_format: Some(ShExFormat::ShExC),
            base: None,
        }
    }
}

impl ShExConfig {
    pub fn rdf_config(&self) -> RdfDataConfig {
        match &self.rdf_config_shex {
            None => RdfDataConfig::default(),
            Some(c) => c.clone(),
        }
    }

    pub fn check_well_formed(&self) -> bool {
        self.check_well_formed.unwrap_or(true)
    }

    pub fn with_show_extends(mut self, flag: bool) -> Self {
        self.show_extends = Some(flag);
        self
    }

    pub fn set_show_extends(mut self, flag: bool) {
        self.show_extends = Some(flag);
    }

    pub fn with_show_imports(mut self, flag: bool) -> Self {
        self.show_imports = Some(flag);
        self
    }

    pub fn with_show_shapes(mut self, flag: bool) -> Self {
        self.show_shapes = Some(flag);
        self
    }

    pub fn with_show_dependencies(mut self, flag: bool) -> Self {
        self.show_dependencies = Some(flag);
        self
    }

    pub fn without_showing_stats(&mut self) {
        self.show_extends = Some(false);
        self.show_imports = Some(false);
        self.show_shapes = Some(false);
        self.show_dependencies = Some(false);
    }
}

#[derive(Error, Debug, Clone)]
pub enum ShExConfigError {
    #[error("Error reading config file from path {path}: {error}")]
    FromPathError { path: String, error: String },

    #[error("Error reading config file from path {path}: {error}")]
    TomlError { path: String, error: String },
}

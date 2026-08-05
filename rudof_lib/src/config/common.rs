use serde::{Deserialize, Serialize};
use rudof_iri::{iri_once, IriS};

iri_once!(default_base, "http://base");

/// Shared configuration
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CommonConfig {
    /// Base IRI
    #[serde(rename = "base_iri", skip_serializing_if = "Option::is_none")]
    pub(crate) base: Option<IriS>,

    /// If auto_base is true, the base will be http://base if no base is provided
    #[serde(rename = "auto_base")]
    pub(crate) auto_base: bool,
}

/// Constructor and setters
impl CommonConfig {
    /// Creates a new [`CommonConfig`].
    pub fn new() -> Self {
        Self {
            base: Self::default_base(),
            auto_base: Self::default_auto_base(),
        }
    }

    /// Sets `base` and returns itself
    pub fn with_base(mut self, base: Option<IriS>) -> Self {
        self.base = base;
        self
    }

    /// Sets `auto_base` and returns itself
    pub fn with_auto_base(mut self, base_no_fail: bool) -> Self {
        self.auto_base = base_no_fail;
        self
    }
}

/// Accessor methods
impl CommonConfig {
    /// Returns the base IRI
    /// If `base` is not set and auto_base is `true` returns http://base
    pub fn base(&self) -> Option<&IriS> {
        if let Some(iri) = &self.base {
            return Some(iri)
        }

        if self.auto_base {
            return Some(&default_base())
        }

        None
    }

    /// Returns whether `auto_base` is enabled
    pub fn auto_base(&self) -> bool {
        self.auto_base
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl CommonConfig {
    #[inline] fn default_base() -> Option<IriS> { None }
    #[inline] fn default_auto_base() -> bool { false }
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self::new()
    }
}
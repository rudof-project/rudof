use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use crate::errors::IriError;

/// IRI (Internationalized Resource Identifier) for Rudof.
///
/// Wraps the `iri_s::IriS` type to provide a consistent interface for handling IRIs throughout Rudof.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Iri {
    inner: IriS,
}

impl Iri {
    /// Creates a new IRI from a string with validation.
    ///
    /// This function validates that the provided string is a syntactically valid IRI.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice containing a valid IRI
    ///
    /// # Errors
    ///
    /// Returns `IriError::ParseError` if:
    /// - The string is not a valid IRI syntax
    /// - The IRI contains invalid characters
    /// - The IRI structure is malformed
    pub fn new(s: &str) -> Result<Self, IriError> {
        let inner = IriS::new(s).map_err(|e| IriError::ParseError {
            iri: s.to_string(),
            error: e.to_string(),
        })?;
        Ok(Iri { inner })
    }

    /// Creates a new IRI from a string without validation.
    ///
    /// This is a performance optimization that skips validation checks.
    /// Use this only when you are certain the string is a valid IRI.
    /// 
    /// # Arguments
    ///
    /// * `s` - A string slice that MUST be a valid IRI
    pub fn new_unchecked(s: &str) -> Self {
        Iri {
            inner: IriS::new_unchecked(s),
        }
    }

    /// Creates an IRI from a string with an optional base IRI for resolution.
    ///
    /// If a base IRI is provided, the string is resolved relative to it. If no base is provided, the string must be an absolute IRI.
    ///
    /// # Arguments
    ///
    /// * `s` - A string that may be relative or absolute
    /// * `base` - Optional base IRI for resolving relative references
    ///
    /// # Errors
    ///
    /// Returns `IriError::ParseError` if:
    /// - The string cannot be parsed as an IRI
    /// - Resolution against the base fails
    /// - The resulting IRI is invalid
    pub fn from_str_with_base_iri(s: &str, base: Option<&Iri>) -> Result<Self, IriError> {
        let base_iris = base.map(|b| &b.inner);
        let inner = IriS::from_str_base_iri(s, base_iris).map_err(|e| IriError::ParseError {
            iri: s.to_string(),
            error: e.to_string(),
        })?;
        Ok(Iri { inner })
    }

    /// Extends this IRI by appending a path segment.
    ///
    /// This operation concatenates the segment to the end of the IRI, adding a `/` separator if needed (unless the IRI already ends with `/` or `#`).
    /// This is useful for building hierarchical IRIs in RDF vocabularies.
    ///
    /// Unlike `join()`, this does NOT interpret `..` or `./` - the segment is
    /// appended literally.
    ///
    /// # Arguments
    ///
    /// * `segment` - The path segment to append (should not contain `/`)
    ///
    /// # Errors
    ///
    /// Returns `IriError::ExtendError` if the resulting IRI would be syntactically invalid.
    pub fn extend(&self, segment: &str) -> Result<Self, IriError> {
        let inner = self.inner.extend(segment).map_err(|e| IriError::ExtendError {
            base: self.to_string(),
            segment: segment.to_string(),
            error: e.to_string(),
        })?;
        Ok(Iri { inner })
    }

    /// Extends this IRI with a path segment without validation.
    ///
    /// This is a performance optimization that skips validation of the resulting IRI.
    /// Use with caution.
    ///
    /// # Arguments
    ///
    /// * `segment` - The path segment to append
    pub fn extend_unchecked(&self, segment: &str) -> Self {
        Iri {
            inner: self.inner.extend_unchecked(segment),
        }
    }

    /// Joins a path to this IRI following URL resolution rules.
    ///
    /// This method follows standard web URL resolution:
    /// - Interprets `..` to move up one directory level
    /// - Interprets `./` as current directory
    /// - Absolute paths (starting with `/`) replace the entire path
    /// - Complete URLs replace the entire IRI
    ///
    /// This is appropriate for resolving web URLs but may give unexpected results when building RDF vocabulary IRIs. Use `extend()` for that case.
    ///
    /// # Arguments
    ///
    /// * `path` - A relative or absolute path/URL to join
    ///
    /// # Errors
    ///
    /// Returns `IriError::JoinError` if the join operation fails or produces an invalid IRI.
    pub fn join(&self, path: &str) -> Result<Self, IriError> {
        let inner = self.inner.clone().join(path).map_err(|e| IriError::JoinError {
            base: self.to_string(),
            path: path.to_string(),
            error: e.to_string(),
        })?;
        Ok(Iri { inner })
    }

    /// Resolves a relative IRI reference against this IRI as the base.
    ///
    /// # Arguments
    ///
    /// * `relative` - A relative IRI reference to resolve
    ///
    /// # Errors
    ///
    /// Returns `IriError::ResolveError` if:
    /// - The relative reference is malformed
    /// - The resolution produces an invalid IRI
    /// - The base IRI cannot be parsed
    pub fn resolve(&self, relative: &str) -> Result<Self, IriError> {
        let inner = self.inner.resolve_str(relative).map_err(|e| IriError::ResolveError {
            base: self.to_string(),
            relative: relative.to_string(),
            error: e.to_string(),
        })?;
        Ok(Iri { inner })
    }

    /// Dereferences the IRI and retrieves its content via HTTP(S) or file system.
    ///
    /// This method fetches the content located at the IRI:
    /// - For `http://` and `https://` URLs: Makes an HTTP GET request
    /// - For `file://` URLs: Reads from the local file system
    ///
    /// The User-Agent header is set to "rudof" for HTTP requests.
    ///
    /// # Arguments
    ///
    /// * `base` - Optional base IRI for resolving relative references before dereferencing
    ///
    /// # Errors
    ///
    /// Returns `IriError::DereferenceError` if:
    /// - Network request fails (timeout, DNS error, connection refused)
    /// - File cannot be read (permission denied, file not found)
    /// - Response cannot be decoded as text
    /// - The IRI uses an unsupported scheme
    #[cfg(not(target_family = "wasm"))]
    pub fn dereference(&self, base: Option<&Iri>) -> Result<String, IriError> {
        let base_iris = base.map(|b| &b.inner);
        self.inner.dereference(base_iris).map_err(|e| IriError::DereferenceError {
            iri: self.to_string(),
            error: e.to_string(),
        })
    }

    /// Dereference stub for WASM environments (not supported).
    ///
    /// Network operations and file system access are not available in WebAssembly.
    ///
    /// # Errors
    ///
    /// Always returns `IriError::WasmNotSupported`.
    #[cfg(target_family = "wasm")]
    pub fn dereference(&self, _base: Option<&Iri>) -> Result<String, IriError> {
        Err(IriError::WasmNotSupported {
            operation: "dereference".to_string(),
        })
    }

    /// Returns the standard RDF type predicate IRI (`rdf:type`).
    pub fn rdf_type() -> Self {
        Iri {
            inner: IriS::rdf_type(),
        }
    }

    /// Returns a reference to the inner `IriS` type.
    ///
    /// This is an internal method used for interoperability with libraries that expect `iri_s::IriS`.
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Returns a reference to the inner `IriS`.
    pub(crate) fn inner(&self) -> &IriS {
        &self.inner
    }

    /// Consumes the wrapper and returns the inner `IriS`.
    ///
    /// This is an internal method used for converting to the underlying type.
    pub(crate) fn into_inner(self) -> IriS {
        self.inner
    }
}

impl Display for Iri {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FromStr for Iri {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Iri::new(s)
    }
}

impl Default for Iri {
    fn default() -> Self {
        Iri {
            inner: IriS::default(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&std::path::Path> for Iri {
    type Error = IriError;

    fn try_from(path: &std::path::Path) -> Result<Self, Self::Error> {
        let inner = IriS::try_from(path).map_err(|e| IriError::PathConversionError {
            path: path.to_string_lossy().to_string(),
            error: e.to_string(),
        })?;
        Ok(Iri { inner })
    }
}

impl From<IriS> for Iri {
    fn from(inner: IriS) -> Self {
        Iri { inner }
    }
}

impl From<Iri> for IriS {
    fn from(iri: Iri) -> Self {
        iri.inner
    }
}
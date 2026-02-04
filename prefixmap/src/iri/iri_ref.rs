use crate::PrefixMap;
use crate::error::{DerefError, IriRefError, PrefixMapError};
use crate::iri::deref::Deref;
use iri_s::error::IriSError;
use iri_s::IriS;
use serde::Serialize;
use std::borrow::Cow;
use std::{fmt::Display, str::FromStr};

/// An IRI reference, which can be either a full IRI or a prefixed name
// TODO - Move to iri_s crate
#[derive(Serialize, Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
#[serde(into = "String")]
pub enum IriRef {
    Iri(IriS),
    Prefixed { prefix: String, local: String },
}

/// Functions for working with [`IriRef`]
impl IriRef {
    /// Tries to get the IRI
    ///
    /// Usually you want to use [`self.get_iri_prefixmap`] instead
    ///
    /// Returns a reference to the IRI if successful, or an [`IriRefError`] if it is a prefixed name
    pub fn get_iri(&self) -> Result<&IriS, IriRefError> {
        match self {
            IriRef::Iri(iri) => Ok(iri),
            IriRef::Prefixed { prefix, local } => Err(IriRefError {
                prefix: prefix.clone(),
                local: local.clone(),
            }),
        }
    }

    /// Gets the IRI, resolving prefixed names using the provided [`PrefixMap`]
    ///
    /// Returns a [`Cow`], which is borrowed if the [`IriRef`] is already an IRI, or owned if it was a prefixed name.
    /// If the prefixed name cannot be resolved, returns a [`PrefixMapError`]
    pub fn get_iri_prefixmap(
        &self,
        prefixmap: &PrefixMap,
    ) -> Result<Cow<'_, IriS>, PrefixMapError> {
        match self {
            IriRef::Iri(iri) => Ok(Cow::Borrowed(iri)),
            IriRef::Prefixed { prefix, local } => prefixmap
                .resolve_prefix_local(prefix, local)
                .map(Cow::Owned),
        }
    }

    /// Creates a prefixed name [`IriRef`] from the given prefix and local part
    pub fn prefixed<S: Into<String>>(prefix: S, local: S) -> IriRef {
        IriRef::Prefixed {
            prefix: prefix.into(),
            local: local.into(),
        }
    }

    /// Creates an ['IriRef'] from an [`IriS`]
    pub fn iri(iri: IriS) -> IriRef {
        IriRef::Iri(iri)
    }
}

impl Deref for IriRef {
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError> {
        match self {
            IriRef::Iri(iri_s) => {
                let resolved = match base {
                    None => iri_s,
                    Some(base) => base.resolve(iri_s)?,
                };
                Ok(IriRef::Iri(resolved))
            }
            IriRef::Prefixed { prefix, local } => {
                let prefixmap = match prefixmap {
                    None => return Err(DerefError::NoPrefixMapPrefixedName { prefix, local }),
                    Some(pm) => pm,
                };

                let iri = prefixmap
                    .resolve_prefix_local(&prefix, &local)
                    .map_err(|e| DerefError::DerefPrefixMapError {
                        alias: prefix,
                        local,
                        error: Box::new(e),
                    })?;

                Ok(IriRef::Iri(iri))
            }
        }
    }
}

impl TryFrom<&str> for IriRef {
    type Error = IriSError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        FromStr::from_str(value)
    }
}

impl FromStr for IriRef {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IriRef::Iri(IriS::from_str(s)?))
    }
}

impl From<IriRef> for IriS {
    fn from(iri_ref: IriRef) -> IriS {
        match iri_ref {
            IriRef::Iri(iri_s) => iri_s,
            IriRef::Prefixed { prefix, local } => {
                panic!("Cannot convert prefixed name {prefix}:{local} to IriS without context")
            }
        }
    }
}

impl From<IriS> for IriRef {
    fn from(i: IriS) -> IriRef {
        IriRef::Iri(i)
    }
}

impl From<IriRef> for String {
    fn from(iri_ref: IriRef) -> String {
        match iri_ref {
            IriRef::Iri(i) => i.as_str().to_string(),
            IriRef::Prefixed { prefix, local } => format!("{prefix}:{local}"),
        }
    }
}

impl Display for IriRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriRef::Iri(i) => write!(f, "{i}"),
            IriRef::Prefixed { prefix, local } => write!(f, "{prefix}:{local}"),
        }
    }
}

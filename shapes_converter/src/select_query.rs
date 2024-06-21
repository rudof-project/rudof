use std::fmt::Display;

use iri_s::IriS;
use prefixmap::PrefixMap;

use crate::TriplePattern;

pub struct SelectQuery {
    prefixmap: Option<PrefixMap>,
    base: Option<IriS>,
    patterns: Vec<TriplePattern>,
}

impl SelectQuery {
    pub fn new() -> SelectQuery {
        SelectQuery {
            prefixmap: None,
            base: None,
            patterns: Vec::new(),
        }
    }

    pub fn with_prefixmap(mut self, prefixmap: Option<PrefixMap>) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_base(mut self, base: Option<IriS>) -> Self {
        self.base = base;
        self
    }

    pub fn with_patterns(mut self, patterns: Vec<TriplePattern>) -> Self {
        self.patterns = patterns;
        self
    }
}

impl Display for SelectQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(base) = &self.base {
            writeln!(f, "{}", base)?
        };
        // TODO: Unify these 2 branches in one...it was giving an move error on prefixmap that I wanted to bypass quickly...
        if let Some(prefixmap) = &self.prefixmap {
            writeln!(f, "{}", prefixmap)?;
            writeln!(f, "SELECT * WHERE {{")?;
            for pattern in &self.patterns {
                write!(f, " ")?;
                pattern.show_qualified(f, &prefixmap).map_err(|e| match e {
                    prefixmap::PrefixMapError::IriSError(_) => todo!(),
                    prefixmap::PrefixMapError::PrefixNotFound {
                        prefix: _,
                        prefixmap: _,
                    } => todo!(),
                    prefixmap::PrefixMapError::FormatError(err) => err,
                })?;
                writeln!(f, "")?;
            }
        } else {
            let prefixmap = PrefixMap::new();
            for pattern in &self.patterns {
                pattern.show_qualified(f, &prefixmap).map_err(|e| match e {
                    prefixmap::PrefixMapError::IriSError(_) => todo!(),
                    prefixmap::PrefixMapError::PrefixNotFound {
                        prefix: _,
                        prefixmap: _,
                    } => todo!(),
                    prefixmap::PrefixMapError::FormatError(err) => err,
                })?;
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

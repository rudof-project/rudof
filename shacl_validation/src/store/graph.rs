use std::{path::Path, str::FromStr};

use oxiri::Iri;
use srdf::{RDFFormat, ReaderMode, SRDFGraph};

use crate::validate_error::ValidateError;

use super::Store;

pub struct Graph {
    store: SRDFGraph,
}

impl Graph {
    pub fn new(
        path: &Path,
        rdf_format: RDFFormat,
        base: Option<&str>,
    ) -> Result<Self, ValidateError> {
        match SRDFGraph::from_path(
            path,
            &rdf_format,
            match base {
                Some(base) => match Iri::from_str(base) {
                    Ok(iri) => Some(iri),
                    Err(_) => todo!(),
                },
                None => None,
            },
            &ReaderMode::default(), // TODO: this should be revisited
        ) {
            Ok(store) => Ok(Self { store }),
            Err(error) => Err(ValidateError::Graph(error)),
        }
    }
}

impl Store<SRDFGraph> for Graph {
    fn store(&self) -> &SRDFGraph {
        &self.store
    }
}

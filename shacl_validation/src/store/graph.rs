use std::{path::Path, str::FromStr};

use oxiri::Iri;
use srdf::{RDFFormat, SRDFGraph};

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
        let store = match SRDFGraph::from_path(
            path,
            &rdf_format,
            match base {
                Some(base) => match Iri::from_str(base) {
                    Ok(iri) => Some(iri),
                    Err(_) => None,
                },
                None => None,
            },
        ) {
            Ok(rdf) => rdf,
            Err(_) => return Err(ValidateError::GraphCreation),
        };
        Ok(Self { store })
    }
}

impl Store<SRDFGraph> for Graph {
    fn store(&self) -> &SRDFGraph {
        &self.store
    }
}

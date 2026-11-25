use crate::EarlError;
use crate::error::graph_error;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::vocabs::{DoapVocab, XsdVocab};

/// Which release of the software was tested.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Release {
    name: Option<String>,
    revision: String,
    created: Option<String>,
}

impl Release {
    pub fn new(revision: impl Into<String>) -> Self {
        Self {
            revision: revision.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[must_use]
    pub fn with_created(mut self, created: impl Into<String>) -> Self {
        self.created = Some(created.into());
        self
    }
}

impl Release {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn created(&self) -> Option<&str> {
        self.created.as_deref()
    }

    pub fn revision(&self) -> &str {
        &self.revision
    }
}

impl Release {
    pub fn to_rdf<RDF: BuildRDF>(&self, writer: &mut RDF, node: RDF::Subject) -> Result<(), EarlError> {
        writer
            .add_type(node.clone(), DoapVocab::doap_version())
            .map_err(graph_error::<RDF>("Error typing the release"))?;

        writer
            .add_triple::<_, _, RDF::Literal>(
                node.clone(),
                DoapVocab::doap_revision(),
                ConcreteLiteral::str(self.revision()).into(),
            )
            .map_err(graph_error::<RDF>("Error adding the revision to the release"))?;

        if let Some(name) = &self.name {
            writer
                .add_triple::<_, _, RDF::Literal>(
                    node.clone(),
                    DoapVocab::doap_name(),
                    ConcreteLiteral::str(name).into(),
                )
                .map_err(graph_error::<RDF>("Error adding the name to the release"))?;
        }

        if let Some(created) = &self.created {
            let date = ConcreteLiteral::lit_datatype(created, &XsdVocab::xsd_date().into());
            writer
                .add_triple::<_, _, RDF::Literal>(node, DoapVocab::doap_created(), date.into())
                .map_err(graph_error::<RDF>("Error adding the creation date to the release"))?;
        }

        Ok(())
    }
}

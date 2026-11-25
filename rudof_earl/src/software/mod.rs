//! Description of the software an EARL report is about

mod release;

use crate::error::{EarlError, graph_error};
use crate::{new_subject, subject_for};
pub use release::Release;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::vocabs::{DoapVocab, EarlVocab};

/// A `doap:Project` that is also an `earl:Software` and an `earl:TestSubject`
#[derive(Debug, Clone, PartialEq)]
pub struct Software {
    node: Object,
    name: Option<String>,
    description: Option<String>,
    homepage: Option<IriS>,
    developers: Vec<Object>,
    programming_language: Option<String>,
    release: Option<Release>,
}

impl Software {
    pub fn new(node: Object) -> Self {
        Self {
            node,
            name: None,
            description: None,
            homepage: None,
            developers: Vec::new(),
            programming_language: None,
            release: None,
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_homepage(mut self, homepage: IriS) -> Self {
        self.homepage = Some(homepage);
        self
    }

    #[must_use]
    pub fn with_developer(mut self, developer: Object) -> Self {
        self.developers.push(developer);
        self
    }

    #[must_use]
    pub fn with_developers(mut self, developers: Vec<Object>) -> Self {
        self.developers = developers;
        self
    }

    #[must_use]
    pub fn with_programming_language(mut self, language: impl Into<String>) -> Self {
        self.programming_language = Some(language.into());
        self
    }

    #[must_use]
    pub fn with_release(mut self, release: Release) -> Self {
        self.release = Some(release);
        self
    }
}

impl Software {
    pub fn node(&self) -> &Object {
        &self.node
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn homepage(&self) -> Option<&IriS> {
        self.homepage.as_ref()
    }

    pub fn developers(&self) -> &[Object] {
        &self.developers
    }

    pub fn programming_language(&self) -> Option<&str> {
        self.programming_language.as_deref()
    }

    pub fn release(&self) -> Option<&Release> {
        self.release.as_ref()
    }
}

impl Software {
    pub(crate) fn to_rdf<RDF: BuildRDF>(&self, writer: &mut RDF) -> Result<(), EarlError> {
        let node = subject_for::<RDF>(&self.node)?;

        for t in [
            DoapVocab::doap_project(),
            EarlVocab::earl_software(),
            EarlVocab::earl_test_subject(),
        ] {
            writer
                .add_type(node.clone(), t)
                .map_err(graph_error::<RDF>("Error typing the software"))?;
        }

        if let Some(name) = &self.name {
            writer
                .add_triple::<_, _, RDF::Literal>(
                    node.clone(),
                    DoapVocab::doap_name(),
                    ConcreteLiteral::str(name).into(),
                )
                .map_err(graph_error::<RDF>("Error adding the name to the software"))?;
        }

        if let Some(description) = &self.description {
            writer
                .add_triple::<_, _, RDF::Literal>(
                    node.clone(),
                    DoapVocab::doap_description(),
                    ConcreteLiteral::str(description).into(),
                )
                .map_err(graph_error::<RDF>("Error adding the description to the software"))?;
        }

        if let Some(homepage) = &self.homepage {
            let homepage: RDF::Term = homepage.clone().into();
            writer
                .add_triple(node.clone(), DoapVocab::doap_homepage(), homepage)
                .map_err(graph_error::<RDF>("Error adding the homepage to the software"))?;
        }

        for developer in &self.developers {
            let developer: RDF::Term = developer.clone().into();
            writer
                .add_triple(node.clone(), DoapVocab::doap_developer(), developer)
                .map_err(graph_error::<RDF>("Error adding a developer to the software"))?;
        }

        if let Some(language) = &self.programming_language {
            writer
                .add_triple::<_, _, RDF::Literal>(
                    node.clone(),
                    DoapVocab::doap_programming_language(),
                    ConcreteLiteral::str(language).into(),
                )
                .map_err(graph_error::<RDF>("Error adding the language to the software"))?;
        }

        if let Some(release) = &self.release {
            let release_node = new_subject(writer)?;
            let release_term: RDF::Term = release_node.clone().into();
            writer
                .add_triple(node, DoapVocab::doap_release(), release_term)
                .map_err(graph_error::<RDF>("Error adding the release to the software"))?;
            release.to_rdf(writer, release_node)?;
        }

        Ok(())
    }
}

impl From<IriS> for Software {
    fn from(value: IriS) -> Self {
        Software::new(Object::iri(value))
    }
}

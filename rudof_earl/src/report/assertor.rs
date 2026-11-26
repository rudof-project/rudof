use crate::error::graph_error;
use crate::{EarlError, subject_for};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::vocabs::{EarlVocab, FoafVocab};

/// Who asserted the results
#[derive(Debug, Clone, PartialEq)]
pub struct Assertor {
    node: Object,
    types: Vec<IriS>,
    name: Option<String>,
    homepage: Option<IriS>,
}

impl Assertor {
    pub fn new(node: Object) -> Self {
        Self {
            node,
            types: vec![EarlVocab::earl_assertor()],
            name: None,
            homepage: None,
        }
    }

    pub fn with_type(mut self, t: IriS) -> Self {
        self.types.push(t);
        self
    }

    pub fn with_types(mut self, types: Vec<IriS>) -> Self {
        self.types = types;
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_homepage(mut self, homepage: IriS) -> Self {
        self.homepage = Some(homepage);
        self
    }
}

impl Assertor {
    pub fn node(&self) -> &Object {
        &self.node
    }

    pub fn types(&self) -> &[IriS] {
        &self.types
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn homepage(&self) -> Option<&IriS> {
        self.homepage.as_ref()
    }
}

impl Assertor {
    /// Writes this assertor into `writer`.
    pub fn to_rdf<RDF: BuildRDF>(&self, writer: &mut RDF) -> Result<(), EarlError> {
        let node = subject_for::<RDF>(&self.node)?;

        for type_ in &self.types {
            writer
                .add_type(node.clone(), type_.clone())
                .map_err(graph_error::<RDF>("Error typing the assertor"))?;
        }

        if let Some(name) = &self.name {
            writer
                .add_triple::<_, _, RDF::Literal>(
                    node.clone(),
                    FoafVocab::foaf_name(),
                    ConcreteLiteral::str(name).into(),
                )
                .map_err(graph_error::<RDF>("Error adding the name to the assertor"))?;
        }

        if let Some(homepage) = &self.homepage {
            let homepage: RDF::Term = homepage.clone().into();
            writer
                .add_triple(node, FoafVocab::foaf_homepage(), homepage)
                .map_err(graph_error::<RDF>("Error adding the homepage to the assertor"))?;
        }

        Ok(())
    }
}

impl From<IriS> for Assertor {
    fn from(value: IriS) -> Self {
        Self::new(Object::iri(value))
    }
}

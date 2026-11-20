//! EARL report

mod assertor;

use crate::error::{EarlError, graph_error};
use crate::model::{Assertion, Outcome};
use crate::{Software, earl_prefixes, new_subject};
pub use assertor::Assertor;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, XsdDateTime};
use rudof_rdf::rdf_core::vocabs::{DctVocab, FoafVocab};

/// An EARL report, ready to be written as RDF.
///
/// ```
/// use rudof_earl::{Release, Software};
/// use rudof_earl::model::{TestResult, Assertion};
/// use rudof_earl::report::EarlReport;
/// use rudof_iri::IriS;
/// use rudof_rdf::rdf_core::term::Object;
///
/// let tool = IriS::new_unchecked("https://github.com/rudof-project/rudof");
/// let org = IriS::new_unchecked("https://github.com/rudof-project");
///
/// let report = EarlReport::new()
///     .with_software_item(
///         Into::<Software>::into(tool.clone())
///             .with_name("rudof")
///             .with_developer(Object::iri(org.clone()))
///             .with_release(Release::new("0.3.39")),
///     )
///     .with_assertion(Assertion::new(
///         Object::iri(tool),
///         Object::iri(IriS::new_unchecked("urn:x-shacl-test:/core/node/disjoint-001")),
///         TestResult::passed(),
///     ).with_asserted_by(Object::iri(org)));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EarlReport {
    document: Option<IriS>,
    issued: Option<XsdDateTime>,
    maker: Option<Object>,
    primary_topic: Option<Object>,
    title: Option<String>,
    software: Vec<Software>,
    assertions: Vec<Assertion>,
}

impl EarlReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_document(mut self, document: IriS) -> Self {
        self.document = Some(document);
        self
    }

    pub fn with_issued(mut self, issued: XsdDateTime) -> Self {
        self.issued = Some(issued);
        self
    }

    pub fn with_maker(mut self, maker: Object) -> Self {
        self.maker = Some(maker);
        self
    }

    pub fn with_primary_topic(mut self, primary_topic: Object) -> Self {
        self.primary_topic = Some(primary_topic);
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_software_item(mut self, software: Software) -> Self {
        self.software.push(software);
        self
    }

    pub fn with_software(mut self, software: Vec<Software>) -> Self {
        self.software = software;
        self
    }

    pub fn with_assertion(mut self, assertion: Assertion) -> Self {
        self.assertions.push(assertion);
        self
    }

    pub fn with_assertions(mut self, assertions: Vec<Assertion>) -> Self {
        self.assertions = assertions;
        self
    }
}

impl EarlReport {
    pub fn document(&self) -> Option<&IriS> {
        self.document.as_ref()
    }

    pub fn issued(&self) -> Option<&XsdDateTime> {
        self.issued.as_ref()
    }

    pub fn maker(&self) -> Option<&Object> {
        self.maker.as_ref()
    }

    pub fn primary_topic(&self) -> Option<&Object> {
        self.primary_topic.as_ref()
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn assertions(&self) -> &[Assertion] {
        &self.assertions
    }

    pub fn software(&self) -> &[Software] {
        &self.software
    }
}

impl EarlReport {
    /// How many assertions in this report carry the given outcome.
    pub fn count_of(&self, outcome: &Outcome) -> usize {
        self.assertions.iter().filter(|a| a.outcome() == outcome).count()
    }

    /// Writes the whole report into `writer`
    pub fn to_rdf<RDF: BuildRDF>(&self, writer: &mut RDF) -> Result<(), EarlError> {
        for (alias, iri) in earl_prefixes() {
            writer.add_prefix(alias, &iri);
        }

        self.document_to_rdf(writer)?;

        for software in &self.software {
            software.to_rdf(writer)?;
        }

        for assertion in &self.assertions {
            let node = new_subject(writer)?;
            assertion.to_rdf(writer, node)?;
        }

        Ok(())
    }

    fn document_to_rdf<RDF: BuildRDF>(&self, writer: &mut RDF) -> Result<(), EarlError> {
        let Some(document) = &self.document else {
            return Ok(());
        };
        let node: RDF::Subject = document.clone().into();

        if let Some(issued) = &self.issued {
            let issued: RDF::Term = Object::literal(ConcreteLiteral::datetime(issued.clone())).into();
            writer
                .add_triple(node.clone(), DctVocab::dct_issued(), issued)
                .map_err(graph_error::<RDF>("Error adding the issue date to the report"))?;
        }

        if let Some(title) = &self.title {
            writer
                .add_triple::<_, _, RDF::Literal>(
                    node.clone(),
                    DctVocab::dct_title(),
                    ConcreteLiteral::str(title).into(),
                )
                .map_err(graph_error::<RDF>("Error adding the title to the report"))?;
        }

        if let Some(maker) = &self.maker {
            let maker: RDF::Term = maker.clone().into();
            writer
                .add_triple(node.clone(), FoafVocab::foaf_maker(), maker)
                .map_err(graph_error::<RDF>("Error adding the maker to the report"))?;
        }

        if let Some(primary_topic) = &self.primary_topic {
            let primary_topic: RDF::Term = primary_topic.clone().into();
            writer
                .add_triple(node, FoafVocab::foaf_primary_topic(), primary_topic)
                .map_err(graph_error::<RDF>("Error adding the primary topic to the report"))?;
        }

        Ok(())
    }
}

use rudof_iri::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::EarlVocab;
use crate::{new_subject, EarlError};
use crate::error::graph_error;
use crate::model::{Outcome, TestResult};

/// A statement that a subject was tested against a criterion, with a given result.
#[derive(Debug, Clone, PartialEq)]
pub struct Assertion {
    subject: Object,
    test: Object,
    result: TestResult,
    asserted_by: Option<Object>,
    extra: Vec<(IriS, Object)>,
}

impl Assertion {
    pub fn new(subject: Object, test: Object, result: TestResult) -> Self {
        Self {
            subject,
            test,
            result,
            asserted_by: None,
            extra: Vec::new(),
        }
    }

    pub fn with_asserted_by(mut self, asserted_by: Object) -> Self {
        self.asserted_by = Some(asserted_by);
        self
    }

    pub fn with_extra(mut self, extra: Vec<(IriS, Object)>) -> Self {
        self.extra = extra;
        self
    }

    pub fn with_extra_item(mut self, predicate: IriS, object: Object) -> Self {
        self.extra.push((predicate, object));
        self
    }
}

impl Assertion {
    pub fn subject(&self) -> &Object {
        &self.subject
    }

    pub fn test(&self) -> &Object {
        &self.test
    }

    pub fn result(&self) -> &TestResult {
        &self.result
    }

    pub fn asserted_by(&self) -> Option<&Object> {
        self.asserted_by.as_ref()
    }

    pub fn extra(&self) -> &[(IriS, Object)] {
        &self.extra
    }

    pub fn outcome(&self) -> &Outcome {
        self.result.outcome()
    }
}

impl Assertion {
    /// Writes this assertion onto `node`.
    pub(crate) fn to_rdf<RDF: BuildRDF>(&self, writer: &mut RDF, node: RDF::Subject) -> Result<(), EarlError> {
        writer
            .add_type(node.clone(), EarlVocab::earl_assertion())
            .map_err(graph_error::<RDF>("Error typing the assertion"))?;

        let subject: RDF::Term = self.subject.clone().into();
        writer
            .add_triple(node.clone(), EarlVocab::earl_subject(), subject)
            .map_err(graph_error::<RDF>("Error adding the subject to the assertion"))?;

        let test: RDF::Term = self.test.clone().into();
        writer
            .add_triple(node.clone(), EarlVocab::earl_test(), test)
            .map_err(graph_error::<RDF>("Error adding the test to the assertion"))?;

        if let Some(asserted_by) = &self.asserted_by {
            let asserted_by: RDF::Term = asserted_by.clone().into();
            writer
                .add_triple(node.clone(), EarlVocab::earl_asserted_by(), asserted_by)
                .map_err(graph_error::<RDF>("Error adding the assertor to the assertion"))?;
        }

        for (predicate, object) in &self.extra {
            let object: RDF::Term = object.clone().into();
            writer
                .add_triple(node.clone(), predicate.clone(), object)
                .map_err(graph_error::<RDF>("Error adding an extra triple to the assertion"))?;
        }

        let result_node = new_subject(writer)?;
        let result_term: RDF::Term = result_node.clone().into();
        writer
            .add_triple(node, EarlVocab::earl_result(), result_term)
            .map_err(graph_error::<RDF>("Error adding the result to the assertion"))?;
        self.result.to_rdf(writer, result_node)
    }
}

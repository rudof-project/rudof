use crate::EarlError;
use crate::error::graph_error;
use crate::model::{Outcome, TestMode};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, XsdDateTime};
use rudof_rdf::rdf_core::vocabs::{DctVocab, EarlVocab};

/// What carrying out the test produced
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TestResult {
    outcome: Outcome,
    mode: Option<TestMode>,
    info: Option<ConcreteLiteral>,
    pointer: Option<Object>,
    date: Option<XsdDateTime>,
    extra: Vec<(IriS, Object)>,
}

impl TestResult {
    pub fn new(outcome: Outcome) -> Self {
        Self {
            outcome,
            ..Default::default()
        }
    }

    /// A passing result, carried out automatically
    pub fn passed() -> Self {
        Self::new(Outcome::Passed).with_mode(TestMode::Automatic)
    }

    /// A failing result, carried out automatically
    pub fn failed() -> Self {
        Self::new(Outcome::Failed).with_mode(TestMode::Automatic)
    }

    #[must_use]
    pub fn with_mode(mut self, mode: TestMode) -> Self {
        self.mode = Some(mode);
        self
    }

    #[must_use]
    pub fn with_info(mut self, info: ConcreteLiteral) -> Self {
        self.info = Some(info);
        self
    }

    #[must_use]
    pub fn with_pointer(mut self, pointer: Object) -> Self {
        self.pointer = Some(pointer);
        self
    }

    #[must_use]
    pub fn with_date(mut self, date: XsdDateTime) -> Self {
        self.date = Some(date);
        self
    }

    #[must_use]
    pub fn with_extra(mut self, extra: Vec<(IriS, Object)>) -> Self {
        self.extra = extra;
        self
    }

    #[must_use]
    pub fn with_extra_item(mut self, predicate: IriS, object: Object) -> Self {
        self.extra.push((predicate, object));
        self
    }
}

impl TestResult {
    pub fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    pub fn mode(&self) -> Option<&TestMode> {
        self.mode.as_ref()
    }

    pub fn info(&self) -> Option<&ConcreteLiteral> {
        self.info.as_ref()
    }

    pub fn pointer(&self) -> Option<&Object> {
        self.pointer.as_ref()
    }

    pub fn date(&self) -> Option<&XsdDateTime> {
        self.date.as_ref()
    }

    pub fn extra(&self) -> &[(IriS, Object)] {
        &self.extra
    }
}

impl TestResult {
    /// Writes this result onto `node`
    pub(crate) fn to_rdf<RDF: BuildRDF>(&self, writer: &mut RDF, node: RDF::Subject) -> Result<(), EarlError> {
        writer
            .add_type(node.clone(), EarlVocab::earl_test_result())
            .map_err(graph_error::<RDF>("Error typing the test result"))?;

        let outcome: RDF::Term = self.outcome.iri().into();
        writer
            .add_triple(node.clone(), EarlVocab::earl_outcome(), outcome)
            .map_err(graph_error::<RDF>("Error adding the outcome to the test result"))?;

        if let Some(mode) = self.mode {
            let mode: RDF::Term = mode.iri().into();
            writer
                .add_triple(node.clone(), EarlVocab::earl_mode(), mode)
                .map_err(graph_error::<RDF>("Error adding the mode to the test result"))?;
        }

        if let Some(info) = &self.info {
            writer
                .add_triple::<_, _, RDF::Literal>(node.clone(), EarlVocab::earl_info(), info.clone().into())
                .map_err(graph_error::<RDF>("Error adding the info to the test result"))?;
        }

        if let Some(pointer) = &self.pointer {
            let pointer: RDF::Term = pointer.clone().into();
            writer
                .add_triple(node.clone(), EarlVocab::earl_pointer(), pointer)
                .map_err(graph_error::<RDF>("Error adding the pointer to the test result"))?;
        }

        if let Some(date) = &self.date {
            let date: RDF::Term = Object::literal(ConcreteLiteral::datetime(date.clone())).into();
            writer
                .add_triple(node.clone(), DctVocab::dct_date(), date)
                .map_err(graph_error::<RDF>("Error adding the date to the test result"))?;
        }

        for (predicate, object) in &self.extra {
            let object: RDF::Term = object.clone().into();
            writer
                .add_triple(node.clone(), predicate.clone(), object)
                .map_err(graph_error::<RDF>("Error adding an extra triple to the test result"))?;
        }

        Ok(())
    }
}

use crate::{Result, Rudof, api::data::DataOperations, formats::IriNormalizationMode, types::Triples};

/// Builder for `triples` operation.
///
/// Every position left unset matches any term, so a builder with none of them set
/// walks the whole graph.
pub struct TriplesBuilder<'a> {
    rudof: &'a Rudof,
    subject: Option<&'a str>,
    predicate: Option<&'a str>,
    object: Option<&'a str>,
    iri_mode: IriNormalizationMode,
}

impl<'a> TriplesBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::triples()` and should not be constructed
    /// directly.
    pub(crate) fn new(rudof: &'a Rudof) -> Self {
        Self {
            rudof,
            subject: None,
            predicate: None,
            object: None,
            iri_mode: IriNormalizationMode::default(),
        }
    }

    pub fn with_subject(mut self, subject: &'a str) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn with_predicate(mut self, predicate: &'a str) -> Self {
        self.predicate = Some(predicate);
        self
    }

    pub fn with_object(mut self, object: &'a str) -> Self {
        self.object = Some(object);
        self
    }

    pub fn with_iri_mode(mut self, iri_mode: IriNormalizationMode) -> Self {
        self.iri_mode = iri_mode;
        self
    }

    /// Executes the operation, returning the triples that match the pattern.
    pub fn execute(self) -> Result<Triples<'a>> {
        <Rudof as DataOperations>::triples(self.rudof, self.subject, self.predicate, self.object, self.iri_mode)
    }
}

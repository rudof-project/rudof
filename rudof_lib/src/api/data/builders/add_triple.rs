use crate::{Result, Rudof, api::data::DataOperations, formats::IriNormalizationMode};

/// Builder for `add_triple` operation.
pub struct AddTripleBuilder<'a> {
    rudof: &'a mut Rudof,
    subject: &'a str,
    predicate: &'a str,
    object: &'a str,
    iri_mode: IriNormalizationMode,
}

impl<'a> AddTripleBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::add_triple()` and should not be
    /// constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, subject: &'a str, predicate: &'a str, object: &'a str) -> Self {
        Self {
            rudof,
            subject,
            predicate,
            object,
            iri_mode: IriNormalizationMode::default(),
        }
    }

    pub fn with_iri_mode(mut self, iri_mode: IriNormalizationMode) -> Self {
        self.iri_mode = iri_mode;
        self
    }

    /// Executes the operation, adding the triple to the current RDF data.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::add_triple(self.rudof, self.subject, self.predicate, self.object, self.iri_mode)
    }
}

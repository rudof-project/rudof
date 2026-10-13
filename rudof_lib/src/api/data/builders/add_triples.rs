use crate::{Result, Rudof, api::data::DataOperations, formats::IriNormalizationMode};

/// Builder for `add_triples` operation.
pub struct AddTriplesBuilder<'a> {
    rudof: &'a mut Rudof,
    triples: &'a [(&'a str, &'a str, &'a str)],
    iri_mode: IriNormalizationMode,
}

impl<'a> AddTriplesBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::add_triples()` and should not be
    /// constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, triples: &'a [(&'a str, &'a str, &'a str)]) -> Self {
        Self {
            rudof,
            triples,
            iri_mode: IriNormalizationMode::default(),
        }
    }

    pub fn with_iri_mode(mut self, iri_mode: IriNormalizationMode) -> Self {
        self.iri_mode = iri_mode;
        self
    }

    /// Executes the operation, adding the triples to the current RDF data.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::add_triples(self.rudof, self.triples, self.iri_mode)
    }
}

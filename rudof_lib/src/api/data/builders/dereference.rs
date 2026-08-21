use crate::{Result, Rudof, api::data::DataOperations, formats::DataReaderMode};

/// Builder for the `dereference` operation.
///
/// Provides a fluent interface for configuring and executing HTTP
/// dereferencing of a URI into RDF data.
#[derive(Debug)]
pub struct DereferenceBuilder<'a> {
    rudof: &'a mut Rudof,
    uri: &'a str,
    reader_mode: Option<&'a DataReaderMode>,
    merge: Option<bool>,
}

impl<'a> DereferenceBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::dereference()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, uri: &'a str) -> Self {
        Self {
            rudof,
            uri,
            reader_mode: None,
            merge: None,
        }
    }

    /// Sets the reader mode for parsing the dereferenced data.
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Sets whether to merge the dereferenced data with existing data.
    pub fn with_merge(mut self, merge: bool) -> Self {
        self.merge = Some(merge);
        self
    }

    /// Executes the dereference operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::dereference(self.rudof, self.uri, self.reader_mode, self.merge)
    }
}

use crate::{
    Result, Rudof,
    api::data::DataOperations,
    formats::{DataFormat, DataReaderMode, InputSpec},
    types::{DataDiff, GraphRelation, SubgraphMode},
};

/// Builder for `compare_data` operation.
///
/// Compares two RDF graphs and returns their [`GraphRelation`]: isomorphic,
/// equivalent, subgraph, supergraph or different (up to blank node renaming).
/// The first graph is either read from an input or the loaded data.
pub struct CompareDataBuilder<'a> {
    rudof: &'a Rudof,
    data1: Option<&'a InputSpec>,
    data2: &'a InputSpec,
    format1: Option<&'a DataFormat>,
    format2: Option<&'a DataFormat>,
    base1: Option<&'a str>,
    base2: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
    mode: Option<&'a SubgraphMode>,
    exact_search_limit: Option<usize>,
}

impl<'a> CompareDataBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::compare_data()` and
    /// `Rudof::compare_loaded_data()` (with no `data1`) and should not be
    /// constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, data1: Option<&'a InputSpec>, data2: &'a InputSpec) -> Self {
        Self {
            rudof,
            data1,
            data2,
            format1: None,
            format2: None,
            base1: None,
            base2: None,
            reader_mode: None,
            mode: None,
            exact_search_limit: None,
        }
    }

    /// Sets the format of the first graph (Turtle by default). Ignored for the loaded data.
    pub fn with_format1(mut self, format1: &'a DataFormat) -> Self {
        self.format1 = Some(format1);
        self
    }

    /// Sets the format of the second graph (Turtle by default).
    pub fn with_format2(mut self, format2: &'a DataFormat) -> Self {
        self.format2 = Some(format2);
        self
    }

    /// Sets the format of both graphs.
    pub fn with_format(self, format: &'a DataFormat) -> Self {
        self.with_format1(format).with_format2(format)
    }

    /// Sets the base IRI used to resolve relative IRIs in the first graph. Ignored for the loaded data.
    pub fn with_base1(mut self, base1: &'a str) -> Self {
        self.base1 = Some(base1);
        self
    }

    /// Sets the base IRI used to resolve relative IRIs in the second graph.
    pub fn with_base2(mut self, base2: &'a str) -> Self {
        self.base2 = Some(base2);
        self
    }

    /// Sets the reader mode (strict or lax) used to parse both graphs.
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Sets how blank nodes may be mapped when checking whether one graph is
    /// contained in the other ([`SubgraphMode::Injective`] by default).
    pub fn with_mode(mut self, mode: &'a SubgraphMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Sets the maximum number of steps of the search for the best blank node
    /// mapping, used by [`execute_diff`](Self::execute_diff) when neither graph
    /// is contained in the other (100 000 by default; 0 uses the heuristic only).
    pub fn with_exact_search_limit(mut self, limit: usize) -> Self {
        self.exact_search_limit = Some(limit);
        self
    }

    /// Executes the comparison, returning also the triples of each graph that
    /// the other one does not have.
    pub fn execute_diff(self) -> Result<DataDiff> {
        <Rudof as DataOperations>::diff_data(
            self.rudof,
            self.data1,
            self.data2,
            self.format1,
            self.format2,
            self.base1,
            self.base2,
            self.reader_mode,
            self.mode,
            self.exact_search_limit,
        )
    }

    /// Executes the comparison.
    pub fn execute(self) -> Result<GraphRelation> {
        <Rudof as DataOperations>::compare_data(
            self.rudof,
            self.data1,
            self.data2,
            self.format1,
            self.format2,
            self.base1,
            self.base2,
            self.reader_mode,
            self.mode,
        )
    }
}

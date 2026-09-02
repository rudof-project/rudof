use crate::{Result, Rudof, api::data::DataOperations, formats::ResultDataFormat};
use rudof_viz::VizEngine;
use std::io;

/// Builder for `serialize_data` operation.
///
/// Provides a fluent interface for configuring and executing data serialization
/// operations with optional parameters.
pub struct SerializeDataBuilder<'a, W: io::Write> {
    rudof: &'a mut Rudof,
    writer: &'a mut W,
    result_data_format: Option<&'a ResultDataFormat>,
    viz_engine: Option<&'a VizEngine>,
}

impl<'a, W: io::Write> SerializeDataBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_data()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            result_data_format: None,
            viz_engine: None,
        }
    }

    /// Sets the output format for serialization.
    ///
    /// # Arguments
    ///
    /// * `result_data_format` - The format to use when serializing the data
    pub fn with_result_data_format(mut self, result_data_format: &'a ResultDataFormat) -> Self {
        self.result_data_format = Some(result_data_format);
        self
    }

    /// Sets the visualization engine used when the result format is an image (SVG/PNG).
    ///
    /// # Arguments
    ///
    /// * `viz_engine` - The engine to render images with (defaults to `VizEngine::PlantUml`)
    pub fn with_viz_engine(mut self, viz_engine: &'a VizEngine) -> Self {
        self.viz_engine = Some(viz_engine);
        self
    }

    /// Executes the data serialization operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::serialize_data(self.rudof, self.result_data_format, self.viz_engine, self.writer)
    }
}

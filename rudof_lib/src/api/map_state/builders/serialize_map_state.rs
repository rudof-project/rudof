use crate::{Result, Rudof, api::map_state::MapStateOperations};
use std::io;

/// Builder for `serialize_map_state` operation.
///
/// Provides a fluent interface for configuring and executing data serialization
/// operations with optional parameters.
pub struct SerializeMapStateBuilder<'a, W: io::Write> {
    rudof: &'a mut Rudof,
    writer: &'a mut W,
}

impl<'a, W: io::Write> SerializeMapStateBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_data()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, writer: &'a mut W) -> Self {
        Self { rudof, writer }
    }

    /// Executes the map state serialization operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as MapStateOperations>::serialize_map_state(self.rudof, self.writer)
    }
}

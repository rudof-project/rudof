use crate::{Result, Rudof, api::map_state::implementations::serialize_map_state};
use std::io;

/// Operations for accessing and serializing ShEx Map semantic action state.
pub trait MapStateOperations {
    /// Serializes the current map state to a writer as pretty-printed JSON.
    ///
    /// # Arguments
    ///
    /// * `writer` - The destination to write the serialized map state to
    ///
    /// # Errors
    ///
    /// Returns an error if no map state is available or serialization fails.
    fn serialize_map_state<W: io::Write>(&self, writer: &mut W) -> Result<()>;
}

impl MapStateOperations for Rudof {
    fn serialize_map_state<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        serialize_map_state(self, writer)
    }
}

use crate::{
    Result, Rudof,
    api::map_state::implementations::{load_map_state, serialize_map_state},
};
use std::{io, path::Path};

/// Operations for accessing and serializing ShEx Map semantic action state.
pub trait MapStateOperations {
    /// Loads a MapState from a JSON file at `path` and stores it in the current context.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON file containing the serialized MapState
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the JSON cannot be deserialized.
    fn load_map_state(&mut self, path: &Path) -> Result<()>;

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
    fn load_map_state(&mut self, path: &Path) -> Result<()> {
        load_map_state(self, path)
    }

    fn serialize_map_state<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        serialize_map_state(self, writer)
    }
}

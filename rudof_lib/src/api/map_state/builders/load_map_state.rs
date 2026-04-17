use crate::{Result, Rudof, api::map_state::MapStateOperations};
use std::path::Path;

/// Builder for `load_map_state` operation.
///
/// Provides a fluent interface for loading a MapState from a JSON file.
pub struct LoadMapStateBuilder<'a> {
    rudof: &'a mut Rudof,
    path: &'a Path,
}

impl<'a> LoadMapStateBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// Called internally by `Rudof::load_map_state()`; not intended for direct
    /// public construction.
    pub(crate) fn new(rudof: &'a mut Rudof, path: &'a Path) -> Self {
        Self { rudof, path }
    }

    /// Executes the map state loading operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as MapStateOperations>::load_map_state(self.rudof, self.path)
    }
}

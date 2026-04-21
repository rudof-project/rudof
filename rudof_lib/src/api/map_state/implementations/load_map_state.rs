use crate::{Result, Rudof, errors::MapStateError};
use shex_ast::ir::map_state::MapState;
use std::{fs, path::Path};

pub fn load_map_state(rudof: &mut Rudof, path: &Path) -> Result<()> {
    let content = fs::read_to_string(path).map_err(|e| MapStateError::FailedLoadingMapState {
        path: path.display().to_string(),
        error: e.to_string(),
    })?;

    let map_state: MapState =
        serde_json::from_str(&content).map_err(|e| MapStateError::FailedDeserializingMapState {
            path: path.display().to_string(),
            error: e.to_string(),
        })?;

    rudof.map_state = Some(map_state);
    Ok(())
}

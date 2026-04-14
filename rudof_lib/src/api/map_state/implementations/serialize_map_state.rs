use crate::{Result, Rudof, errors::MapStateError};
use std::io;

pub fn serialize_map_state<W: io::Write>(rudof: &Rudof, writer: &mut W) -> Result<()> {
    let map_state = rudof.map_state.as_ref().ok_or(MapStateError::NoMapStateLoaded)?;

    serde_json::to_writer_pretty(writer, map_state)
        .map_err(|e| MapStateError::FailedSerializingMapState { error: e.to_string() })?;

    Ok(())
}

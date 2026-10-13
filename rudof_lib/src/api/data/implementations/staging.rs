use crate::{Result, Rudof, api::data::implementations::begin_staging, types::StagingGuard};

/// Opens a staging frame and returns the guard that closes it.
pub fn staging(rudof: &mut Rudof) -> Result<StagingGuard<'_>> {
    let depth = begin_staging(rudof)?;
    Ok(StagingGuard::new(rudof, depth))
}

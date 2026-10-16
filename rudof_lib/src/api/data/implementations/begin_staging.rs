use crate::{
    Result, Rudof,
    types::StagingFrame,
    utils::{check_data_is_mutable, rdf_data},
};

/// Opens a staging frame over the loaded RDF data, returning the new depth.
pub fn begin_staging(rudof: &mut Rudof) -> Result<usize> {
    // Checked eagerly, at the begin rather than at the first write: a session whose
    // data cannot be changed at all is better told so before it stages anything.
    check_data_is_mutable(rdf_data(rudof)?)?;

    rudof.staging.push(StagingFrame::default());
    Ok(rudof.staging.len())
}

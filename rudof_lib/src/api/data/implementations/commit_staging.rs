use crate::{Result, Rudof, errors::DataError};

/// Keeps the changes staged in the innermost open frame, closing it.
///
/// The changes of a frame nested in another are taken over by the enclosing frame
/// rather than made final, so rolling that one back still undoes them. Committing
/// the outermost frame makes them final: nothing is left to undo them with.
pub fn commit_staging(rudof: &mut Rudof) -> Result<()> {
    let frame = rudof.staging.pop().ok_or_else(|| {
        Box::new(DataError::NoStagingFrame {
            operation: "commit_staging".to_string(),
        })
    })?;

    if let Some(parent) = rudof.staging.last_mut() {
        parent.absorb(frame);
    }

    Ok(())
}

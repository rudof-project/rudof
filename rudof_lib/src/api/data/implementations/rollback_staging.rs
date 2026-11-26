use crate::{
    Result, Rudof,
    api::data::{
        implementations::{apply_additions, apply_removals},
        staging::reset_derived_state,
    },
    errors::DataError,
};

/// Discards the changes staged in the innermost open frame, closing it and putting
/// the loaded data back as it was when the frame was opened.
///
/// Only the data is put back. The loaded schemas, shapemaps, prefixes and
/// configuration are session state, not staged state, and a rollback leaves them
/// exactly as they are.
pub fn rollback_staging(rudof: &mut Rudof) -> Result<()> {
    let frame = rudof.staging.pop().ok_or_else(|| {
        Box::new(DataError::NoStagingFrame {
            operation: "rollback_staging".to_string(),
        })
    })?;

    let (snapshot, mut undo) = frame.into_undo();
    if let Some(data) = snapshot {
        rudof.data = data;
    }

    // The frame is already closed, so whatever happens here leaves no half-undone
    // frame behind. Nothing can happen with the in-memory backend, whose triple
    // writes are infallible, but `BuildRDF` is fallible in general.
    let outcome = undo.try_fold((), |(), entry| {
        let (triples, undo_by_removing) = entry.into_parts();
        if undo_by_removing {
            apply_removals(rudof, triples)?;
        } else {
            apply_additions(rudof, triples)?;
        }
        Ok(())
    });

    // Even a failed rollback leaves data the results no longer describe.
    reset_derived_state(rudof);

    outcome.map_err(|error: crate::errors::RudofError| {
        Box::new(DataError::FailedStagingRollback {
            error: error.to_string(),
        })
        .into()
    })
}

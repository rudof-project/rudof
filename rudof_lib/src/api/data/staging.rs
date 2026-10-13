//! The journal every change to the loaded data is recorded in while a staging frame
//! is open.
//!
//! Each of these is a no-op when nothing is staged, so the ops that change the data
//! call them unconditionally. **Every such op must**: a write that is not journaled
//! here is a write [`rollback_staging`](crate::api::data::DataOperations::rollback_staging)
//! cannot undo, which is why `rdf_data_mut_unjournaled` is named as it is.

use crate::{
    Rudof,
    types::{StagingFrame, UndoEntry},
};
use oxrdf::Triple as OxTriple;

/// Records that `triples` were added to the loaded data, absent from it before, so
/// that rolling the open frame back removes them again.
pub(crate) fn journal_added(rudof: &mut Rudof, triples: Vec<OxTriple>) {
    record(rudof, UndoEntry::Added(triples));
}

/// Records that `triples` were removed from the loaded data, present in it before, so
/// that rolling the open frame back adds them again.
pub(crate) fn journal_removed(rudof: &mut Rudof, triples: Vec<OxTriple>) {
    record(rudof, UndoEntry::Removed(triples));
}

/// Captures what the open frame restores on rollback.
///
/// Call it **before** a mutation that cannot be undone triple by triple (one that
/// replaces or drops the data as a whole, as `load_data`, `dereference` and
/// `reset_data` do) and before anything it may fail on, so the captured state is the
/// one the mutation started from.
pub(crate) fn journal_snapshot(rudof: &mut Rudof) {
    let Some(frame) = current_frame(rudof) else {
        return;
    };
    if frame.has_snapshot() {
        return;
    }

    // Cheap: the in-memory backend's graph is behind an `Arc`, so this clones a
    // handle and leaves the one deep copy to the next write to the live graph.
    let snapshot = rudof.data.clone();
    if let Some(frame) = current_frame(rudof) {
        frame.capture(snapshot);
    }
}

fn record(rudof: &mut Rudof, entry: UndoEntry) {
    if let Some(frame) = current_frame(rudof) {
        frame.record(entry);
    }
}

/// The innermost open frame, the one a rollback closes first and so the one every
/// command is journaled in. `None` when nothing is staged.
fn current_frame(rudof: &mut Rudof) -> Option<&mut StagingFrame> {
    rudof.staging.last_mut()
}

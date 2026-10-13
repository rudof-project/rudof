use crate::types::Data;
use oxrdf::Triple as OxTriple;

/// What has to be done to undo one applied data command.
#[derive(Debug)]
pub(crate) enum UndoEntry {
    /// Triples the command inserted, absent from the data before it ran.
    /// Undo: remove them.
    Added(Vec<OxTriple>),

    /// Triples the command removed, present in the data before it ran.
    /// Undo: add them back.
    Removed(Vec<OxTriple>),
}

impl UndoEntry {
    /// Whether there is nothing to undo, the command having changed no triple.
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            UndoEntry::Added(triples) | UndoEntry::Removed(triples) => triples.is_empty(),
        }
    }
}

/// One open staging frame: what it takes to put the loaded data back as it was when
/// the frame was opened.
///
/// Triple-level commands are undone one by one, so rolling a frame back costs what
/// was staged in it rather than a copy of the whole graph. A command that cannot be
/// inverted (`load_data`, `dereference`, `reset_data`) instead captures `snapshot`,
/// a copy-on-write copy of the session data taken *before* it runs.
#[derive(Debug, Default)]
pub(crate) struct StagingFrame {
    /// The inverses of the triple-level commands applied before `snapshot` was
    /// captured, in application order.
    undo: Vec<UndoEntry>,

    /// The session data as it was immediately before the first command of this frame
    /// that cannot be inverted. The outer `Option` says whether a snapshot was
    /// captured at all, the inner one carries the fact that the data may have been
    /// absent at that moment.
    snapshot: Option<Option<Data>>,
}

impl StagingFrame {
    /// Whether a snapshot has been captured, which makes journaling the commands
    /// applied from now on unnecessary: restoring the snapshot undoes them all.
    pub(crate) fn has_snapshot(&self) -> bool {
        self.snapshot.is_some()
    }

    /// Records the inverse of a triple-level command.
    pub(crate) fn record(&mut self, entry: UndoEntry) {
        if self.has_snapshot() || entry.is_empty() {
            return;
        }
        self.undo.push(entry);
    }

    /// Captures `data` as the state this frame restores, unless it already has one:
    /// the state to go back to is the one *before* the first such command.
    pub(crate) fn capture(&mut self, data: Option<Data>) {
        if !self.has_snapshot() {
            self.snapshot = Some(data);
        }
    }
}

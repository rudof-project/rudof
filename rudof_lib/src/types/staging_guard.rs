use crate::{Result, Rudof, api::data::DataOperations};
use std::ops::{Deref, DerefMut};

/// An open staging frame that closes itself: dropping it rolls the frame back unless
/// it was committed, so a staged change cannot be left half-applied by an early
/// return or an error.
///
/// Returned by [`Rudof::staging`]. It derefs to the [`Rudof`] it was opened on, so
/// the session is used through it as usual.
///
/// Only the data is staged: the schemas and the rest of the session are untouched by
/// a commit and by a rollback alike. See [`DataOperations::begin_staging`].
pub struct StagingGuard<'a> {
    rudof: &'a mut Rudof,

    /// The staging depth this guard owns: the depth right after its own
    /// `begin_staging`. Closing it means closing every frame down to `depth - 1`,
    /// which is what makes the guard harmless when the frames were already closed by
    /// hand through the deref.
    depth: usize,

    finished: bool,
}

impl<'a> StagingGuard<'a> {
    pub(crate) fn new(rudof: &'a mut Rudof, depth: usize) -> Self {
        Self {
            rudof,
            depth,
            finished: false,
        }
    }

    /// Keeps what was staged in this guard's frame, closing it.
    ///
    /// # Errors
    ///
    /// As [`DataOperations::commit_staging`], which this is.
    pub fn commit(mut self) -> Result<()> {
        self.finished = true;
        self.close(true)
    }

    /// Discards what was staged in this guard's frame, closing it and putting the
    /// loaded data back as it was when the guard was opened.
    ///
    /// # Errors
    ///
    /// As [`DataOperations::rollback_staging`], which this is.
    pub fn rollback(mut self) -> Result<()> {
        self.finished = true;
        self.close(false)
    }

    /// Closes every frame still open down to the one below this guard's, committing
    /// or rolling each back. Does nothing when they are already closed.
    fn close(&mut self, commit: bool) -> Result<()> {
        while <Rudof as DataOperations>::staging_depth(self.rudof) >= self.depth {
            if commit {
                <Rudof as DataOperations>::commit_staging(self.rudof)?;
            } else {
                <Rudof as DataOperations>::rollback_staging(self.rudof)?;
            }
        }
        Ok(())
    }
}

impl Deref for StagingGuard<'_> {
    type Target = Rudof;

    fn deref(&self) -> &Self::Target {
        self.rudof
    }
}

impl DerefMut for StagingGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rudof
    }
}

impl Drop for StagingGuard<'_> {
    fn drop(&mut self) {
        if self.finished {
            return;
        }
        // `Drop` cannot report an error, and this one is unreachable with the
        // in-memory backend, whose triple writes are infallible. Logged rather than
        // ignored for the day a fallible mutable backend exists.
        if let Err(error) = self.close(false) {
            tracing::error!("rolling back the staging frame on drop failed: {error}");
        }
    }
}

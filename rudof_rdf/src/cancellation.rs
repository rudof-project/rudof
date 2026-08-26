//! Process-wide cancellation signal for long-running operations (SPARQL
//! requests, ShEx validation steps), so a command that's taking too long can
//! be aborted — e.g. by Ctrl-C in the interactive shell — without killing
//! the whole process.
//!
//! [`request_cancellation`] is safe to call from ordinary code (a plain
//! thread, not a raw OS signal handler) — see `rudof_cli::signals`, which
//! runs a dedicated thread blocking on the signal itself rather than doing
//! any work inside a `sigaction`-installed handler.

use std::sync::atomic::{AtomicBool, Ordering};

static CANCELLED: AtomicBool = AtomicBool::new(false);

/// `true` if cancellation has been requested since the last [`reset`].
pub fn is_cancelled() -> bool {
    CANCELLED.load(Ordering::Relaxed)
}

/// Requests cancellation of whatever long-running operation is in progress.
/// Idempotent.
pub fn request_cancellation() {
    CANCELLED.store(true, Ordering::Relaxed);
}

/// Clears a previously requested cancellation. Call once a command
/// finishes, so the next one (and a later Ctrl-C at an idle prompt) isn't
/// pre-cancelled.
pub fn reset() {
    CANCELLED.store(false, Ordering::Relaxed);
}

/// How often [`cancelled`] re-checks the flag while waiting. Bounds how
/// long an in-flight request/sleep can take to notice a cancellation —
/// short enough to feel immediate, long enough not to matter for CPU usage.
#[cfg(not(target_family = "wasm"))]
const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);

/// Resolves once cancellation is requested; never resolves otherwise.
/// `await` this in a `tokio::select!` alongside a request or sleep to
/// interrupt it, e.g.:
///
/// ```ignore
/// tokio::select! {
///     biased;
///     _ = cancellation::cancelled() => return Err(Cancelled),
///     result = some_future => result,
/// }
/// ```
#[cfg(not(target_family = "wasm"))]
pub async fn cancelled() {
    while !is_cancelled() {
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests share the single process-wide flag, so they run
    // sequentially within this module (the default test harness already
    // does, absent `#[test]`-level parallelism tricks) and each resets
    // what it touched.

    #[test]
    fn starts_and_resets_to_not_cancelled() {
        reset();
        assert!(!is_cancelled());
        request_cancellation();
        assert!(is_cancelled());
        reset();
        assert!(!is_cancelled());
    }

    #[cfg(not(target_family = "wasm"))]
    #[tokio::test]
    async fn cancelled_future_resolves_once_requested() {
        reset();
        request_cancellation();
        // Already cancelled: resolves immediately, no polling wait needed.
        cancelled().await;
        reset();
    }
}

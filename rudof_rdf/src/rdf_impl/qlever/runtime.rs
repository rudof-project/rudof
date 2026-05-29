//! Process-wide Tokio runtime that backs the QLever container.
//!
//! `QleverServer` holds a [`tokio::runtime::Handle`] so its [`Drop`] impl can remove the container asynchronously.
//!
//! Routing every QLever async call through this one [`OnceLock<Runtime>`] guarantees the reactor outlives every `QleverServer` instance.

use std::sync::OnceLock;

use tokio::runtime::Runtime;

static QLEVER_RT: OnceLock<Runtime> = OnceLock::new();

/// Shared multi-thread Tokio runtime used by every QLever operation.
///
/// Built lazily on first call. Lives for the rest of the process so the
/// container's async `Drop` always has a live reactor available.
pub fn runtime() -> &'static Runtime {
    QLEVER_RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("rudof-qlever")
            .build()
            .expect("failed to build rudof QLever Tokio runtime")
    })
}

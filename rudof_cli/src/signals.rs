//! Ctrl-C handling for the interactive shell.
//!
//! Outside the shell, Ctrl-C killing the process outright is normal and left
//! alone. Inside the shell, though, `rustyline` only ever sees Ctrl-C while
//! it's actively reading the next line (terminal in raw mode, so the byte
//! never becomes a real `SIGINT` at all — `rustyline` just maps it to
//! `ReadlineError::Interrupted`). The moment a command starts running, the
//! terminal is back in normal mode, and a real `SIGINT` with no handler
//! installed takes the whole process down mid-command — losing all session
//! state (`data`, `shex`, the active endpoint, ...) along with it.
//!
//! [`install_shell_ctrl_c_handler`] fixes that: the first Ctrl-C during a
//! running command sets [`rudof_rdf::cancellation`]'s flag, which
//! long-running operations (SPARQL requests, ShEx validation steps) check
//! and abort on instead of running to completion or taking the process with
//! them. A second Ctrl-C — a safety net for anything that doesn't check the
//! flag — forces an immediate exit, same as the old unhandled-`SIGINT`
//! behavior.

use anyhow::{Context, Result};

/// Installs the Ctrl-C handler described above. `ctrlc` runs the callback on
/// a dedicated background thread it manages internally, on both Unix and
/// Windows — no polling — so this returns immediately and costs nothing
/// while idle.
///
/// Must be called at most once per process: `ctrlc::set_handler` itself
/// errors out if a handler is already installed.
pub fn install_shell_ctrl_c_handler() -> Result<()> {
    ctrlc::set_handler(|| {
        // The REPL loop clears the flag before dispatching each command
        // (see `repl::run`), so "already cancelled" here means this is at
        // least the second Ctrl-C since then — whatever's running hasn't
        // stopped in response to the first one.
        if rudof_rdf::cancellation::is_cancelled() {
            std::process::exit(130);
        }
        rudof_rdf::cancellation::request_cancellation();
    })
    .context("failed to install Ctrl-C handler")
}

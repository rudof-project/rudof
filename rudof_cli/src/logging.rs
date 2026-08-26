//! Tracing setup for rudof's own diagnostic output (progress, warnings,
//! retries — distinct from command results, which go to stdout/`--output`).
//!
//! `tracing_subscriber::EnvFilter` reads `$RUST_LOG` exactly once, at
//! [`init`], into a filter that's otherwise fixed for the life of the
//! process — setting `$RUST_LOG` afterwards (e.g. from inside the
//! interactive shell) has no effect. [`init`] wraps that filter in a
//! [`reload::Layer`], and stashes a [`reload::Handle`] here, so [`set_level`]
//! can swap it out at runtime instead — this is what backs the shell's
//! `config set logging.level <LEVEL>`.

use std::io;
use std::sync::OnceLock;
use tracing_subscriber::{
    EnvFilter, Registry, filter::ParseError, layer::SubscriberExt, reload, util::SubscriberInitExt,
};

/// Handle to the live tracing filter, set once by [`init`]. `None` in
/// contexts that never call `init` (e.g. library unit tests), in which case
/// [`set_level`] is a no-op.
static RELOAD_HANDLE: OnceLock<reload::Handle<EnvFilter, Registry>> = OnceLock::new();

/// First-party crate names (as `tracing` targets — a crate's lib name, which
/// for every crate here is already a valid target prefix with no `-` to
/// swap for `_`) that are actually linked into the `rudof` binary: the
/// workspace's own crates, minus test/bench-only crates and the
/// Python/Emacs bindings, which aren't. Kept in sync with `[workspace]
/// members` in the repo root `Cargo.toml`.
const RUDOF_CRATES: &[&str] = &[
    "rudof_cli",
    "rudof_lib",
    "rudof_rdf",
    "rudof_config",
    "rudof_iri",
    "rudof_generate",
    "rudof_mcp",
    "shex_ast",
    "shex_validation",
    "shacl",
    "shapes_comparator",
    "shapes_converter",
    "sparql_service",
    "dctap",
    "prefixmap",
    "rbe",
    "rdf_config",
    "mie",
    "pgschema",
];

/// The known bare level names `EnvFilter` accepts, in ascending verbosity —
/// also `logging.level`'s Tab-completion candidates (see
/// `shell::completer`).
pub const LEVEL_NAMES: &[&str] = &["error", "warn", "info", "debug", "trace"];

/// `true` for a single bare level word (one of [`LEVEL_NAMES`],
/// case-insensitively, ignoring surrounding whitespace) — as opposed to a
/// scoped or multi-directive `EnvFilter` string (anything with `=` or `,`
/// in it, e.g. `"rudof_rdf=debug,info"`).
fn is_bare_level(directives: &str) -> bool {
    LEVEL_NAMES.contains(&directives.trim().to_ascii_lowercase().as_str())
}

/// Expands a bare `level` into a filter scoped to just [`RUDOF_CRATES`],
/// with everything else — dependencies: `reqwest`, `hyper`, `rustyline`,
/// tokio's own internals, ... — capped at `warn`. Without this, asking for
/// `debug` to see what rudof itself is doing also turns on `rustyline`'s
/// per-keystroke tracing and every dependency's own internals, which is
/// rarely what's wanted.
fn scoped_directives(level: &str) -> String {
    let level = level.trim();
    let mut directives = String::from("warn");
    for crate_name in RUDOF_CRATES {
        directives.push(',');
        directives.push_str(crate_name);
        directives.push('=');
        directives.push_str(level);
    }
    directives
}

/// Builds the `EnvFilter` for `directives`: a bare level (see
/// [`is_bare_level`]) is scoped to [`RUDOF_CRATES`] via
/// [`scoped_directives`]; anything else — the user's own scoped or
/// multi-directive filter, e.g. `"debug,reqwest=trace"` to opt back into a
/// dependency's own logs — is passed to `EnvFilter` untouched.
fn build_filter(directives: &str) -> Result<EnvFilter, ParseError> {
    if is_bare_level(directives) {
        EnvFilter::try_new(scoped_directives(directives))
    } else {
        EnvFilter::try_new(directives)
    }
}

/// Initializes the global tracing subscriber. The filter starts out as
/// `$RUST_LOG` (falling back to `"info"` if that's unset or invalid), run
/// through [`build_filter`] — but can be changed later via [`set_level`].
///
/// Must be called at most once per process (a second call is a no-op:
/// `tracing`'s global default subscriber can only be installed once).
pub fn init() {
    let requested = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let env_filter =
        build_filter(&requested).unwrap_or_else(|_| EnvFilter::try_new("info").expect("\"info\" is always valid"));

    let (reload_layer, handle) = reload::Layer::new(env_filter.clone());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .with_writer(io::stderr)
        .without_time();

    tracing_subscriber::registry().with(reload_layer).with(fmt_layer).init();

    // Only fails if called twice; `init` documents that it must not be.
    let _ = RELOAD_HANDLE.set(handle);

    tracing::trace!("rudof running with tracing filter {}", env_filter);
}

/// Changes the live tracing filter to `directives` — a bare level like
/// `"debug"` (scoped to rudof's own crates, see [`build_filter`]), or a
/// scoped filter like `"rudof_rdf=debug,info"` — without restarting the
/// process.
///
/// Returns an error if `directives` isn't a valid `EnvFilter` string. A
/// no-op (returns `Ok`) if [`init`] was never called.
pub fn set_level(directives: &str) -> Result<(), ParseError> {
    let filter = build_filter(directives)?;
    if let Some(handle) = RELOAD_HANDLE.get() {
        // Only fails if the subscriber was dropped, which can't happen: the
        // handle and the subscriber it points at live for the process.
        let _ = handle.reload(filter);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_level_is_scoped_to_rudof_crates_with_a_warn_baseline() {
        let directives = scoped_directives("debug");
        assert!(directives.starts_with("warn,"));
        assert!(directives.contains("rudof_cli=debug"));
        assert!(directives.contains("shex_validation=debug"));
    }

    #[test]
    fn bare_level_detection_is_case_insensitive_and_trims_whitespace() {
        assert!(is_bare_level("debug"));
        assert!(is_bare_level("DEBUG"));
        assert!(is_bare_level("  Trace  "));
        assert!(!is_bare_level("rudof_rdf=debug,info"));
        assert!(!is_bare_level("debug,reqwest=trace"));
    }

    #[test]
    fn scoped_and_multi_directive_filters_pass_through_unscoped() {
        // A filter the user wrote themselves — containing '=' or ',' — is
        // left untouched, so they can opt back into a dependency's logs.
        assert!(build_filter("rudof_rdf=debug,info").is_ok());
        assert!(build_filter("debug,reqwest=trace").is_ok());
    }

    #[test]
    fn every_bare_level_name_builds_a_valid_filter() {
        for level in LEVEL_NAMES {
            assert!(build_filter(level).is_ok(), "expected {level} to build a valid filter");
        }
    }
}

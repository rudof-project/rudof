use rudof_config::TomlConfig;
use serde::{Deserialize, Serialize};

/// Logging configuration: controls the `tracing` filter used for rudof's own
/// diagnostic output (progress, warnings, retries — distinct from command
/// results, which go to stdout / `--output`).
///
/// Applied at startup (see the CLI's `logging` module), and — inside the
/// interactive shell — can be changed for the rest of the session via
/// `config set logging.level <LEVEL>`, without restarting the process.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct LoggingConfig {
    /// A `tracing_subscriber::EnvFilter` directive string: a bare level
    /// (`"error"`, `"warn"`, `"info"`, `"debug"`, `"trace"`) or a scoped
    /// filter such as `"rudof_rdf=debug,info"`.
    ///
    /// Empty (the default) defers to the `RUST_LOG` environment variable,
    /// falling back to `"info"` if that's unset too. Kept as a plain
    /// (rather than `Option`) string, unlike most other optional settings in
    /// this config: `toml` can't serialize `None`, so an `Option` field is
    /// normally omitted from the TOML tree entirely until first set — but
    /// `config get`/`config set`'s Tab-completion (and `config get` itself)
    /// only ever see keys already present in that tree, which would leave
    /// `logging.level` undiscoverable in a fresh shell session.
    #[serde(rename = "level")]
    pub(crate) level: String,
}

/// Constructor and setters
impl LoggingConfig {
    /// Creates a new [`LoggingConfig`].
    pub fn new() -> Self {
        Self {
            level: Self::default_level(),
        }
    }

    /// Sets `level` and returns itself. `None` clears it back to deferring
    /// to `$RUST_LOG`.
    pub fn with_level(mut self, level: Option<String>) -> Self {
        self.level = level.unwrap_or_default();
        self
    }
}

/// Accessor methods
impl LoggingConfig {
    /// Returns the configured filter directive, or `None` if unset (in
    /// which case `$RUST_LOG` applies, falling back to `"info"`).
    pub fn level(&self) -> Option<&str> {
        if self.level.is_empty() { None } else { Some(&self.level) }
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl LoggingConfig {
    #[inline] fn default_level() -> String { String::new() }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlConfig for LoggingConfig {}

#[cfg(test)]
mod tests {
    use super::LoggingConfig;
    use rudof_config::TomlConfig;

    #[test]
    fn defaults() {
        let c = LoggingConfig::default();
        assert_eq!(c.level(), None);
    }

    #[test]
    fn partial_toml_fills_remaining_defaults() {
        let c = LoggingConfig::from_toml_str(r#"level = "debug""#).unwrap();
        assert_eq!(c.level(), Some("debug"));
    }

    #[test]
    fn toml_round_trip() {
        let c = LoggingConfig::default().with_level(Some("trace".to_string()));
        let s = c.to_toml_string().unwrap();
        let d = LoggingConfig::from_toml_str(&s).unwrap();
        assert_eq!(c, d);
    }

    #[test]
    fn empty_level_is_always_present_in_the_toml_tree() {
        // Unlike an `Option` field (which `toml` can't serialize as `None`
        // and so omits entirely), the default empty string still appears —
        // this is what makes `logging.level` discoverable via `config get`/
        // Tab-completion before it's ever been set.
        let s = LoggingConfig::default().to_toml_string().unwrap();
        assert!(s.contains("level"));
    }
}

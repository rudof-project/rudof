//! Docker state persistence for ephemeral container support.
//!
//! This module provides state persistence for MCP servers running in Docker containers
//! that are created ephemerally (one container per tool call). State is persisted to
//! a Docker volume mounted at `/app/state/` to maintain data across container restarts.
//!
//! # Usage
//!
//! The MCP server will:
//! 1. On startup: Load persisted state from `/app/state/data.json` if it exists
//! 2. After state-modifying tools: Save state to `/app/state/data.json`

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Default path for state persistence in Docker containers.
pub const DEFAULT_STATE_PATH: &str = "/app/state/data.json";

/// Environment variable to override the state file path.
pub const STATE_PATH_ENV_VAR: &str = "RUDOF_MCP_STATE_PATH";

/// Persisted state structure for the MCP server.
///
/// This structure captures the essential state that needs to survive
/// across ephemeral container restarts.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersistedState {
    /// Version of the state format for future migrations
    pub version: u32,

    /// Serialized RDF data in N-Triples format (portable and simple)
    #[serde(default)]
    pub rdf_data_ntriples: Option<String>,

    /// Timestamp of last save (ISO 8601 format)
    #[serde(default)]
    pub last_saved: Option<String>,

    /// Number of triples in the persisted data
    #[serde(default)]
    pub triple_count: Option<usize>,
}

impl PersistedState {
    /// Current version of the state format.
    pub const CURRENT_VERSION: u32 = 1;

    /// Create a new empty state.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            rdf_data_ntriples: None,
            last_saved: None,
            triple_count: None,
        }
    }

    /// Create state with RDF data.
    pub fn with_rdf_data(rdf_data_ntriples: String, triple_count: usize) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            rdf_data_ntriples: Some(rdf_data_ntriples),
            last_saved: Some(chrono::Utc::now().to_rfc3339()),
            triple_count: Some(triple_count),
        }
    }

    /// Check if the state has any RDF data.
    #[allow(dead_code)]
    pub fn has_rdf_data(&self) -> bool {
        self.rdf_data_ntriples
            .as_ref()
            .is_some_and(|s| !s.is_empty())
    }
}

/// Get the state file path from environment or use default.
pub fn get_state_path() -> PathBuf {
    std::env::var(STATE_PATH_ENV_VAR)
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_STATE_PATH))
}

/// Check if the state directory exists and is writable.
pub fn is_persistence_available() -> bool {
    let state_path = get_state_path();
    if let Some(parent) = state_path.parent() {
        parent.exists() && parent.is_dir()
    } else {
        false
    }
}

/// Load persisted state from the state file.
///
/// Returns `None` if:
/// - The state file doesn't exist
/// - The file cannot be read
/// - The JSON is malformed
///
/// Logs warnings for recoverable errors and continues with empty state.
pub fn load_state() -> Option<PersistedState> {
    let state_path = get_state_path();

    if !state_path.exists() {
        debug!(
            "State file not found at {:?}, starting with empty state",
            state_path
        );
        return None;
    }

    match load_state_from_path(&state_path) {
        Ok(state) => {
            info!(
                "Loaded persisted state from {:?} (version: {}, triples: {:?}, last_saved: {:?})",
                state_path, state.version, state.triple_count, state.last_saved
            );
            Some(state)
        }
        Err(e) => {
            warn!(
                "Failed to load state from {:?}: {}. Starting with empty state.",
                state_path, e
            );
            None
        }
    }
}

/// Load state from a specific path.
pub fn load_state_from_path(path: &Path) -> io::Result<PersistedState> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let state: PersistedState = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(state)
}

/// Save state to the state file.
///
/// Creates parent directories if they don't exist.
/// Returns an error if the save fails.
pub fn save_state(state: &PersistedState) -> io::Result<()> {
    let state_path = get_state_path();
    save_state_to_path(state, &state_path)
}

/// Save state to a specific path.
pub fn save_state_to_path(state: &PersistedState, path: &Path) -> io::Result<()> {
    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let file = fs::File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, state)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    info!(
        "Saved state to {:?} (triples: {:?})",
        path, state.triple_count
    );
    Ok(())
}

/// Error type for state persistence operations.
#[derive(Debug)]
pub enum StatePersistenceError {
    /// IO error during file operations
    IoError(io::Error),
    /// JSON serialization/deserialization error
    JsonError(String),
    /// RDF serialization error
    RdfSerializationError(String),
    /// RDF parsing error
    RdfParseError(String),
}

impl std::fmt::Display for StatePersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::JsonError(e) => write!(f, "JSON error: {}", e),
            Self::RdfSerializationError(e) => write!(f, "RDF serialization error: {}", e),
            Self::RdfParseError(e) => write!(f, "RDF parse error: {}", e),
        }
    }
}

impl std::error::Error for StatePersistenceError {}

impl From<io::Error> for StatePersistenceError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_persisted_state_new() {
        let state = PersistedState::new();
        assert_eq!(state.version, PersistedState::CURRENT_VERSION);
        assert!(state.rdf_data_ntriples.is_none());
        assert!(!state.has_rdf_data());
    }

    #[test]
    fn test_persisted_state_with_rdf_data() {
        let rdf = "<http://example.org/s> <http://example.org/p> <http://example.org/o> .\n";
        let state = PersistedState::with_rdf_data(rdf.to_string(), 1);
        assert!(state.has_rdf_data());
        assert_eq!(state.triple_count, Some(1));
        assert!(state.last_saved.is_some());
    }

    #[test]
    fn test_save_and_load_state() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let rdf = "<http://example.org/s> <http://example.org/p> <http://example.org/o> .\n";
        let state = PersistedState::with_rdf_data(rdf.to_string(), 1);

        // Save
        save_state_to_path(&state, &path).unwrap();

        // Load
        let loaded = load_state_from_path(&path).unwrap();
        assert_eq!(loaded.version, state.version);
        assert_eq!(loaded.rdf_data_ntriples, state.rdf_data_ntriples);
        assert_eq!(loaded.triple_count, state.triple_count);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/path/state.json");
        let result = load_state_from_path(&path);
        assert!(result.is_err());
    }
}

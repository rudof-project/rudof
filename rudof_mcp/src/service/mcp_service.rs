//! Core service implementation for the Rudof MCP server.
//!
//! This module contains the main [`RudofMcpService`] struct which implements
//! the MCP `ServerHandler` trait and manages all server state.

use std::{path::PathBuf, sync::Arc};
use tokio::sync::{Mutex, RwLock};

use crate::service::tools::helpers::RDF_FORMAT_LIST;
use crate::service::{logging::LogRateLimiter, state};
// `LoggingLevel` is deprecated by SEP-2577; the protocol still supports
// MCP logging and there is no replacement API, so it stays in use here.
#[allow(deprecated)]
use rmcp::{
    handler::server::router::{prompt::PromptRouter, tool::ToolRouter},
    model::LoggingLevel,
};
use rudof_lib::{
    Rudof, RudofConfig,
    formats::{DataFormat, InputSpec, ResultDataFormat},
};

/// Errors that can occur when creating a [`RudofMcpService`].
///
/// These errors typically indicate configuration issues or problems
/// initializing the underlying Rudof library.
#[derive(Debug)]
pub enum ServiceCreationError {
    /// Failed to initialize the Rudof instance.
    ///
    /// This may occur if there are issues loading default prefixes
    /// or other initialization steps.
    RudofError(String),
}

impl std::fmt::Display for ServiceCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RudofError(e) => write!(f, "Failed to initialize Rudof: {}", e),
        }
    }
}

impl std::error::Error for ServiceCreationError {}

/// The main MCP service for Rudof operations.
///
/// This struct implements the [`rmcp::ServerHandler`] trait and serves as the
/// central coordinator for all MCP protocol interactions. It manages:
///
/// - **Tool execution**: Routes tool calls to appropriate handlers
/// - **Prompt templates**: Provides guided interaction templates for common workflows
/// - **Resource access**: Exposes RDF data through MCP's resource protocol
/// - **Completions**: Provides context-aware autocompletion suggestions for
///   prompt arguments (formats, IRIs, shape labels) and resource URI templates
/// - **Logging**: Sends structured log messages to MCP clients with
///   configurable severity filtering (RFC 5424 levels)
///
/// # Thread Safety
///
/// The service is designed to be cloned and shared across async tasks.
/// All mutable state is protected by `Arc<Mutex<_>>` or `Arc<RwLock<_>>`.
/// Request-scoped context is carried via task-local storage to avoid cross-request contamination.
#[derive(Clone)]
pub struct RudofMcpService {
    /// Core Rudof instance wrapped in async-safe synchronization.
    pub rudof: Arc<Mutex<Rudof>>,

    /// Router for dispatching tool calls to handler functions.
    ///
    /// Built at service creation from the tool definitions in the `tools` module.
    /// Uses rmcp's macro-generated routing for type-safe parameter handling.
    pub tool_router: ToolRouter<RudofMcpService>,

    /// Router for dispatching prompt requests to handler functions.
    ///
    /// Built at service creation from prompt definitions in the `prompts` module.
    pub prompt_router: PromptRouter<RudofMcpService>,

    /// Current minimum log level for MCP logging notifications.
    ///
    /// Set via `logging/setLevel` requests. Only log messages at or
    /// above this severity level are sent to the client.
    #[allow(deprecated)]
    pub current_min_log_level: Arc<RwLock<Option<LoggingLevel>>>,

    /// Rate limiter for outbound MCP logging notifications.
    ///
    /// Helps avoid flooding clients with `notifications/message`.
    pub(crate) log_rate_limiter: Arc<Mutex<LogRateLimiter>>,

    /// This session's virtual working directory, used to resolve relative local
    /// file paths passed to tools (schema/data/shapes arguments, ...).
    ///
    /// Deliberately *not* backed by `std::env::set_current_dir`: under the
    /// streamable-HTTP transport many sessions (each with their own
    /// `RudofMcpService`) can run concurrently inside one OS process, and the
    /// process's current directory is shared global state — one session's `cd`
    /// would otherwise silently redirect every other session's relative paths.
    /// Starts at the server process's actual cwd and is only ever read/written
    /// through this field; see [`tools::resolve_input_spec`](crate::service::tools::helpers::resolve_input_spec)
    /// for how tools resolve paths against it.
    pub session_dir: Arc<RwLock<PathBuf>>,
}

impl RudofMcpService {
    /// Create a new RudofMcpService instance.
    ///
    /// # Panics
    /// Panics if Rudof configuration or initialization fails.
    /// For fallible construction, use [`try_new`](Self::try_new).
    pub fn new() -> Self {
        Self::try_new().expect("Failed to create RudofMcpService")
    }

    /// Try to create a new RudofMcpService instance.
    ///
    /// This method will:
    /// 1. Initialize the Rudof instance with default configuration
    /// 2. Attempt to load persisted state from `/app/state/data.json` (for Docker containers)
    ///
    /// Returns an error if Rudof configuration or initialization fails.
    /// State loading failures are logged but don't prevent service creation.
    pub fn try_new() -> Result<Self, ServiceCreationError> {
        let rudof_config = RudofConfig::new();
        let mut rudof = Rudof::new(rudof_config);

        // Attempt to load persisted state (for Docker ephemeral containers)
        if let Some(persisted_state) = state::load_state()
            && persisted_state.has_rdf_data()
            && let Some(rdf_ntriples) = &persisted_state.rdf_data_ntriples
        {
            tracing::info!(
                "Restoring {} triples from persisted state",
                persisted_state.triple_count.unwrap_or(0)
            );
            // Load persisted N-Triples string into Rudof using an in-memory InputSpec
            let spec = InputSpec::Str(rdf_ntriples.clone());
            let data_specs = vec![spec];
            if let Err(e) = rudof
                .load_data()
                .with_data(&data_specs)
                .with_data_format(&DataFormat::NTriples)
                .execute()
            {
                tracing::warn!("Failed to restore persisted RDF data: {}", e);
            } else {
                tracing::info!("Successfully restored RDF data from persisted state");
            }
        }

        let session_dir = std::env::current_dir().map_err(|e| ServiceCreationError::RudofError(e.to_string()))?;

        Ok(Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
            current_min_log_level: Arc::new(RwLock::new(None)),
            log_rate_limiter: Arc::new(Mutex::new(LogRateLimiter::default())),
            session_dir: Arc::new(RwLock::new(session_dir)),
        })
    }

    /// Count the number of triples in an N-Triples string.
    ///
    /// Skips empty lines and comment lines (starting with `#`).
    pub fn count_triples_in_ntriples(ntriples: &str) -> usize {
        ntriples
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })
            .count()
    }

    /// Persist the current RDF data state to the state file.
    ///
    /// This method is called after state-modifying operations to ensure
    /// data survives Docker container restarts. Only saves if persistence
    /// is available (i.e., the state directory exists).
    ///
    /// If `prebuilt_ntriples` is provided the Rudof lock is not re-acquired
    /// and the data is not re-serialized — this avoids a redundant round-trip
    /// when the caller has already serialized the data (e.g., for triple counting).
    ///
    /// # Returns
    /// - `Ok(true)` if state was saved successfully
    /// - `Ok(false)` if persistence is not available (no state directory)
    /// - `Err` if an error occurred during saving
    pub async fn persist_state_with(
        &self,
        prebuilt_ntriples: Option<String>,
    ) -> Result<bool, state::StatePersistenceError> {
        if !state::is_persistence_available() {
            tracing::debug!("State persistence not available (no state directory)");
            return Ok(false);
        }

        let rdf_ntriples = if let Some(ntriples) = prebuilt_ntriples {
            ntriples
        } else {
            let mut rudof = self.rudof.lock().await;
            let mut v = Vec::new();
            rudof
                .serialize_data(&mut v)
                .with_result_data_format(&ResultDataFormat::NTriples)
                .execute()
                .map_err(|e| state::StatePersistenceError::RdfSerialization(e.to_string()))?;
            String::from_utf8(v).map_err(|e| state::StatePersistenceError::Json(e.to_string()))?
        };

        let triple_count = Self::count_triples_in_ntriples(&rdf_ntriples);
        let persisted_state = state::PersistedState::with_rdf_data(rdf_ntriples, triple_count);
        state::save_state(&persisted_state)?;

        tracing::info!("Persisted {} triples to state file", triple_count);
        Ok(true)
    }

    /// Persist the current RDF data state to the state file.
    ///
    /// Convenience wrapper around [`persist_state_with`](Self::persist_state_with)
    /// that always re-serializes from the Rudof engine.
    pub async fn persist_state(&self) -> Result<bool, state::StatePersistenceError> {
        self.persist_state_with(None).await
    }

    /// Get completion suggestions for prompt arguments
    pub fn get_prompt_argument_completions(&self, prompt_name: &str, argument_name: &str) -> Vec<String> {
        tracing::debug!(
            prompt_name = %prompt_name,
            argument_name = %argument_name,
            "Getting prompt argument completions"
        );

        match (prompt_name, argument_name) {
            // Base IRI suggestions
            (_, "base") | (_, "base_iri") => {
                vec![
                    "http://example.org/".to_string(),
                    "https://schema.org/".to_string(),
                    "http://www.w3.org/2001/XMLSchema#".to_string(),
                ]
            },
            // Focus argument for analyze_rdf_data prompt
            ("analyze_rdf_data", "focus") | (_, "focus") => {
                vec![
                    "all".to_string(),
                    "structure".to_string(),
                    "quality".to_string(),
                    "statistics".to_string(),
                ]
            },
            // Technology argument for validation_guide prompt
            ("validation_guide", "technology") | (_, "technology") => {
                vec!["shex".to_string(), "shacl".to_string()]
            },
            _ => vec![],
        }
    }

    /// Get completion suggestions for resource URI templates
    pub fn get_resource_uri_completions(&self, uri: &str, argument_name: &str) -> Vec<String> {
        tracing::debug!(
            uri = %uri,
            argument_name = %argument_name,
            "Getting resource URI completions"
        );

        if !uri.starts_with("rudof://") {
            return vec![];
        }

        match argument_name {
            // The `{format}` of the `rudof://current-data/{format}` template
            "format" => RDF_FORMAT_LIST.iter().map(|s| s.to_string()).collect(),
            "endpoint" => vec![
                "https://query.wikidata.org/sparql".to_string(),
                "https://dbpedia.org/sparql".to_string(),
            ],
            _ => vec![],
        }
    }
}

impl Default for RudofMcpService {
    fn default() -> Self {
        Self::new()
    }
}

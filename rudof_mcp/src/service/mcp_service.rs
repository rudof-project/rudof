//! Core service implementation for the Rudof MCP server.
//!
//! This module contains the main [`RudofMcpService`] struct which implements
//! the MCP `ServerHandler` trait and manages all server state.

use std::{future::Future, sync::Arc};
use tokio::sync::{Mutex, RwLock};

use crate::service::{logging::LogRateLimiter, prompts, state, tools};
use crate::service::tools::helpers::{
    NODE_INFO_MODE_LIST, RDF_FORMAT_LIST, RESULT_FORMAT_LIST, SHACL_FORMAT_LIST, SHEX_FORMAT_LIST,
};
use rmcp::{
    RoleServer,
    handler::server::router::{prompt::PromptRouter, tool::ToolRouter},
    model::{LoggingLevel},
    service::RequestContext,
};
use rudof_lib::{
    Rudof, RudofConfig,
    formats::{DataFormat, InputSpec, ResultDataFormat},
};

tokio::task_local! {
    static REQUEST_CONTEXT: RequestContext<RoleServer>;
}

/// Errors that can occur when creating a [`RudofMcpService`].
///
/// These errors typically indicate configuration issues or problems
/// initializing the underlying Rudof library.
#[derive(Debug)]
pub enum ServiceCreationError {
    /// Failed to create Rudof configuration.
    ///
    /// This usually indicates invalid environment settings or missing
    /// configuration files.
    ConfigError(String),

    /// Failed to initialize the Rudof instance.
    ///
    /// This may occur if there are issues loading default prefixes
    /// or other initialization steps.
    RudofError(String),
}

impl std::fmt::Display for ServiceCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigError(e) => write!(f, "Failed to create Rudof configuration: {}", e),
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
    pub current_min_log_level: Arc<RwLock<Option<LoggingLevel>>>,

    /// Rate limiter for outbound MCP logging notifications.
    ///
    /// Helps avoid flooding clients with `notifications/message`.
    pub(crate) log_rate_limiter: Arc<Mutex<LogRateLimiter>>,
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
            && let Some(rdf_ntriples) = &persisted_state.rdf_data_ntriples
            && !rdf_ntriples.is_empty()
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

        Ok(Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
            current_min_log_level: Arc::new(RwLock::new(None)),
            log_rate_limiter: Arc::new(Mutex::new(LogRateLimiter::default())),
        })
    }

    /// Run a future with a request context bound to task-local storage.
    pub async fn with_request_context<F, T>(context: RequestContext<RoleServer>, future: F) -> T
    where
        F: Future<Output = T>,
    {
        REQUEST_CONTEXT.scope(context, future).await
    }

    /// Returns the request context bound to the current async task, if any.
    pub fn current_request_context() -> Option<RequestContext<RoleServer>> {
        REQUEST_CONTEXT.try_with(Clone::clone).ok()
    }

    /// Persist the current RDF data state to the state file.
    ///
    /// This method is called after state-modifying operations to ensure
    /// data survives Docker container restarts. Only saves if persistence
    /// is available (i.e., the state directory exists).
    ///
    /// # Returns
    /// - `Ok(true)` if state was saved successfully
    /// - `Ok(false)` if persistence is not available (no state directory)
    /// - `Err` if an error occurred during saving
    pub async fn persist_state(&self) -> Result<bool, state::StatePersistenceError> {
        if !state::is_persistence_available() {
            tracing::debug!("State persistence not available (no state directory)");
            return Ok(false);
        }

        let mut rudof = self.rudof.lock().await;

        // Serialize RDF data to N-Triples format
        let mut v = Vec::new();
        rudof
            .serialize_data(&mut v)
            .with_result_data_format(&ResultDataFormat::NTriples)
            .execute()
            .map_err(|e| state::StatePersistenceError::RdfSerialization(e.to_string()))?;

        let rdf_ntriples = String::from_utf8(v).map_err(|e| state::StatePersistenceError::Json(e.to_string()))?;

        // Count triples (count lines that aren't empty or comments)
        let triple_count = rdf_ntriples
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })
            .count();

        let persisted_state = state::PersistedState::with_rdf_data(rdf_ntriples, triple_count);
        state::save_state(&persisted_state)?;

        tracing::info!("Persisted {} triples to state file", triple_count);
        Ok(true)
    }

    /// Get completion suggestions for format-related arguments (shared by prompts and tools).
    pub fn get_format_argument_completions(argument_name: &str) -> Vec<String> {
        let list: &[&str] = match argument_name {
            "format" | "input_format" | "output_format" | "rdf_format" => RDF_FORMAT_LIST,
            "schema_format" | "shex_format" => SHEX_FORMAT_LIST,
            "shacl_format" | "shapes_format" => SHACL_FORMAT_LIST,
            "result_format" => RESULT_FORMAT_LIST,
            "mode" => NODE_INFO_MODE_LIST,
            _ => return vec![],
        };
        list.iter().map(|s| s.to_string()).collect()
    }

    /// Get completion suggestions for prompt arguments
    pub fn get_prompt_argument_completions(&self, prompt_name: &str, argument_name: &str) -> Vec<String> {
        tracing::debug!(
            prompt_name = %prompt_name,
            argument_name = %argument_name,
            "Getting prompt argument completions"
        );

        // Delegate format arguments to the shared helper first
        let format_completions = Self::get_format_argument_completions(argument_name);
        if !format_completions.is_empty() {
            return format_completions;
        }

        match (prompt_name, argument_name) {
            // Common boolean arguments
            (_, "verbose") | (_, "debug") | (_, "strict") => {
                vec!["true".to_string(), "false".to_string()]
            },
            // Base IRI suggestions
            (_, "base") | (_, "base_iri") => {
                vec![
                    "http://example.org/".to_string(),
                    "https://schema.org/".to_string(),
                    "http://www.w3.org/2001/XMLSchema#".to_string(),
                ]
            },
            // Shape label suggestions (common patterns)
            (_, "shape") | (_, "shape_label") | (_, "start_shape") => {
                vec![
                    ":Person".to_string(),
                    ":Thing".to_string(),
                    ":Organization".to_string(),
                    "schema:Person".to_string(),
                    "foaf:Person".to_string(),
                ]
            },
            // Node selector suggestions
            (_, "node") | (_, "focus_node") => {
                vec![":node1".to_string(), "<http://example.org/resource>".to_string()]
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
            // Query type argument for sparql_builder prompt (from rudof://formats/query-types)
            ("sparql_builder", "query_type") | (_, "query_type") => {
                vec![
                    "select".to_string(),
                    "construct".to_string(),
                    "ask".to_string(),
                    "describe".to_string(),
                ]
            },
            _ => vec![],
        }
    }

    /// Get completion suggestions for tool arguments.
    ///
    /// Tool argument names follow the same conventions as prompt argument names,
    /// so this delegates to the shared format helper for format-related arguments.
    pub fn get_tool_argument_completions(&self, _tool_name: &str, argument_name: &str) -> Vec<String> {
        tracing::debug!(
            tool_name = %_tool_name,
            argument_name = %argument_name,
            "Getting tool argument completions"
        );
        Self::get_format_argument_completions(argument_name)
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
            "format" => RDF_FORMAT_LIST.iter().map(|s| s.to_string()).collect(),
            "shex_format" | "schema_format" => SHEX_FORMAT_LIST.iter().map(|s| s.to_string()).collect(),
            "shacl_format" | "shapes_format" => SHACL_FORMAT_LIST.iter().map(|s| s.to_string()).collect(),
            "mode" => NODE_INFO_MODE_LIST.iter().map(|s| s.to_string()).collect(),
            // SPARQL endpoint suggestions
            "endpoint" => vec![
                "https://query.wikidata.org/sparql".to_string(),
                "https://dbpedia.org/sparql".to_string(),
                "http://localhost:3030/sparql".to_string(),
            ],
            // Query result formats (from rudof://formats/query-results)
            "result_format" => vec![
                "internal".to_string(),
                "json".to_string(),
                "xml".to_string(),
                "csv".to_string(),
                "tsv".to_string(),
                "turtle".to_string(),
                "ntriples".to_string(),
                "rdfxml".to_string(),
                "trig".to_string(),
            ],
            // Validation reader modes (from rudof://formats/validation-reader-modes)
            "reader_mode" => vec!["strict".to_string(), "lax".to_string()],
            _ => vec![],
        }
    }
}

impl Default for RudofMcpService {
    fn default() -> Self {
        Self::new()
    }
}

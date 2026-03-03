//! Core service implementation for the Rudof MCP server.
//!
//! This module contains the main [`RudofMcpService`] struct which implements
//! the MCP `ServerHandler` trait and manages all server state.

use std::{collections::HashMap, io::Cursor, sync::Arc};
use tokio::sync::{Mutex, RwLock};

use crate::service::{prompts, state, tasks::TaskStore, tools};
use rmcp::{
    RoleServer,
    handler::server::router::{prompt::PromptRouter, tool::ToolRouter},
    model::{LoggingLevel, ResourceUpdatedNotificationParam},
    service::RequestContext,
};
use rudof_lib::{RDFFormat, ReaderMode, Rudof, RudofConfig};

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
/// - **Tool execution**: Routes tool calls to appropriate handlers for
///   RDF validation, SPARQL queries, and data transformations
/// - **Prompt templates**: Provides guided interaction templates for
///   common validation and query workflows
/// - **Resource access**: Exposes RDF data through MCP's resource protocol
///   with support for multiple serialization formats
/// - **Resource subscriptions**: Clients can subscribe to resource URIs and
///   receive notifications when resources are updated via `notify_resource_updated`
/// - **Completions**: Provides context-aware autocompletion suggestions for
///   prompt arguments (formats, IRIs, shape labels) and resource URI templates
/// - **Logging**: Sends structured log messages to MCP clients with
///   configurable severity filtering (RFC 5424 levels)
/// - **Task management**: Supports async operations for long-running
///   validations (SEP-1686)
///
/// # Thread Safety
///
/// The service is designed to be cloned and shared across async tasks.
/// All mutable state is protected by `Arc<Mutex<_>>` or `Arc<RwLock<_>>`.
/// Each cloned instance maintains its own request context, enabling
/// concurrent tool executions without interference.
#[derive(Clone)]
pub struct RudofMcpService {
    /// Core Rudof instance wrapped in async-safe synchronization.
    ///
    /// This holds the RDF graph, loaded schemas, and validation state.
    /// All operations that read or modify RDF data must acquire this lock.
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

    /// Tracks resource subscriptions by URI.
    ///
    /// Maps resource URIs to lists of subscriber IDs. When a resource
    /// changes, all subscribers receive update notifications.
    pub resource_subscriptions: Arc<RwLock<HashMap<String, Vec<String>>>>,

    /// Current minimum log level for MCP logging notifications.
    ///
    /// Set via `logging/setLevel` requests. Only log messages at or
    /// above this severity level are sent to the client.
    pub current_min_log_level: Arc<RwLock<Option<LoggingLevel>>>,

    /// Request context for the currently executing tool call.
    ///
    /// This enables tools to send notifications (e.g., resource updates,
    /// progress) during execution. Set at the start of `call_tool` and
    /// cleared on completion.
    ///
    /// # Note
    /// Each cloned service instance maintains its own context, enabling
    /// concurrent tool executions without interference.
    pub current_context: Arc<RwLock<Option<RequestContext<RoleServer>>>>,

    /// Store for managing async tasks (SEP-1686).
    ///
    /// Tracks long-running operations like large dataset validations,
    /// allowing clients to poll for status and retrieve results.
    pub task_store: TaskStore,
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
        let rudof_config = RudofConfig::new().map_err(|e| ServiceCreationError::ConfigError(e.to_string()))?;
        let mut rudof = Rudof::new(&rudof_config).map_err(|e| ServiceCreationError::RudofError(e.to_string()))?;

        // Attempt to load persisted state (for Docker ephemeral containers)
        if let Some(persisted_state) = state::load_state()
            && let Some(rdf_ntriples) = &persisted_state.rdf_data_ntriples
            && !rdf_ntriples.is_empty()
        {
            tracing::info!(
                "Restoring {} triples from persisted state",
                persisted_state.triple_count.unwrap_or(0)
            );
            let mut cursor = Cursor::new(rdf_ntriples.as_bytes());
            if let Err(e) = rudof.read_data(
                &mut cursor,
                "persisted_state",
                Some(&RDFFormat::NTriples),
                None,
                Some(&ReaderMode::default()),
                Some(false),
            ) {
                tracing::warn!("Failed to restore persisted RDF data: {}", e);
            } else {
                tracing::info!("Successfully restored RDF data from persisted state");
            }
        }

        Ok(Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
            resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            current_min_log_level: Arc::new(RwLock::new(None)),
            current_context: Arc::new(RwLock::new(None)),
            task_store: TaskStore::new(),
        })
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

        let rudof = self.rudof.lock().await;

        // Serialize RDF data to N-Triples format
        let mut buffer = Vec::new();
        rudof
            .serialize_data(Some(&RDFFormat::NTriples), &mut buffer)
            .map_err(|e| state::StatePersistenceError::RdfSerialization(e.to_string()))?;

        let rdf_ntriples = String::from_utf8(buffer).map_err(|e| state::StatePersistenceError::Json(e.to_string()))?;

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

    /// Add a resource subscription
    pub async fn subscribe_resource(&self, uri: String, subscriber_id: String) {
        tracing::debug!(
            uri = %uri,
            subscriber_id = %subscriber_id,
            "Subscribing to resource"
        );
        let mut subs = self.resource_subscriptions.write().await;
        subs.entry(uri).or_insert_with(Vec::new).push(subscriber_id);
    }

    /// Remove a resource subscription
    pub async fn unsubscribe_resource(&self, uri: &str, subscriber_id: &str) {
        tracing::debug!(
            uri = %uri,
            subscriber_id = %subscriber_id,
            "Unsubscribing from resource"
        );
        let mut subs = self.resource_subscriptions.write().await;
        if let Some(subscribers) = subs.get_mut(uri) {
            subscribers.retain(|id| id != subscriber_id);
            if subscribers.is_empty() {
                subs.remove(uri);
            }
        }
    }

    /// Get all subscribers for a resource
    pub async fn get_resource_subscribers(&self, uri: &str) -> Vec<String> {
        tracing::debug!(uri = %uri, "Getting resource subscribers");
        let subs = self.resource_subscriptions.read().await;
        subs.get(uri).cloned().unwrap_or_default()
    }

    /// Send a notification about resource updates using rmcp's notification system
    pub async fn notify_resource_updated(&self, uri: String) {
        tracing::debug!(uri = %uri, "Notifying resource updated");
        let subscribers = self.get_resource_subscribers(&uri).await;

        if subscribers.is_empty() {
            tracing::debug!(uri = %uri, "No subscribers for resource update");
            return;
        }

        // Use rmcp's notification system via the current RequestContext
        let context_guard = self.current_context.read().await;
        if let Some(context) = context_guard.as_ref() {
            if let Err(e) = context
                .peer
                .notify_resource_updated(ResourceUpdatedNotificationParam { uri: uri.clone() })
                .await
            {
                tracing::error!(
                    uri = %uri,
                    error = ?e,
                    "Failed to send resource updated notification"
                );
            } else {
                tracing::debug!(
                    uri = %uri,
                    subscriber_count = subscribers.len(),
                    "Resource updated notification sent via rmcp"
                );
            }
        } else {
            tracing::debug!(
                uri = %uri,
                subscriber_count = subscribers.len(),
                "Resource updated (no active request context)"
            );
        }
    }

    /// Send a notification that the resources list has changed
    /// Future work!
    #[allow(dead_code)]
    pub async fn notify_resource_list_changed(&self) {
        tracing::debug!("Notifying resource list changed");
        let context_guard = self.current_context.read().await;
        if let Some(context) = context_guard.as_ref() {
            if let Err(e) = context.peer.notify_resource_list_changed().await {
                tracing::error!(
                    error = ?e,
                    "Failed to send resource list changed notification"
                );
            } else {
                tracing::debug!("Resource list changed notification sent via rmcp");
            }
        } else {
            tracing::debug!("Resource list changed (no active request context)");
        }
    }

    /// Get completion suggestions for prompt arguments
    pub fn get_prompt_argument_completions(&self, prompt_name: &str, argument_name: &str) -> Vec<String> {
        tracing::debug!(
            prompt_name = %prompt_name,
            argument_name = %argument_name,
            "Getting prompt argument completions"
        );

        // Provide context-aware completions based on prompt and argument
        match (prompt_name, argument_name) {
            // Format-related arguments (RDF formats from rudof://formats/rdf)
            (_, "format") | (_, "input_format") | (_, "output_format") | (_, "rdf_format") => {
                vec![
                    "turtle".to_string(),
                    "ntriples".to_string(),
                    "rdfxml".to_string(),
                    "trig".to_string(),
                    "nquads".to_string(),
                    "n3".to_string(),
                    "jsonld".to_string(),
                ]
            },
            // ShEx schema format arguments (from rudof://formats/shex)
            (_, "schema_format") | (_, "shex_format") => {
                vec![
                    "shexc".to_string(),
                    "shexj".to_string(),
                    "turtle".to_string(),
                    "ntriples".to_string(),
                    "rdfxml".to_string(),
                    "jsonld".to_string(),
                    "trig".to_string(),
                    "n3".to_string(),
                    "nquads".to_string(),
                ]
            },
            // SHACL format arguments (from rudof://formats/shacl)
            (_, "shacl_format") | (_, "shapes_format") => {
                vec![
                    "turtle".to_string(),
                    "jsonld".to_string(),
                    "rdfxml".to_string(),
                    "trig".to_string(),
                    "nquads".to_string(),
                    "json".to_string(),
                ]
            },
            // Validation result format arguments (from rudof://formats/shex-validation-result and rudof://formats/shacl-validation-result)
            (_, "result_format") => {
                vec![
                    "details".to_string(),
                    "compact".to_string(),
                    "json".to_string(),
                    "turtle".to_string(),
                    "ntriples".to_string(),
                    "rdfxml".to_string(),
                    "trig".to_string(),
                    "n3".to_string(),
                    "nquads".to_string(),
                ]
            },
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
            // Mode argument for explore_rdf_node prompt (from rudof://formats/node-modes)
            (_, "mode") => {
                vec!["both".to_string(), "outgoing".to_string(), "incoming".to_string()]
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

    /// Get completion suggestions for resource URI templates
    pub fn get_resource_uri_completions(&self, uri: &str, argument_name: &str) -> Vec<String> {
        tracing::debug!(
            uri = %uri,
            argument_name = %argument_name,
            "Getting resource URI completions"
        );

        // Provide completions based on resource URI patterns
        if uri.starts_with("rudof://") {
            match argument_name {
                // RDF formats (from rudof://formats/rdf and rudof://current-data/*)
                "format" => {
                    vec![
                        "turtle".to_string(),
                        "ntriples".to_string(),
                        "rdfxml".to_string(),
                        "jsonld".to_string(),
                        "trig".to_string(),
                        "nquads".to_string(),
                        "n3".to_string(),
                    ]
                },
                // SPARQL endpoint suggestions
                "endpoint" => {
                    vec![
                        "https://query.wikidata.org/sparql".to_string(),
                        "https://dbpedia.org/sparql".to_string(),
                        "http://localhost:3030/sparql".to_string(),
                    ]
                },
                // Node inspection modes (from rudof://formats/node-modes)
                "mode" => {
                    vec!["both".to_string(), "outgoing".to_string(), "incoming".to_string()]
                },
                // Query result formats (from rudof://formats/query-results)
                "result_format" => {
                    vec![
                        "internal".to_string(),
                        "json".to_string(),
                        "xml".to_string(),
                        "csv".to_string(),
                        "tsv".to_string(),
                        "turtle".to_string(),
                        "ntriples".to_string(),
                        "rdfxml".to_string(),
                        "trig".to_string(),
                    ]
                },
                // ShEx schema formats (from rudof://formats/shex)
                "shex_format" | "schema_format" => {
                    vec![
                        "shexc".to_string(),
                        "shexj".to_string(),
                        "turtle".to_string(),
                        "ntriples".to_string(),
                        "rdfxml".to_string(),
                        "jsonld".to_string(),
                    ]
                },
                // SHACL formats (from rudof://formats/shacl)
                "shacl_format" | "shapes_format" => {
                    vec![
                        "turtle".to_string(),
                        "jsonld".to_string(),
                        "rdfxml".to_string(),
                        "trig".to_string(),
                        "nquads".to_string(),
                        "json".to_string(),
                    ]
                },
                // Validation reader modes (from rudof://formats/validation-reader-modes)
                "reader_mode" => {
                    vec!["strict".to_string(), "lax".to_string()]
                },
                _ => vec![],
            }
        } else {
            vec![]
        }
    }
}

impl Default for RudofMcpService {
    fn default() -> Self {
        Self::new()
    }
}

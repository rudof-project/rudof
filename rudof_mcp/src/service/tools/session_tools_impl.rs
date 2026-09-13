//! Session-management tools: `reset` and `cd`.
//!
//! These mirror the `reset` and `cd` commands already available in the `rudof`
//! interactive shell (`rudof_cli/src/shell/repl.rs`), adapted for MCP:
//! - `reset` wraps the same `rudof_lib::Rudof::reset_*` builders the shell uses.
//! - `cd` does **not** call `std::env::set_current_dir` like the shell does —
//!   under the streamable-HTTP transport, several sessions (each its own
//!   `RudofMcpService`) can run concurrently inside one OS process, and the
//!   process's cwd is shared global state. Instead each session keeps its own
//!   virtual working directory in `RudofMcpService::session_dir`, which
//!   `resolve_input_spec` (see `helpers.rs`) consults when resolving relative
//!   local file paths passed to other tools.

use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
};
use rudof_lib::Rudof;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::helpers::*;
use crate::service::mcp_service::RudofMcpService;

/// Valid `reset` target names, mirroring the shell's `RESET_TARGETS`
/// (`rudof_cli/src/shell/repl.rs`) minus `endpoint`, which has no equivalent
/// piece of session state in the MCP service.
const RESET_TARGETS: &[&str] = &[
    "data",
    "shex",
    "shex-validation",
    "shacl",
    "shacl-validation",
    "pgschema",
    "pgschema-validation",
    "shapemap",
    "dctap",
    "service",
    "query",
    "sparql",
    "typemap",
    "rdf-config",
];

/// Request parameters for resetting session state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResetSessionStateRequest {
    /// Which pieces of session state to clear. Omit, pass [], or pass ["all"]
    /// to clear everything (RDF data, schemas, shapes, shapemap, results, ...).
    /// Otherwise pass one or more of: data, shex, shex-validation, shacl,
    /// shacl-validation, pgschema, pgschema-validation, shapemap, dctap,
    /// service, query, sparql, typemap, rdf-config.
    pub targets: Option<Vec<String>>,
}

/// Response after resetting session state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResetSessionStateResponse {
    /// The targets that were actually reset (expanded to the full list when "all"/none was requested).
    pub reset: Vec<String>,
}

/// Reset one or more pieces of session state (RDF data, loaded schemas/shapes,
/// shapemap, cached results, ...), or everything when no target is given.
///
/// # Errors
///
/// Returns a Tool Execution Error when an unknown target name is given.
pub async fn reset_session_state_impl(
    service: &RudofMcpService,
    params: Parameters<ResetSessionStateRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(ResetSessionStateRequest { targets }) = params;
    let requested = targets.unwrap_or_default();
    let reset_all = requested.is_empty() || requested.iter().any(|t| t == "all");

    if !reset_all {
        let unknown: Vec<&str> = requested
            .iter()
            .map(String::as_str)
            .filter(|target| !RESET_TARGETS.contains(target))
            .collect();
        if !unknown.is_empty() {
            return Ok(ToolExecutionError::with_hint(
                format!("Unknown reset target(s): {}", unknown.join(", ")),
                format!("Valid targets: {}, or 'all'", RESET_TARGETS.join(", ")),
            )
            .into_call_tool_result());
        }
    }

    let reset_list: Vec<String> = if reset_all {
        RESET_TARGETS.iter().map(|s| s.to_string()).collect()
    } else {
        requested
    };

    let data_touched = reset_all || reset_list.iter().any(|t| t == "data");

    {
        let mut rudof = service.rudof.lock().await;
        if reset_all {
            rudof.reset_all().execute();
        } else {
            for target in &reset_list {
                reset_target(&mut rudof, target);
            }
        }
    }

    // Re-sync Docker-persisted state (see `mcp_service::persist_state`) so a
    // container restart doesn't resurrect data this reset just cleared. Done
    // after releasing the lock above: `persist_state` re-acquires it.
    if data_touched && let Err(e) = service.persist_state().await {
        tracing::warn!("Failed to persist state after reset: {}", e);
    }

    let response = ResetSessionStateResponse {
        reset: reset_list.clone(),
    };
    let structured = serialize_structured(&response, "reset_session_state_impl")?;
    let summary = format!("Reset: {}.", reset_list.join(", "));

    let mut result = CallToolResult::success(vec![ContentBlock::text(summary)]);
    result.structured_content = Some(structured);
    Ok(result)
}

fn reset_target(rudof: &mut Rudof, target: &str) {
    match target {
        "data" => rudof.reset_data().execute(),
        "shex" => rudof.reset_shex_schema().execute(),
        "shex-validation" => rudof.reset_shex().execute(),
        "shacl" => rudof.reset_shacl_shapes().execute(),
        "shacl-validation" => rudof.reset_shacl().execute(),
        "pgschema" => rudof.reset_pg_schema().execute(),
        "pgschema-validation" => rudof.reset_pg_schema_validation().execute(),
        "shapemap" => rudof.reset_shapemap().execute(),
        "dctap" => rudof.reset_dctap().execute(),
        "service" => rudof.reset_service_description().execute(),
        "query" => {
            rudof.reset_sparql_query().execute();
            rudof.reset_query_results().execute();
        },
        "sparql" => rudof.reset_sparql_query().execute(),
        "typemap" => rudof.reset_typemap().execute(),
        "rdf-config" => rudof.reset_rdf_config().execute(),
        _ => unreachable!("target validated against RESET_TARGETS by reset_session_state_impl"),
    }
}

/// Request parameters for getting/changing the session's virtual working directory.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChangeDirectoryRequest {
    /// Directory to change into (absolute, or relative to the current session
    /// directory). Omit to just report the current session directory without
    /// changing it.
    pub path: Option<String>,
}

/// Response reporting the session's (possibly just-changed) virtual working directory.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChangeDirectoryResponse {
    /// The session's current working directory after this call.
    pub cwd: String,
}

/// Get or change this session's virtual working directory, used to resolve
/// relative local file paths passed to other tools (schema/data/shapes
/// arguments, ...).
///
/// This does not affect the server process's actual working directory or any
/// other concurrent session — see the module docs for why.
///
/// # Errors
///
/// Returns a Tool Execution Error when `path` doesn't resolve to an existing directory.
pub async fn change_directory_impl(
    service: &RudofMcpService,
    params: Parameters<ChangeDirectoryRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(ChangeDirectoryRequest { path }) = params;
    let mut session_dir = service.session_dir.write().await;

    if let Some(path) = path {
        let candidate = session_dir.join(&path);
        let canonical = match candidate.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Failed to change directory to '{}': {}", path, e),
                    "Provide a path to an existing directory, absolute or relative to the current session directory",
                )
                .into_call_tool_result());
            },
        };
        if !canonical.is_dir() {
            return Ok(ToolExecutionError::with_hint(
                format!("'{}' is not a directory", path),
                "Provide a path to an existing directory",
            )
            .into_call_tool_result());
        }
        *session_dir = canonical;
    }

    let response = ChangeDirectoryResponse {
        cwd: session_dir.display().to_string(),
    };
    let structured = serialize_structured(&response, "change_directory_impl")?;
    let summary = format!("Session directory: {}", response.cwd);

    let mut result = CallToolResult::success(vec![ContentBlock::text(summary)]);
    result.structured_content = Some(structured);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rudof_lib::formats::{DataFormat, InputSpec as RudofInputSpec};

    async fn load_one_triple(service: &RudofMcpService) {
        let mut rudof = service.rudof.lock().await;
        rudof
            .load_data()
            .with_data(&[RudofInputSpec::Str(
                "<http://a.example/s> <http://a.example/p> <http://a.example/o> .".to_string(),
            )])
            .with_data_format(&DataFormat::NTriples)
            .execute()
            .expect("loading inline test data should succeed");
    }

    /// After a reset there's no data source at all (not even an empty one), so
    /// `serialize_data` errors with `NoDataLoaded` rather than returning "" —
    /// that error counts as "empty" here too.
    async fn is_data_empty(service: &RudofMcpService) -> bool {
        let mut rudof = service.rudof.lock().await;
        let mut buf = Vec::new();
        match rudof.serialize_data(&mut buf).execute() {
            Ok(()) => buf.is_empty(),
            Err(_) => true,
        }
    }

    #[tokio::test]
    async fn reset_all_clears_data() {
        let service = RudofMcpService::new();
        load_one_triple(&service).await;
        assert!(!is_data_empty(&service).await);

        let result = reset_session_state_impl(&service, Parameters(ResetSessionStateRequest { targets: None }))
            .await
            .expect("reset should not be a protocol error");
        assert_ne!(result.is_error, Some(true));
        assert!(is_data_empty(&service).await);
    }

    #[tokio::test]
    async fn reset_specific_target_clears_only_that_state() {
        let service = RudofMcpService::new();
        load_one_triple(&service).await;

        let result = reset_session_state_impl(
            &service,
            Parameters(ResetSessionStateRequest {
                targets: Some(vec!["shex".to_string()]),
            }),
        )
        .await
        .expect("reset should not be a protocol error");
        assert_ne!(result.is_error, Some(true));
        // "shex" was reset, not "data" — data should be untouched.
        assert!(!is_data_empty(&service).await);
    }

    #[tokio::test]
    async fn reset_unknown_target_is_a_tool_error() {
        let service = RudofMcpService::new();

        let result = reset_session_state_impl(
            &service,
            Parameters(ResetSessionStateRequest {
                targets: Some(vec!["not-a-real-target".to_string()]),
            }),
        )
        .await
        .expect("invalid target should be a tool error, not a protocol error");
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn change_directory_with_no_path_reports_current_dir() {
        let service = RudofMcpService::new();
        let before = service.session_dir.read().await.clone();

        let result = change_directory_impl(&service, Parameters(ChangeDirectoryRequest { path: None }))
            .await
            .expect("querying cwd should not be a protocol error");
        assert_ne!(result.is_error, Some(true));
        assert_eq!(*service.session_dir.read().await, before);
    }

    #[tokio::test]
    async fn change_directory_updates_session_dir_not_process_cwd() {
        let service = RudofMcpService::new();
        let tmp_dir = std::env::temp_dir().canonicalize().expect("temp dir should exist");
        let process_cwd_before = std::env::current_dir().unwrap();

        let result = change_directory_impl(
            &service,
            Parameters(ChangeDirectoryRequest {
                path: Some(tmp_dir.display().to_string()),
            }),
        )
        .await
        .expect("changing to an existing directory should not be a protocol error");
        assert_ne!(result.is_error, Some(true));

        assert_eq!(*service.session_dir.read().await, tmp_dir);
        // The OS process cwd must be untouched — that's the whole point of a
        // per-session virtual directory instead of `std::env::set_current_dir`.
        assert_eq!(std::env::current_dir().unwrap(), process_cwd_before);
    }

    #[tokio::test]
    async fn change_directory_rejects_nonexistent_path() {
        let service = RudofMcpService::new();

        let result = change_directory_impl(
            &service,
            Parameters(ChangeDirectoryRequest {
                path: Some("/no/such/directory/hopefully".to_string()),
            }),
        )
        .await
        .expect("a bad path should be a tool error, not a protocol error");
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn change_directory_rejects_a_file_path() {
        let service = RudofMcpService::new();
        let file = tempfile::NamedTempFile::new().expect("should create temp file");

        let result = change_directory_impl(
            &service,
            Parameters(ChangeDirectoryRequest {
                path: Some(file.path().display().to_string()),
            }),
        )
        .await
        .expect("a file path should be a tool error, not a protocol error");
        assert_eq!(result.is_error, Some(true));
    }
}

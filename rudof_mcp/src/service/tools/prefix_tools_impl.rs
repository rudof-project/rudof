//! `manage_prefixes` tool: mirrors the shell's `prefixes` command.
//!
//! Shows, or manages, the *default* prefix declarations -- the ones assumed
//! and prepended by default to RDF data, SPARQL queries, ShEx schemas and
//! SHACL shapes, independently of whatever prefixes a loaded resource already
//! declares. See `rudof_cli/src/shell/repl.rs::handle_prefixes`.

use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::helpers::*;
use crate::service::mcp_service::RudofMcpService;

/// Request parameters for showing or managing the default prefixes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ManagePrefixesRequest {
    /// Action to perform. One of: "list" (default) -- show the current default
    /// prefixes; "add" -- add or overwrite an alias; "remove" -- delete an
    /// alias; "rename" -- rename an alias, keeping its IRI; "copy" -- add a
    /// new alias for the same IRI as an existing one.
    pub action: Option<String>,

    /// Prefix alias, e.g. "wd". Required for "add" and "remove"; for "rename"
    /// and "copy" this is the existing alias to rename/copy from.
    pub alias: Option<String>,

    /// IRI to associate with `alias`, e.g. "http://www.wikidata.org/entity/". Required for "add".
    pub iri: Option<String>,

    /// New alias name. Required for "rename" and "copy".
    pub new_alias: Option<String>,
}

/// A single alias/IRI prefix declaration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrefixEntry {
    /// The prefix alias, e.g. "wd".
    pub alias: String,
    /// The IRI the alias expands to.
    pub iri: String,
}

/// Response listing the default prefixes after the requested action.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ManagePrefixesResponse {
    /// The default prefixes after this call, sorted by alias.
    pub prefixes: Vec<PrefixEntry>,
}

/// Show, or manage, the session's default prefix declarations.
///
/// # Errors
///
/// Returns a Tool Execution Error when `action` is unknown, a required field
/// for the given `action` is missing, or the underlying operation fails (e.g.
/// removing/renaming/copying an alias that isn't defined, or adding an
/// invalid IRI).
pub async fn manage_prefixes_impl(
    service: &RudofMcpService,
    params: Parameters<ManagePrefixesRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(ManagePrefixesRequest {
        action,
        alias,
        iri,
        new_alias,
    }) = params;
    let action = action.unwrap_or_else(|| "list".to_string());

    let mut rudof = service.rudof.lock().await;

    match action.as_str() {
        "list" => {},
        "add" => {
            let alias = match require(&alias, "alias", "add") {
                Ok(v) => v,
                Err(result) => return Ok(result),
            };
            let iri = match require(&iri, "iri", "add") {
                Ok(v) => v,
                Err(result) => return Ok(result),
            };
            if let Err(e) = rudof.add_prefix(alias, iri).execute() {
                return Ok(ToolExecutionError::with_hint(
                    format!("Failed to add prefix '{alias}': {e}"),
                    "Provide a valid absolute IRI for 'iri'",
                )
                .into_call_tool_result());
            }
        },
        "remove" => {
            let alias = match require(&alias, "alias", "remove") {
                Ok(v) => v,
                Err(result) => return Ok(result),
            };
            if let Err(e) = rudof.remove_prefix(alias).execute() {
                return Ok(ToolExecutionError::with_hint(
                    format!("Failed to remove prefix '{alias}': {e}"),
                    "List the current prefixes with action \"list\" to see valid aliases",
                )
                .into_call_tool_result());
            }
        },
        "rename" => {
            let old_alias = match require(&alias, "alias", "rename") {
                Ok(v) => v,
                Err(result) => return Ok(result),
            };
            let new_alias = match require(&new_alias, "new_alias", "rename") {
                Ok(v) => v,
                Err(result) => return Ok(result),
            };
            if let Err(e) = rudof.rename_prefix(old_alias, new_alias).execute() {
                return Ok(ToolExecutionError::with_hint(
                    format!("Failed to rename prefix '{old_alias}' to '{new_alias}': {e}"),
                    "List the current prefixes with action \"list\" to see valid aliases",
                )
                .into_call_tool_result());
            }
        },
        "copy" => {
            let old_alias = match require(&alias, "alias", "copy") {
                Ok(v) => v,
                Err(result) => return Ok(result),
            };
            let new_alias = match require(&new_alias, "new_alias", "copy") {
                Ok(v) => v,
                Err(result) => return Ok(result),
            };
            if let Err(e) = rudof.copy_prefix(old_alias, new_alias).execute() {
                return Ok(ToolExecutionError::with_hint(
                    format!("Failed to copy prefix '{old_alias}' to '{new_alias}': {e}"),
                    "List the current prefixes with action \"list\" to see valid aliases",
                )
                .into_call_tool_result());
            }
        },
        other => {
            return Ok(ToolExecutionError::with_hint(
                format!("Unknown action '{other}'"),
                "Valid actions: list, add, remove, rename, copy",
            )
            .into_call_tool_result());
        },
    }

    let prefix_map = rudof.prefixes().execute();
    drop(rudof);

    let mut prefixes: Vec<PrefixEntry> = prefix_map
        .iter()
        .map(|(alias, iri)| PrefixEntry {
            alias: alias.clone(),
            iri: iri.as_str().to_string(),
        })
        .collect();
    prefixes.sort_by(|a, b| a.alias.cmp(&b.alias));

    let response = ManagePrefixesResponse {
        prefixes: prefixes.clone(),
    };
    let structured = serialize_structured(&response, "manage_prefixes_impl")?;
    let summary = if prefixes.is_empty() {
        "No default prefixes are defined.".to_string()
    } else {
        prefixes
            .iter()
            .map(|p| format!("{}: <{}>", p.alias, p.iri))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let mut result = CallToolResult::success(vec![ContentBlock::text(summary)]);
    result.structured_content = Some(structured);
    Ok(result)
}

/// Extracts a required string field, or a Tool Execution Error result naming
/// which field is missing for which action.
fn require<'a>(value: &'a Option<String>, field: &str, action: &str) -> Result<&'a str, CallToolResult> {
    value.as_deref().ok_or_else(|| {
        ToolExecutionError::with_hint(
            format!("Missing required field '{field}' for action '{action}'"),
            format!("Provide '{field}' when action is '{action}'"),
        )
        .into_call_tool_result()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        action: &str,
        alias: Option<&str>,
        iri: Option<&str>,
        new_alias: Option<&str>,
    ) -> Parameters<ManagePrefixesRequest> {
        Parameters(ManagePrefixesRequest {
            action: Some(action.to_string()),
            alias: alias.map(String::from),
            iri: iri.map(String::from),
            new_alias: new_alias.map(String::from),
        })
    }

    #[tokio::test]
    async fn list_on_fresh_session_is_empty() {
        let service = RudofMcpService::new();

        let result = manage_prefixes_impl(&service, request("list", None, None, None))
            .await
            .expect("list should not be a protocol error");
        assert_ne!(result.is_error, Some(true));

        let structured = result
            .structured_content
            .expect("response should have structured content");
        let prefixes = structured.get("prefixes").and_then(|v| v.as_array()).unwrap();
        assert!(prefixes.is_empty());
    }

    #[tokio::test]
    async fn add_then_list_shows_the_new_prefix() {
        let service = RudofMcpService::new();

        let add_result = manage_prefixes_impl(
            &service,
            request("add", Some("wd"), Some("http://www.wikidata.org/entity/"), None),
        )
        .await
        .expect("add should not be a protocol error");
        assert_ne!(add_result.is_error, Some(true));

        let structured = add_result
            .structured_content
            .expect("response should have structured content");
        let prefixes = structured.get("prefixes").and_then(|v| v.as_array()).unwrap();
        assert_eq!(prefixes.len(), 1);
        assert_eq!(prefixes[0]["alias"], "wd");
        assert_eq!(prefixes[0]["iri"], "http://www.wikidata.org/entity/");
    }

    #[tokio::test]
    async fn add_missing_iri_is_a_tool_error() {
        let service = RudofMcpService::new();

        let result = manage_prefixes_impl(&service, request("add", Some("wd"), None, None))
            .await
            .expect("missing field should be a tool error, not a protocol error");
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn add_invalid_iri_is_a_tool_error() {
        let service = RudofMcpService::new();

        let result = manage_prefixes_impl(&service, request("add", Some("wd"), Some("not an iri"), None))
            .await
            .expect("invalid IRI should be a tool error, not a protocol error");
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn remove_unknown_alias_is_a_tool_error() {
        let service = RudofMcpService::new();

        let result = manage_prefixes_impl(&service, request("remove", Some("nope"), None, None))
            .await
            .expect("removing an unknown alias should be a tool error, not a protocol error");
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn rename_then_list_reflects_the_new_alias() {
        let service = RudofMcpService::new();
        manage_prefixes_impl(
            &service,
            request("add", Some("wd"), Some("http://www.wikidata.org/entity/"), None),
        )
        .await
        .unwrap();

        let result = manage_prefixes_impl(&service, request("rename", Some("wd"), None, Some("wikidata")))
            .await
            .expect("rename should not be a protocol error");
        assert_ne!(result.is_error, Some(true));

        let structured = result
            .structured_content
            .expect("response should have structured content");
        let prefixes = structured.get("prefixes").and_then(|v| v.as_array()).unwrap();
        assert_eq!(prefixes.len(), 1);
        assert_eq!(prefixes[0]["alias"], "wikidata");
    }

    #[tokio::test]
    async fn copy_then_list_has_both_aliases() {
        let service = RudofMcpService::new();
        manage_prefixes_impl(
            &service,
            request("add", Some("wd"), Some("http://www.wikidata.org/entity/"), None),
        )
        .await
        .unwrap();

        let result = manage_prefixes_impl(&service, request("copy", Some("wd"), None, Some("wikidata")))
            .await
            .expect("copy should not be a protocol error");
        assert_ne!(result.is_error, Some(true));

        let structured = result
            .structured_content
            .expect("response should have structured content");
        let prefixes = structured.get("prefixes").and_then(|v| v.as_array()).unwrap();
        assert_eq!(prefixes.len(), 2);
    }

    #[tokio::test]
    async fn unknown_action_is_a_tool_error() {
        let service = RudofMcpService::new();

        let result = manage_prefixes_impl(&service, request("frobnicate", None, None, None))
            .await
            .expect("unknown action should be a tool error, not a protocol error");
        assert_eq!(result.is_error, Some(true));
    }
}

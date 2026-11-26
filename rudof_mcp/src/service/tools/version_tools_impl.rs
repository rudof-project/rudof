//! `get_rudof_version` tool: reports the rudof version this MCP server runs.

use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::data_tools_impl::EmptyRequest;
use super::helpers::*;
use crate::service::mcp_service::RudofMcpService;

/// Response reporting the rudof version.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetVersionResponse {
    /// The rudof version this MCP server is running (matches `rudof --version`).
    pub version: String,
}

/// Report the rudof version this MCP server is running.
///
/// # Errors
///
/// This tool never returns a Tool Execution Error.
pub async fn get_version_impl(
    service: &RudofMcpService,
    _params: Parameters<EmptyRequest>,
) -> Result<CallToolResult, McpError> {
    let version = {
        let rudof = service.rudof.lock().await;
        rudof.version().execute().to_string()
    };

    let response = GetVersionResponse {
        version: version.clone(),
    };
    let structured = serialize_structured(&response, "get_version_impl")?;
    let summary = format!("rudof version {version}");

    let mut result = CallToolResult::success(vec![ContentBlock::text(summary)]);
    result.structured_content = Some(structured);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_version_reports_a_non_empty_version() {
        let service = RudofMcpService::new();

        let result = get_version_impl(&service, Parameters(EmptyRequest {}))
            .await
            .expect("get_version should not be a protocol error");
        assert_ne!(result.is_error, Some(true));

        let structured = result
            .structured_content
            .expect("response should have structured content");
        let version = structured
            .get("version")
            .and_then(|v| v.as_str())
            .expect("response should contain a version string");
        assert!(!version.is_empty());
        assert_eq!(version, env!("CARGO_PKG_VERSION"));
    }
}

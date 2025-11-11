use crate::rudof_mcp_service::{errors::*, service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    InputSpec,
    query::{detect_query_type, execute_query},
    query_result_format::ResultQueryFormat,
    query_type::QueryType,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSparqlQueryRequest {
    /// SPARQL query string to execute
    pub query: String,

    /// Result format: "Internal" (table), "NTriples", "JsonLd", "RdfXml", "Csv", "TriG", "N3", "NQuads", "Turtle".
    pub result_format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryExecutionResponse {
    /// Query type that was executed
    pub query_type: String,

    /// Result format used
    pub result_format: String,

    /// Execution status
    pub status: String,

    /// Results as structured data
    pub results: String,

    /// Size of the results in bytes
    pub result_size_bytes: usize,

    /// Number of lines in the result
    pub result_lines: usize,
}

pub async fn execute_sparql_query_impl(
    service: &RudofMcpService,
    Parameters(ExecuteSparqlQueryRequest {
        query,
        result_format,
    }): Parameters<ExecuteSparqlQueryRequest>,
) -> Result<CallToolResult, McpError> {
    let query_type_str = detect_query_type(&query).ok_or_else(|| {
        invalid_request(
            error_messages::INVALID_QUERY_TYPE,
            Some(
                json!({"error": "Could not detect query type (SELECT, CONSTRUCT, ASK, DESCRIBE)"}),
            ),
        )
    })?;
    let parsed_query_type = QueryType::from_str(&query_type_str).map_err(|e| {
        invalid_request(
            error_messages::INVALID_QUERY_TYPE,
            Some(json!({ "error": e.to_string()})),
        )
    })?;

    let result_format_str = result_format.as_deref().unwrap_or("Internal");
    let parsed_result_format = ResultQueryFormat::from_str(result_format_str).map_err(|e| {
        invalid_request(
            error_messages::INVALID_QUERY_RESULT_FORMAT,
            Some(json!({"error": e.to_string()})),
        )
    })?;

    let query_spec = InputSpec::Str(query.clone());

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    execute_query(
        &mut rudof,
        &query_spec,
        &parsed_query_type,
        &parsed_result_format,
        &mut output_buffer,
    )
    .map_err(|e| {
        internal_error(
            error_messages::QUERY_EXECUTION_ERROR,
            Some(json!({"error": e.to_string(),})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            error_messages::CONVERSION_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    // Calculate metadata
    let result_size_bytes = output_str.len();
    let result_lines = output_str.lines().count();

    let response = QueryExecutionResponse {
        query_type: query_type_str.clone(),
        result_format: result_format_str.to_string(),
        status: "success".to_string(),
        results: output_str.to_string(),
        result_size_bytes,
        result_lines,
    };

    tracing::info!(
        query_type = %query_type_str,
        result_format = %result_format_str,
        result_size_bytes,
        result_lines,
        query_length = query.len(),
        "Executed SPARQL query successfully"
    );

    tracing::info!(
        query_type = %query_type_str,
        result_format = %result_format_str,
        result_size_bytes,
        result_lines,
        query_length = query.len(),
        "Executed SPARQL query successfully"
    );

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    // Create a summary text with metadata
    let summary = format!(
        "# SPARQL Query Execution\n\n\
        **Status:** ✓ Success\n\
        **Query Type:** {}\n\
        **Result Format:** {}\n\
        **Result Size:** {} bytes\n\
        **Result Lines:** {}\n",
        query_type_str,
        result_format_str,
        result_size_bytes,
        result_lines
    );

    // Format the query in a code block
    let query_display = format!("## Query\n\n```sparql\n{}\n```", query);

    // Format results based on the format type
    let results_display = match result_format_str.to_lowercase().as_str() {
        "csv" => format!("## Results\n\n```csv\n{}\n```", output_str),
        "jsonld" | "json" => format!("## Results\n\n```json\n{}\n```", output_str),
        "turtle" | "n3" => format!("## Results\n\n```turtle\n{}\n```", output_str),
        "ntriples" | "nquads" => format!("## Results\n\n```ntriples\n{}\n```", output_str),
        "rdfxml" => format!("## Results\n\n```xml\n{}\n```", output_str),
        "trig" => format!("## Results\n\n```trig\n{}\n```", output_str),
        _ => format!("## Results\n\n```\n{}\n```", output_str),
    };

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(query_display),
        Content::text(results_display),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rudof_mcp_service::service::{RudofMcpService, ServiceConfig};
    use crate::rudof_mcp_service::tools::data_tools_impl::{
        LoadRdfDataFromSourcesRequest, load_rdf_data_from_sources_impl,
    };
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};

    const SAMPLE_TURTLE: &str = r#"
        prefix : <http://example.org/>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :a :name "Alice" ;
           :birthdate "1990-05-02"^^xsd:date ;
           :enrolledIn :cs101 .

        :b :name "Bob", "Robert" .

        :cs101 :name "Computer Science" .
    "#;

    async fn create_test_service() -> RudofMcpService {
        tokio::task::spawn_blocking(|| {
            let rudof_config = rudof_lib::RudofConfig::new().unwrap();
            let rudof = rudof_lib::Rudof::new(&rudof_config).unwrap();
            RudofMcpService {
                rudof: Arc::new(Mutex::new(rudof)),
                tool_router: Default::default(),
                prompt_router: Default::default(),
                config: Arc::new(RwLock::new(ServiceConfig::default())),
                resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
                log_level_handle: None,
            }
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_execute_sparql_query_impl_select_success() {
        let service = create_test_service().await;

        let _ = load_rdf_data_from_sources_impl(
            &service,
            Parameters(LoadRdfDataFromSourcesRequest {
                data: vec![SAMPLE_TURTLE.to_string()],
                data_format: "turtle".to_string(),
                base: None,
                endpoint: None,
            }),
        )
        .await
        .unwrap();

        let query = r#"SELECT ?s ?p ?o WHERE { ?s ?p ?o }"#.to_string();

        let params = Parameters(ExecuteSparqlQueryRequest {
            query,
            result_format: Some("Internal".to_string()),
        });

        let result = execute_sparql_query_impl(&service, params).await;
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.structured_content.is_some());
        assert!(!call_result.content.is_empty());

        let structured = call_result.structured_content.unwrap();
        assert_eq!(structured["status"], "success");

        assert_eq!(
            structured["query_type"].as_str().unwrap().to_uppercase(),
            "SELECT"
        );
    }

    #[tokio::test]
    async fn test_execute_sparql_query_impl_invalid_query() {
        let service = create_test_service().await;

        let query = "INVALID QUERY".to_string();

        let params = Parameters(ExecuteSparqlQueryRequest {
            query,
            result_format: Some("Internal".to_string()),
        });

        let result = execute_sparql_query_impl(&service, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, error_messages::INVALID_QUERY_TYPE);
    }

    #[tokio::test]
    async fn test_execute_sparql_query_impl_invalid_result_format() {
        let service = create_test_service().await;

        let query = r#"SELECT ?s WHERE { ?s ?p ?o }"#.to_string();

        let params = Parameters(ExecuteSparqlQueryRequest {
            query,
            result_format: Some("UnknownFormat".to_string()),
        });

        let result = execute_sparql_query_impl(&service, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, error_messages::INVALID_QUERY_RESULT_FORMAT);
    }
}

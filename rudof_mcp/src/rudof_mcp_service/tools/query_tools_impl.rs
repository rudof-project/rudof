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

    let response = QueryExecutionResponse {
        query_type: query_type_str.clone(),
        result_format: result_format_str.to_string(),
        status: "success".to_string(),
        results: output_str.to_string(),
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let text_output = format!(
        "Query executed successfully\n
        Query Type: {}\n
        Result Format: {}\n
        Results:\n{}",
        query_type_str, result_format_str, output_str
    );

    let mut result = CallToolResult::success(vec![Content::text(text_output)]);
    result.structured_content = Some(structured);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rudof_mcp_service::tools::data_tools_impl::{
        load_rdf_data_from_sources_impl,
        LoadRdfDataFromSourcesRequest,
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;

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
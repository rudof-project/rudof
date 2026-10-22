use crate::service::{errors::*, mcp_service::*};
// `CreateMessageRequestParams`/`SamplingMessage` (MCP sampling) are
// deprecated by SEP-2577; the protocol still supports sampling and there
// is no replacement API, so they stay in use here.
#[allow(deprecated)]
use rmcp::{
    ErrorData as McpError, Peer, RoleServer,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock, CreateMessageRequestParams, SamplingMessage},
};
use rudof_lib::formats::{InputSpec, ResultQueryFormat};
use rudof_lib::types::{Bindings, QueryResult, QueryResultKind};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

use super::helpers::*;
use super::params::ResultQueryFormatParam;

/// Request parameters for executing a SPARQL query.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSparqlQueryRequest {
    /// Direct SPARQL query string to execute. Supports SELECT, CONSTRUCT, and ASK.
    /// Mutually exclusive with `query_natural_language` — provide exactly one.
    pub query: Option<String>,

    /// Natural language description of what to query (e.g. "find all persons and their names").
    /// Converted to SPARQL via LLM sampling.
    /// Mutually exclusive with `query` — provide exactly one.
    pub query_natural_language: Option<String>,

    /// Output format for results. SELECT queries only support "internal".
    /// Default: internal.
    pub result_format: Option<ResultQueryFormatParam>,
}

/// Response containing query execution results.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryExecutionResponse {
    /// The SPARQL query that was executed (either provided directly or generated from natural language)
    pub query: String,
    /// Which kind of result the query produced, and so which of the two fields below is set.
    pub query_type: QueryResultKind,
    /// SELECT only: the variable bindings, one row per solution, every term qualified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Bindings>,
    /// ASK only: the answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    /// The results as rendered in `result_format`. For a CONSTRUCT this is the graph
    /// itself; for a SELECT it is the same rows as `bindings`, as a table.
    pub results: String,
    /// Format used for the results
    pub result_format: ResultQueryFormatParam,
    /// Size of the results in bytes
    pub result_size_bytes: usize,
}

/// The typed half of a query result: the bindings of a SELECT, or the answer of an ASK.
/// A CONSTRUCT has neither, and reaches the caller as the rendered graph.
fn query_outcome(results: &QueryResult) -> Result<(Option<Bindings>, Option<bool>), String> {
    match results {
        QueryResult::Select(solutions) => {
            let bindings = Bindings::try_from(solutions).map_err(|e| e.to_string())?;
            Ok((Some(bindings), None))
        },
        QueryResult::Ask(answer) => Ok((None, Some(*answer))),
        QueryResult::Construct(_) => Ok((None, None)),
    }
}

/// Generate a SPARQL query from natural language using rmcp sampling.
///
/// Uses the MCP sampling capability to request LLM assistance in
/// converting natural language to SPARQL.
// MCP sampling is deprecated by SEP-2577; the protocol still supports it
// and there is no replacement API, so it stays in use here.
#[allow(deprecated)]
async fn generate_sparql_from_natural_language(
    natural_language: &str,
    peer: &Peer<RoleServer>,
) -> Result<String, McpError> {
    let system_message = r#"You are a SPARQL query expert. Convert natural language questions into valid SPARQL queries.
                                    - Only output the SPARQL query, no explanations or markdown formatting
                                    - Use standard SPARQL syntax (SELECT, CONSTRUCT, ASK, or DESCRIBE)
                                    - Include appropriate prefixes if needed
                                    - Make the query efficient and correct
                                    - If you need to guess prefixes, use common ones like rdf:, rdfs:, xsd:, ex:, etc."#;

    let user_message = format!("Generate a SPARQL query for: {}", natural_language);

    // Sampling is the client's to provide. Asking a client that never declared
    // it yields an opaque "method not found", so say what is missing instead.
    let declares_sampling = peer
        .peer_info()
        .is_some_and(|info| info.capabilities.sampling.is_some());
    if !declares_sampling {
        return Err(invalid_params_error(
            "Sampling not available",
            "This MCP client does not support sampling, which 'query_natural_language' needs. \
             Pass a SPARQL query in 'query' instead.",
            Some(json!({"operation":"generate_sparql_from_natural_language","phase":"check_capability"})),
        ));
    }

    // Create sampling request with system prompt and user message.
    let sampling_request = CreateMessageRequestParams::new(vec![SamplingMessage::user_text(user_message.clone())], 512)
        .with_system_prompt(system_message)
        .with_temperature(0.1); // Low temperature for more deterministic output

    // Send sampling request through rmcp
    let response = peer.create_message(sampling_request).await.map_err(|e| {
        internal_error(
            "Query generation error",
            e.to_string(),
            Some(json!({"operation":"generate_sparql_from_natural_language","phase":"create_message"})),
        )
    })?;

    // Extract the first text fragment from the sampling response.
    let generated_query = response
        .message
        .content
        .iter()
        .find_map(|content| content.as_text().map(|text| text.text.clone()))
        .ok_or_else(|| {
            internal_error(
                "Sampling response error",
                "Expected text response from LLM",
                Some(json!({"operation":"generate_sparql_from_natural_language","phase":"extract_response_text"})),
            )
        })?;

    // Clean up any markdown code blocks that might have been generated
    let cleaned_query = if generated_query.starts_with("```") {
        generated_query
            .lines()
            .skip(1)
            .take_while(|line| !line.starts_with("```"))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    } else {
        generated_query.trim().to_string()
    };

    tracing::debug!(
        natural_language = %natural_language,
        generated_query = %cleaned_query,
        "Generated SPARQL query from natural language via rmcp sampling"
    );

    Ok(cleaned_query)
}

/// Execute a SPARQL query against the loaded RDF data.
///
/// Supports SELECT, CONSTRUCT, ASK queries.
/// Can accept either a direct SPARQL query or a natural language
/// description that will be converted to SPARQL using LLM sampling.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - Both `query` and `query_natural_language` are provided
/// - Neither `query` nor `query_natural_language` is provided
/// - The query type cannot be determined
/// - The result format is not supported for the query type
///
/// Returns a Protocol Error for internal execution failures.
pub async fn execute_sparql_query_impl(
    service: &RudofMcpService,
    Parameters(ExecuteSparqlQueryRequest {
        query,
        query_natural_language,
        result_format,
    }): Parameters<ExecuteSparqlQueryRequest>,
    peer: &Peer<RoleServer>,
) -> Result<CallToolResult, McpError> {
    // Determine the SPARQL query to execute - return Tool Execution Error for invalid combinations
    let sparql_query = match (query, query_natural_language) {
        (Some(q), None) => {
            // Direct SPARQL query provided
            q
        },
        (None, Some(nl)) => {
            // Natural language query provided - generate SPARQL using rmcp sampling
            generate_sparql_from_natural_language(&nl, peer).await?
        },
        (Some(_), Some(_)) => {
            return Ok(ToolExecutionError::new(
                "Cannot provide both 'query' and 'query_natural_language'. Choose one.",
            )
            .into_call_tool_result());
        },
        (None, None) => {
            return Ok(ToolExecutionError::with_hint(
                "No query provided",
                "Provide either 'query' with a SPARQL query string, or 'query_natural_language' with a description",
            )
            .into_call_tool_result());
        },
    };

    let result_format = result_format.unwrap_or(ResultQueryFormatParam::Internal);
    let parsed_result_format: ResultQueryFormat = result_format.into();

    // Guard: DESCRIBE is not yet implemented in rudof_lib.
    let query_type = sparql_query_type(&sparql_query);
    if matches!(query_type, Some("DESCRIBE")) {
        return Ok(ToolExecutionError::with_hint(
            "DESCRIBE queries are not yet supported",
            "Use SELECT, CONSTRUCT, or ASK queries instead",
        )
        .into_call_tool_result());
    }

    // Guard: SELECT queries only support the 'internal' result format.
    if matches!(query_type, Some("SELECT")) && result_format != ResultQueryFormatParam::Internal {
        return Ok(ToolExecutionError::with_hint(
            format!("Format '{}' is not yet supported for SELECT queries", result_format),
            "SELECT queries only support 'internal' format. Omit result_format or use 'internal'.",
        )
        .into_call_tool_result());
    }

    let query_spec = InputSpec::Str(sparql_query.clone());

    let mut rudof = service.rudof.lock().await;

    rudof.load_sparql_query(&query_spec).execute().map_err(|e| {
        internal_error(
            "Query execution error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"execute_query"})),
        )
    })?;

    rudof.run_query().execute().map_err(|e| {
        internal_error(
            "Query execution error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"run_query"})),
        )
    })?;

    let query_results = rudof.query_results().ok_or_else(|| {
        internal_error(
            "Query execution error",
            "The query ran but left no results behind",
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"read_query_results"})),
        )
    })?;
    let query_type = query_results.kind();
    let (bindings, boolean) = query_outcome(query_results).map_err(|e| {
        internal_error(
            "Query execution error",
            e,
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"qualify_bindings"})),
        )
    })?;

    let mut output_buffer = Cursor::new(Vec::new());

    let serialization = rudof
        .serialize_query_results(&mut output_buffer)
        .with_result_query_format(&parsed_result_format);
    serialization.execute().map_err(|e| {
        internal_error(
            "Query results serialization error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"serialize_results"})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"utf8_conversion"})),
        )
    })?;

    // Calculate metadata
    let result_size_bytes = output_str.len();

    let response = QueryExecutionResponse {
        query: sparql_query.clone(),
        query_type,
        bindings,
        boolean,
        result_format,
        results: output_str.clone(),
        result_size_bytes,
    };

    let structured = serialize_structured(&response, "execute_sparql_query_impl")?;

    let outcome = match query_type {
        QueryResultKind::Select => format!("Rows: {}", response.bindings.as_ref().map_or(0, Bindings::count)),
        QueryResultKind::Ask => format!("Answer: {}", response.boolean.unwrap_or_default()),
        QueryResultKind::Construct => "Constructed an RDF graph".to_string(),
    };
    let summary = format!(
        "SPARQL query executed.\n{}\nResult format: {}\nResult size: {} bytes",
        outcome, result_format, result_size_bytes
    );

    let mut result = CallToolResult::success(vec![ContentBlock::text(summary)]);
    result.structured_content = Some(structured);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rudof_lib::types::{Object, QueryResultKind};
    use rudof_lib::{Rudof, RudofConfig};

    /// Runs `query` over a fixed graph and hands back what the tool would report.
    ///
    /// `execute_sparql_query_impl` itself needs a `Peer`, which rmcp keeps crate-private,
    /// so these drive the result shaping rather than the tool call.
    fn run(query: &str) -> (QueryResultKind, Option<Bindings>, Option<bool>) {
        let mut rudof = Rudof::new(RudofConfig::default());
        rudof
            .load_data()
            .with_data(&[InputSpec::str(
                r#"prefix : <http://example.org/>
                   :alice :name "Alice"@en ; :knows :bob .
                   :bob   :name "Bob" ."#,
            )])
            .execute()
            .expect("the test data should load");

        rudof
            .load_sparql_query(&InputSpec::Str(query.to_string()))
            .execute()
            .expect("the query should parse");
        rudof.run_query().execute().expect("the query should run");

        let results = rudof.query_results().expect("the query leaves results behind");
        let (bindings, boolean) = query_outcome(results).expect("the terms should qualify");
        (results.kind(), bindings, boolean)
    }

    #[test]
    fn a_select_reports_its_rows_keyed_by_variable() {
        let (kind, bindings, boolean) = run(r#"prefix : <http://example.org/>
               select ?person ?name where { ?person :name ?name } order by ?person"#);

        assert_eq!(kind, QueryResultKind::Select);
        assert_eq!(boolean, None);

        let bindings = bindings.expect("a SELECT reports bindings");
        assert_eq!(bindings.variables(), ["person", "name"]);
        assert_eq!(bindings.count(), 2);

        let alice = bindings
            .rows()
            .iter()
            .find(|row| row.get("person").map(|term| term.to_string()) == Some("http://example.org/alice".to_string()))
            .expect("one row binds ?person to :alice");
        match alice.get("name") {
            Some(Object::Literal(literal)) => {
                assert_eq!(literal.lexical_form(), "Alice");
                assert_eq!(
                    literal.lang().map(|lang| lang.as_str().to_string()),
                    Some("en".to_string())
                );
            },
            other => panic!("?name should bind a language-tagged literal, not {other:?}"),
        }
    }

    #[test]
    fn an_unbound_variable_is_absent_from_its_row() {
        let (_, bindings, _) = run(r#"prefix : <http://example.org/>
               select ?person ?nickname where { ?person :name ?name optional { ?person :nick ?nickname } }"#);

        let bindings = bindings.expect("a SELECT reports bindings");
        assert_eq!(bindings.variables(), ["person", "nickname"]);
        assert!(
            bindings.rows().iter().all(|row| !row.contains_key("nickname")),
            "nothing in the data binds ?nickname"
        );
        assert!(bindings.rows().iter().all(|row| row.contains_key("person")));
    }

    #[test]
    fn an_ask_reports_its_answer() {
        let (kind, bindings, boolean) = run(r#"prefix : <http://example.org/>
               ask { :alice :knows :bob }"#);

        assert_eq!(kind, QueryResultKind::Ask);
        assert_eq!(boolean, Some(true));
        assert!(bindings.is_none());
    }

    #[test]
    fn a_construct_reports_neither() {
        let (kind, bindings, boolean) = run(r#"prefix : <http://example.org/>
               construct { ?person :label ?name } where { ?person :name ?name }"#);

        assert_eq!(kind, QueryResultKind::Construct);
        assert!(bindings.is_none());
        assert!(boolean.is_none());
    }
}

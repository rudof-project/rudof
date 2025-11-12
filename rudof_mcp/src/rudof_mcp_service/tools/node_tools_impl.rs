use crate::rudof_mcp_service::{errors::*, service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{node_info::*, parse_node_selector};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoRequest {
    /// Node IRI or prefixed name (e.g. ":a" or "http://example.org/a")
    pub node: String,
    /// Optional list of predicates to filter outgoing arcs
    pub predicates: Option<Vec<String>>,
    /// Optional mode
    pub mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoResponse {
    /// The qualified IRI of the RDF subject node.
    pub subject: String,
    /// List of outgoing arcs from the subject node.
    pub outgoing: Vec<NodePredicateObjects>,
    /// List of incoming arcs to the subject node.
    pub incoming: Vec<NodePredicateSubjects>,
    /// Number of outgoing predicates
    pub outgoing_count: usize,
    /// Number of incoming predicates
    pub incoming_count: usize,
    /// Total number of outgoing objects
    pub total_outgoing_objects: usize,
    /// Total number of incoming subjects
    pub total_incoming_subjects: usize,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodePredicateObjects {
    /// The qualified IRI of the predicate.
    pub predicate: String,
    /// List of qualified object terms linked via this predicate.
    pub objects: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodePredicateSubjects {
    /// The qualified IRI of the predicate.
    pub predicate: String,
    /// List of qualified subject terms that point to the node via this predicate.
    pub subjects: Vec<String>,
}

pub async fn node_info_impl(
    service: &RudofMcpService,
    Parameters(NodeInfoRequest {
        node,
        predicates,
        mode,
    }): Parameters<NodeInfoRequest>,
) -> Result<CallToolResult, McpError> {
    let rudof = service.rudof.lock().await;
    let rdf = rudof.get_rdf_data();

    let node_selector = parse_node_selector(&node)
        .map_err(|e| rdf_error("parsing node selector", e.to_string()))?;

    let mode_str = mode.as_deref().unwrap_or("both");
    let mut options = NodeInfoOptions::from_mode_str(mode_str)
        .map_err(|e| rdf_error("parsing node mode", e.to_string()))?;
    options.show_colors = false;

    let pred_list: Vec<String> = predicates.unwrap_or_default();
    let node_infos = match get_node_info(rdf, node_selector, &pred_list, &options) {
        Ok(infos) => infos,
        Err(e) => {
            return Err(resource_not_found(
                error_messages::NODE_NOT_FOUND,
                Some(json!({ "error": e.to_string() })),
            ));
        }
    };

    let node_info = &node_infos[0];

    let mut output_buffer = Cursor::new(Vec::new());

    format_node_info_list(&node_infos, rdf, &mut output_buffer, &options)
        .map_err(|e| rdf_error("formatting node info", e.to_string()))?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes)
        .map_err(|e| rdf_error("converting to UTF-8", e.to_string()))?;

    let outgoing_data: Vec<NodePredicateObjects> = node_info
        .outgoing
        .iter()
        .map(|(predicate_iri, objects_vec)| NodePredicateObjects {
            predicate: predicate_iri.to_string(),
            objects: objects_vec.iter().map(|term| term.to_string()).collect(),
        })
        .collect();

    let incoming_data: Vec<NodePredicateSubjects> = node_info
        .incoming
        .iter()
        .map(|(predicate_iri, subjects_vec)| NodePredicateSubjects {
            predicate: predicate_iri.to_string(),
            subjects: subjects_vec
                .iter()
                .map(|subject| subject.to_string())
                .collect(),
        })
        .collect();

    // Calculate metadata
    let outgoing_count = outgoing_data.len();
    let incoming_count = incoming_data.len();
    let total_outgoing_objects: usize = outgoing_data.iter().map(|p| p.objects.len()).sum();
    let total_incoming_subjects: usize = incoming_data.iter().map(|p| p.subjects.len()).sum();

    let response = NodeInfoResponse {
        subject: node_info.subject_qualified.clone(),
        outgoing: outgoing_data,
        incoming: incoming_data,
        outgoing_count,
        incoming_count,
        total_outgoing_objects,
        total_incoming_subjects,
    };

    tracing::info!(
        node = %node,
        subject = %node_info.subject_qualified,
        outgoing_predicates = outgoing_count,
        incoming_predicates = incoming_count,
        total_outgoing_objects,
        total_incoming_subjects,
        mode = mode_str,
        "Retrieved node information"
    );

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    // Create a summary text
    let summary = format!(
        "# Node Information: {}\n\n\
        **Mode:** {}\n\
        **Outgoing Predicates:** {}\n\
        **Incoming Predicates:** {}\n\
        **Total Outgoing Objects:** {}\n\
        **Total Incoming Subjects:** {}\n",
        node_info.subject_qualified,
        mode_str,
        outgoing_count,
        incoming_count,
        total_outgoing_objects,
        total_incoming_subjects
    );

    let detailed_output = format!("## Detailed Node Information\n\n```\n{}\n```", output_str);

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(detailed_output),
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
    use rmcp::handler::server::wrapper::Parameters;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio;
    use tokio::sync::{Mutex, RwLock};

    const SAMPLE_TURTLE: &str = r#"
        prefix : <http://example.org/>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :a :name       "Alice"                  ;
        :birthdate  "1990-05-02"^^xsd:date   ;
        :enrolledIn :cs101                   .

        :b :name "Bob", "Robert" .

        :cs101 :name "Computer Science" .
    "#;

    // Initialize the RudofMcpService in a blocking-safe context
    async fn create_test_service() -> RudofMcpService {
        tokio::task::spawn_blocking(|| {
            let rudof_config = rudof_lib::RudofConfig::new().unwrap();
            let rudof = rudof_lib::Rudof::new(&rudof_config).unwrap();
            let (notification_tx, _) = tokio::sync::broadcast::channel(100);
            RudofMcpService {
                rudof: Arc::new(Mutex::new(rudof)),
                tool_router: Default::default(),
                prompt_router: Default::default(),
                config: Arc::new(RwLock::new(ServiceConfig::default())),
                resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
                log_level_handle: None,
                notification_tx: Arc::new(notification_tx),
            }
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_node_info_impl_success() {
        let service = create_test_service().await;

        // Load RDF data
        let _ = load_rdf_data_from_sources_impl(
            &service,
            Parameters(LoadRdfDataFromSourcesRequest {
                data: vec![SAMPLE_TURTLE.to_string()],
                data_format: None,
                base: None,
                endpoint: None,
            }),
        )
        .await
        .unwrap();

        let params = Parameters(NodeInfoRequest {
            node: ":a".to_string(),
            predicates: None,
            mode: Some("both".to_string()),
        });

        let result = node_info_impl(&service, params).await;
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.structured_content.is_some());
        assert!(!call_result.content.is_empty());
    }

    #[tokio::test]
    async fn test_node_info_impl_invalid_node() {
        let service = create_test_service().await;

        // Load RDF data
        let _ = load_rdf_data_from_sources_impl(
            &service,
            Parameters(LoadRdfDataFromSourcesRequest {
                data: vec![SAMPLE_TURTLE.to_string()],
                data_format: None,
                base: None,
                endpoint: None,
            }),
        )
        .await
        .unwrap();

        let params = Parameters(NodeInfoRequest {
            node: "<http://example.org/nonexistent>".to_string(),
            predicates: None,
            mode: Some("both".to_string()),
        });

        let result = node_info_impl(&service, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "Node not found");
    }
}

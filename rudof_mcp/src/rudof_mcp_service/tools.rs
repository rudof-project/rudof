use std::str::FromStr;
use serde_json::json;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content},
    tool,
    tool_router,
    ErrorData as McpError,
};
use crate::rudof_mcp_service::{types::*, errors::*};
use rudof_lib::{RDFFormat, ReaderMode, srdf, node_info::*};
use srdf::{NeighsRDF, Iri, Term, Subject};

#[tool_router]
impl super::RudofMcpService {
    #[tool(name = "load_rdf_data", description = "Load RDF data from a string into the server's datastore")]
    pub async fn load_rdf_data(&self, Parameters(LoadRdfDataRequest { rdf_data, format }): Parameters<LoadRdfDataRequest>) -> Result<CallToolResult, McpError> {
    let mut rudof = self.rudof.lock().await;
        match RDFFormat::from_str(&format) {
            Ok(parsed_format) => {
                let _result = rudof
                    .read_data(rdf_data.as_bytes(), &parsed_format, None, &ReaderMode::default())
                    .map_err(|e| {
                        internal_error(
                                codes::RDF_LOAD_ERROR,
                                Some(json!({ "error": e.to_string() })),
                            )
                    })?;
                let response = crate::rudof_mcp_service::types::LoadRdfDataResponse { message: "RDF data loaded successfully".to_string() };
                let structured = serde_json::to_value(&response).map_err(|e| {
                    internal_error(
                        codes::RDF_LOAD_ERROR,
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let mut result = CallToolResult::success(vec![Content::text(response.message.clone())]);
                result.structured_content = Some(structured);
                Ok(result)
            }
            Err(e) => {
                tracing::error!("Failed to load RDF data: {}", e);
                return Err(invalid_request(
                    codes::INVALID_FORMAT,
                    Some(json!({ "format": format, "error": e.to_string() })),
                ));
            }
        }
    }

    #[tool(name = "export_rdf_data", description = "Serialize and return the current RDF datastore in the requested format")]
    pub async fn export_rdf_data(&self, Parameters(ExportRdfDataRequest { format }): Parameters<ExportRdfDataRequest>) -> Result<CallToolResult, McpError> {
        let rudof = self.rudof.lock().await;
        match RDFFormat::from_str(&format) {
            Ok(parsed_format) => {
                let mut v = Vec::new();
                let _result = rudof.serialize_data(&parsed_format, &mut v).map_err(|e| {
                    internal_error(
                        codes::SERIALIZE_DATA_ERROR,
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let str = String::from_utf8(v).map_err(|e| {
                    internal_error(
                        codes::UTF8_CONVERSION_ERROR,
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let response = crate::rudof_mcp_service::types::ExportRdfDataResponse { data: str.clone(), format: format.clone() };
                let structured = serde_json::to_value(&response).map_err(|e| {
                    internal_error(
                        codes::SERIALIZE_DATA_ERROR,
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let mut result = CallToolResult::success(vec![Content::text(str)]);
                result.structured_content = Some(structured);
                Ok(result)
            }
            Err(e) => {
                tracing::error!("Failed to serialize RDF data: {}", e);
                return Err(invalid_request(
                    codes::INVALID_FORMAT,
                    Some(json!({ "format": format, "error": e.to_string() })),
                ));
            }
        }
    }

    // Formats an IRI/NamedNode as a plain string (no colors, no prefixing).
    // Uses Iri's standard Display implementation (likely the full IRI).
    fn format_iri_no_color<I: Iri>(iri: &I) -> String {
        iri.to_string()
    }

    // Formats an RDF Term as a plain string (no colors, no prefixing).
    // Uses Term's standard Display implementation.
    fn format_term_no_color<T: Term>(term: &T) -> String {
        term.to_string()
    }

    // Formats an RDF Subject as a plain string (no colors, no prefixing).
    // Uses Subject's standard Display implementation.
    fn format_subject_no_color<S: Subject>(subject: &S) -> String {
        subject.to_string()
    }

    // Convert NodeInfo to text format for MCP response
    // This matches the CLI output format for consistency
    fn format_node_info_text<S: NeighsRDF>(node_info: &NodeInfo<S>) -> String {
        let mut out = String::new();

        out.push_str(&format!("Information about {}\n", node_info.subject_qualified));

        // Outgoing arcs
        if !node_info.outgoing.is_empty() {
            out.push_str("Outgoing arcs\n");
            out.push_str(&format!("{}\n", node_info.subject_qualified));

            let mut preds: Vec<_> = node_info.outgoing.keys().collect();
            preds.sort();

            for pred in preds {
                out.push_str(&format!(" -{}-> \n", Self::format_iri_no_color(pred)));
                if let Some(objs) = node_info.outgoing.get(pred) {
                    for o in objs {
                        out.push_str(&format!("       {}\n", Self::format_term_no_color(o)));
                    }
                }
            }
        }

        // Incoming arcs
        if !node_info.incoming.is_empty() {
            out.push_str("Incoming arcs\n");
            let object: S::Term = node_info.subject.clone().into();
            out.push_str(&format!("{}\n", Self::format_term_no_color(&object)));

            let mut preds: Vec<_> = node_info.incoming.keys().collect();
            preds.sort();

            for pred in preds {
                out.push_str(&format!(" <-{}-\n", Self::format_iri_no_color(pred)));
                if let Some(subjs) = node_info.incoming.get(pred) {
                    for s in subjs {
                        out.push_str(&format!("       {}\n", Self::format_subject_no_color(s)));
                    }
                }
            }
        }

        out
    }

    // Convert NodeInfo to structured JSON response
    fn node_info_to_response<S: NeighsRDF>(node_info: &NodeInfo<S>) -> NodeInfoResponse {
        let outgoing = node_info
            .outgoing
            .iter()
            .map(|(p, objs)| {
                let pred_q = Self::format_iri_no_color(p);
                
                let obj_qs = objs.iter().map(|o| Self::format_term_no_color(o)).collect();
                NodePredicateObjects {
                    predicate: pred_q,
                    objects: obj_qs,
                }
            })
            .collect();

        let incoming = node_info
            .incoming
            .iter()
            .map(|(p, subs)| {
                let pred_q = Self::format_iri_no_color(p);
                
                let sub_qs = subs.iter().map(|s| Self::format_subject_no_color(s)).collect();
                NodePredicateSubjects {
                    predicate: pred_q,
                    subjects: sub_qs,
                }
            })
            .collect();

        NodeInfoResponse {
            subject: node_info.subject_qualified.clone(),
            outgoing,
            incoming,
        }
    }

    #[tool(name = "node_info", description = "Show information about a node (outgoing/incoming arcs) from the server RDF datastore")]
    pub async fn node_info(&self, Parameters(NodeInfoRequest {node, predicates,mode}): Parameters<NodeInfoRequest>) -> Result<CallToolResult, McpError> {
        tracing::info!("Tool 'node_info' called with: node='{}', predicates={:?}, mode={:?}", node, predicates, mode);

        let rudof = self.rudof.lock().await;
        let rdf = rudof.get_rdf_data();

        let node_selector = parse_node_selector(&node).map_err(|e| {
            tracing::error!("Failed to parse node selector: {}", e);
            invalid_request(
                codes::INVALID_NODE_SELECTOR,
                Some(json!({ "node": node, "error": e.to_string() })),
            )
        })?;

        let mode_str = mode.as_deref().unwrap_or("both");
        let options = NodeInfoOptions::from_mode_str(mode_str).map_err(|_| {
            invalid_request(codes::INVALID_MODE, Some(json!({ "mode": mode_str })))
        })?;

        let pred_list: Vec<String> = predicates.unwrap_or_default();

        let node_infos = get_node_info(rdf, node_selector, &pred_list, options).map_err(|e| {
            tracing::error!("Failed to get node info: {}", e);
            internal_error(
                codes::RDF_ARC_QUERY_ERROR,
                Some(json!({ "error": e.to_string() })),
            )
        })?;

        let node_info = node_infos.first().ok_or_else(|| {
            tracing::warn!("Node not found in dataset: {}", node);
            internal_error(codes::NODE_NOT_FOUND, Some(json!({ "node": node })))
        })?;

        let text_output = Self::format_node_info_text(node_info);

        let response = Self::node_info_to_response(node_info);

        let structured = serde_json::to_value(&response).map_err(|e| {
            internal_error(
                codes::SERIALIZE_DATA_ERROR,
                Some(json!({ "error": e.to_string() })),
            )
        })?;

        let mut result = CallToolResult::success(vec![Content::text(text_output)]);
        result.structured_content = Some(structured);
        Ok(result)
    }
}

// Public wrapper to expose the generated router from the macro
pub fn tool_router_public() -> ToolRouter<super::RudofMcpService> {
    super::RudofMcpService::tool_router()
}

// Return the tools list annotated with helpful metadata (titles and annotations)
pub fn annotated_tools() -> Vec<rmcp::model::Tool> {
    let mut tools = tool_router_public().list_all();

    for tool in tools.iter_mut() {
        match tool.name.as_ref() {
            "load_rdf_data" => {
                tool.title = Some("Load RDF data".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                    .read_only(false)
                    .destructive(false)
                    .idempotent(false)
                );
            }
            "export_rdf_data" => {
                tool.title = Some("Export RDF data".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                    .read_only(true)
                    .destructive(false)
                    .idempotent(true)
                );
            }
            "node_info" => {
                tool.title = Some("Inspect RDF Node".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true),
                );
            }

            _ => {}
        }
    }

    tools
}

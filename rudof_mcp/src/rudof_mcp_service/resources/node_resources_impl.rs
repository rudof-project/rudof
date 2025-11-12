use rmcp::{
    ErrorData as McpError,
    model::{Annotated, ReadResourceResult, RawResource, ResourceContents},
};
use serde_json::json;

pub fn get_node_resources() -> Vec<Annotated<RawResource>> {
    vec![
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/node-modes".to_string(),
                name: "Node Inspection Modes".to_string(),
                description: Some("Available modes for node inspection".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
    ]
}

pub fn handle_node_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/node-modes" => Some(get_node_modes(uri)),
        _ => None,
    }
}

fn get_node_modes(uri: &str) -> Result<ReadResourceResult, McpError> {
    let modes = json!({
        "modes": [
            {
                "name": "Both",
                "value": "both",
                "description": "Show both incoming and outgoing relationships"
            },
            {
                "name": "Incoming",
                "value": "incoming",
                "description": "Show only relationships pointing to this node"
            },
            {
                "name": "Outgoing",
                "value": "outgoing",
                "description": "Show only relationships originating from this node"
            }
        ],
        "default": "both"
    });
    
    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: serde_json::to_string_pretty(&modes).unwrap(),
            meta: None,
        }],
    })
}

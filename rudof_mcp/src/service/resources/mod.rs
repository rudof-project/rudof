mod data_resources_impl;
mod node_resources_impl;
mod query_resources_impl;
mod resources_impl;
mod shacl_validate_resources_impl;
mod shex_validate_resources_impl;

use crate::service::errors::internal_error;
use rmcp::{
	ErrorData as McpError,
	model::{Annotated, RawResource, ReadResourceResult, ResourceContents},
};
use serde_json::Value;

pub(crate) fn make_resource(uri: &str, name: &str, description: &str, mime_type: &str) -> Annotated<RawResource> {
	Annotated {
		raw: RawResource {
			uri: uri.to_string(),
			name: name.to_string(),
			description: Some(description.to_string()),
			mime_type: Some(mime_type.to_string()),
			title: None,
			size: None,
			icons: None,
			meta: None,
		},
		annotations: None,
	}
}

pub(crate) fn json_resource_result(uri: &str, value: &Value) -> Result<ReadResourceResult, McpError> {
	let json = serde_json::to_string_pretty(value).map_err(|e| {
		internal_error(
			"Serialization error",
			e.to_string(),
			Some(serde_json::json!({"operation":"json_resource_result","uri":uri})),
		)
	})?;

	Ok(ReadResourceResult::new(vec![
		ResourceContents::TextResourceContents {
			uri: uri.to_string(),
			mime_type: Some("application/json".to_string()),
			text: json,
			meta: None,
		},
	]))
}

pub use resources_impl::{list_resources, read_resource};

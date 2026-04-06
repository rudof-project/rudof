use rmcp::ErrorData as McpError;

/// Parse and validate an opaque cursor token.
///
/// This implementation currently stores cursors as numeric offsets.
/// Invalid format or out-of-range values are reported as Invalid Params (-32602).
pub fn parse_cursor(
    cursor: Option<String>,
    upper_bound: usize,
    operation: &str,
) -> Result<usize, McpError> {
    let Some(cursor) = cursor else {
        return Ok(0);
    };

    let parsed = cursor.parse::<usize>().map_err(|_| {
        McpError::invalid_params(
            format!("Invalid cursor for {}: '{}'", operation, cursor),
            None,
        )
    })?;

    if parsed > upper_bound {
        return Err(McpError::invalid_params(
            format!("Invalid cursor for {}: out of range", operation),
            None,
        ));
    }

    Ok(parsed)
}

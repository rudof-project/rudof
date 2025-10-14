use std::collections::HashMap;

use shex_ast::{Node, ShapeLabelIdx, shapemap::ValidationStatus};

/// Typing represents a mapping from (Node, ShapeLabelIdx) to ValidationStatus
/// This will be used to collect errors and reasons during validation
#[derive(Debug, Clone)]
pub struct Typing {
    _map: HashMap<(Node, ShapeLabelIdx), ValidationStatus>,
}

impl Typing {
    pub fn new() -> Self {
        Typing {
            _map: HashMap::new(),
        }
    }
}

impl Default for Typing {
    fn default() -> Self {
        Self::new()
    }
}

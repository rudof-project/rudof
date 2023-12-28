use std::collections::HashMap;

use srdf::lang::Lang;

#[derive(Debug, Clone)]
pub struct MessageMap {
    mmap: HashMap<Option<Lang>, String>
}

impl  MessageMap {
    pub fn new() -> Self {
        MessageMap {
            mmap: HashMap::new()
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct MessageMap {
    // mmap: HashMap<Option<Lang>, String>
}

impl MessageMap {
    pub fn new() -> Self {
        Self::default()
    }
}

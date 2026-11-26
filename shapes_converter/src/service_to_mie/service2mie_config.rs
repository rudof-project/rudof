#[derive(Clone, Debug)]
pub struct Service2MieConfig {}

impl Service2MieConfig {
    pub fn new() -> Self {
        Service2MieConfig {}
    }
}

impl Default for Service2MieConfig {
    fn default() -> Self {
        Self::new()
    }
}

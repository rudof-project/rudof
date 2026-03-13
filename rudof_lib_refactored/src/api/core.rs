use crate::{RudofCore, Result, RudofConfig};

impl RudofCore for crate::Rudof {
    fn new(config: &RudofConfig) -> Result<Self> {
        todo!()
    }

    fn version(&self) -> &str {
        todo!()
    }

    fn config(&self) -> &RudofConfig {
        todo!()
    }

    fn update_config(&mut self, config: &RudofConfig) {
        todo!()
    }

    fn reset_all(&mut self) {
        todo!()
    }
}

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TapConfig {
   
}

impl Default for TapConfig {
    
    fn default() -> Self {
        Self {  }
    }
}
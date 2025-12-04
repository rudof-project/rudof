use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResultAssociation {
    conforms: bool,
    reason: Option<String>,
    app_info: Option<Value>,
}

impl ResultAssociation {
    pub fn new(conforms: bool, reason: Option<String>, app_info: Option<Value>) -> Self {
        ResultAssociation {
            conforms,
            reason,
            app_info,
        }
    }

    pub fn conforms(&self) -> bool {
        self.conforms
    }
}

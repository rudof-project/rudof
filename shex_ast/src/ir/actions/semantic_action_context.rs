/// Context passed to semantic actions when they are executed.
#[derive(Debug, Clone)]
pub struct SemanticActionContext {
    pub subject: Option<String>,
    pub predicate: Option<String>,
    pub object: Option<String>,
}

use rudof_rdf::rdf_core::term::Object;

/// State used during the parsing process
/// This is used to keep track of pending shapes to be parsed
pub(crate) struct State {
    pending: Vec<Object>,
}

impl State {
    fn pop_pending(&mut self) -> Option<Object> {
        self.pending.pop()
    }
}

impl From<Vec<Object>> for State {
    fn from(value: Vec<Object>) -> Self {
        Self { pending: value }
    }
}
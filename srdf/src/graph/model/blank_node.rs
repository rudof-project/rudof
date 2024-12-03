use oxrdf::BlankNode as OxBlankNode;
use oxrdf::BlankNodeRef as OxBlankNodeRef;

use crate::model::BlankNode;

impl BlankNode for OxBlankNode {
    fn id(&self) -> &str {
        self.as_str()
    }
}

impl BlankNode for OxBlankNodeRef<'_> {
    fn id(&self) -> &str {
        self.as_str()
    }
}

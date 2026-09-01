use std::fmt::Display;

/// Identifies a box within a single [`crate::model::Diagram`].
///
/// Domain crates (e.g. an RDF graph or a ShEx schema) keep their own node-id types and convert
/// them to a `BoxId` only when building the technology-agnostic diagram model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoxId(usize);

impl BoxId {
    pub fn new(id: usize) -> Self {
        BoxId(id)
    }

    pub fn as_usize(self) -> usize {
        self.0
    }
}

impl From<usize> for BoxId {
    fn from(id: usize) -> Self {
        BoxId::new(id)
    }
}

impl Display for BoxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::BoxId;

    #[test]
    fn round_trips_through_usize() {
        let id = BoxId::new(42);
        assert_eq!(id.as_usize(), 42);
        assert_eq!(BoxId::from(42usize), id);
    }

    #[test]
    fn displays_as_the_bare_number() {
        assert_eq!(BoxId::new(7).to_string(), "7");
    }
}

use super::NodeId;

#[derive(Debug, PartialEq)]
pub enum UmlCardinality {
    UmlClass {
        id: NodeId,
        label: String,
        href: Option<String>,
        entries: Vec<UmlEntry>,
    },
}

impl UmlComponent {}

enum UmlEntry {
    UmlField {
        name: String,
        href: Option<Href>,
        valueConstraint: Vec<ValueConstraint>,
        card: UMLCardinality,
    },
}

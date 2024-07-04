#[derive(Debug, PartialEq, Clone)]
pub enum UmlCardinality {
    OneOne,
    Star,
    Plus,
    Optional,
    Range(i32, i32),
    Fixed(i32),
}

impl Default for UmlCardinality {
    fn default() -> Self {
        UmlCardinality::OneOne
    }
}

/*#[derive(Debug, PartialEq)]
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
}*/

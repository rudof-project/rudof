use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum UmlCardinality {
    #[default]
    OneOne,

    Star,
    Plus,
    Optional,
    Range(i32, i32),
    Fixed(i32),
}

impl Display for UmlCardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UmlCardinality::OneOne => write!(f, "1..1"),
            UmlCardinality::Star => write!(f, "0..*"),
            UmlCardinality::Plus => write!(f, "1..*"),
            UmlCardinality::Optional => write!(f, "0..1"),
            UmlCardinality::Range(min, max) => write!(f, "{}..{}", min, max),
            UmlCardinality::Fixed(n) => write!(f, "{}", n),
        }
    }
}

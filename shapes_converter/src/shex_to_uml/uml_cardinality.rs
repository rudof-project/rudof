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

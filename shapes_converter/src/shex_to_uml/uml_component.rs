use super::UmlClass;

#[derive(Debug, PartialEq)]
pub enum UmlComponent {
    UmlClass(UmlClass),
}

impl UmlComponent {
    pub fn class(class: UmlClass) -> UmlComponent {
        UmlComponent::UmlClass(class)
    }
}

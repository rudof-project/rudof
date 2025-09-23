use super::UmlClass;

#[derive(Debug, PartialEq)]
pub enum UmlComponent {
    UmlClass(UmlClass),
    Or { exprs: Vec<UmlComponent> },
    Not { expr: Box<UmlComponent> },
    And { exprs: Vec<UmlComponent> },
}

impl UmlComponent {
    pub fn class(class: UmlClass) -> UmlComponent {
        UmlComponent::UmlClass(class)
    }

    pub fn or<I: Iterator<Item = UmlComponent>>(cs: I) -> UmlComponent {
        UmlComponent::Or {
            exprs: cs.collect(),
        }
    }
}

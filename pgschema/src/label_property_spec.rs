use crate::{
    formal_base_type::FormalBaseType,
    // formal_graph_type::FormalGraphType,
    pgs::PropertyGraphSchema,
    pgs_error::PgsError,
    property_value_spec::PropertyValueSpec,
    type_name::{Name, TypeName},
};
use std::fmt::Display;

// In the PGSchema paper, LabelPropertySpec is denoted by F
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelPropertySpec {
    Empty,
    Label(Name),
    Ref(TypeName),
    Optional(Box<LabelPropertySpec>),
    And(Box<LabelPropertySpec>, Box<LabelPropertySpec>),
    Or(Box<LabelPropertySpec>, Box<LabelPropertySpec>),
    Open(Box<LabelPropertySpec>),
    Content(Box<LabelPropertySpec>, PropertyValueSpec),
}

impl Default for LabelPropertySpec {
    fn default() -> Self {
        Self::new()
    }
}

impl LabelPropertySpec {
    pub fn new() -> Self {
        LabelPropertySpec::Empty
    }

    pub fn label(label: Name) -> Self {
        LabelPropertySpec::Label(label)
    }
    pub fn optional(label_property_spec: LabelPropertySpec) -> Self {
        LabelPropertySpec::Optional(Box::new(label_property_spec))
    }
    pub fn and(label_property_spec: LabelPropertySpec, label_property_spec1: LabelPropertySpec) -> Self {
        LabelPropertySpec::And(Box::new(label_property_spec), Box::new(label_property_spec1))
    }
    pub fn or(label_property_spec: LabelPropertySpec, label_property_spec1: LabelPropertySpec) -> Self {
        LabelPropertySpec::Or(Box::new(label_property_spec), Box::new(label_property_spec1))
    }
    pub fn open(label_property_spec: LabelPropertySpec) -> Self {
        LabelPropertySpec::Open(Box::new(label_property_spec))
    }
    pub fn ref_(type_name: TypeName) -> Self {
        LabelPropertySpec::Ref(type_name)
    }

    pub fn content(label_property_spec: LabelPropertySpec, property_value_spec: PropertyValueSpec) -> Self {
        LabelPropertySpec::Content(Box::new(label_property_spec), property_value_spec)
    }

    pub fn semantics(&self, schema: &PropertyGraphSchema) -> Result<FormalBaseType, PgsError> {
        match self {
            LabelPropertySpec::Empty => Ok(FormalBaseType::type_0()),
            LabelPropertySpec::Label(label) => Ok(FormalBaseType::from_label(label.clone())),
            LabelPropertySpec::Ref(type_name) => {
                let label_property_spec = schema.get_node_semantics(type_name)?;
                label_property_spec.semantics(schema)
            },
            LabelPropertySpec::Optional(label_property_spec) => {
                let base_type = label_property_spec.semantics(schema)?;
                Ok(base_type.union(&FormalBaseType::type_0()))
            },
            LabelPropertySpec::And(label_property_spec, label_property_spec1) => {
                let base_type = label_property_spec.semantics(schema)?;
                let base_type1 = label_property_spec1.semantics(schema)?;
                Ok(base_type.combine(&base_type1))
            },
            LabelPropertySpec::Or(label_property_spec, label_property_spec1) => {
                let base_type = label_property_spec.semantics(schema)?;
                let base_type1 = label_property_spec1.semantics(schema)?;
                Ok(base_type.union(&base_type1))
            },
            LabelPropertySpec::Open(label_property_spec) => {
                let base_type = label_property_spec.semantics(schema)?;
                Ok(base_type.with_open())
            },
            LabelPropertySpec::Content(label_property_spec, property_value_spec) => {
                let base_type = label_property_spec.semantics(schema)?;
                let property_value_semantics = property_value_spec.semantics()?;
                let result = base_type.combine(&property_value_semantics);
                Ok(result)
            },
        }
    }
}

impl Display for LabelPropertySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelPropertySpec::Empty => write!(f, "Empty"),
            LabelPropertySpec::Label(label) => write!(f, "Label({})", label),
            LabelPropertySpec::Ref(type_name) => write!(f, "Ref({})", type_name),
            LabelPropertySpec::Optional(spec) => write!(f, "Optional({})", spec),
            LabelPropertySpec::And(left, right) => write!(f, "And({}, {})", left, right),
            LabelPropertySpec::Or(left, right) => write!(f, "Or({}, {})", left, right),
            LabelPropertySpec::Open(spec) => write!(f, "Open({})", spec),
            LabelPropertySpec::Content(spec, value_spec) => {
                write!(f, "Content({}, {})", spec, value_spec)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        card::Card,
        key::Key,
        property_value_spec::{PropertyValue, TypeSpec},
        record_type::RecordType,
        value_type::ValueType,
    };

    use super::*;

    #[test]
    fn test_semantics_basic_record() {
        let mut graph = PropertyGraphSchema::new();
        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let age = PropertyValue::property(Key::new("age"), TypeSpec::integer(Card::One));
        let person_content = PropertyValue::each_of(name, age);
        let _ = graph.add_node_spec(
            "PersonType",
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );

        let semantics = graph
            .get_node_semantics("PersonType")
            .unwrap()
            .semantics(&graph)
            .unwrap();
        let expected = FormalBaseType::new().with_label("Person").with_record_type(
            RecordType::new()
                .with_key_value("age", ValueType::integer(Card::One))
                .with_key_value("name", ValueType::string(Card::One)),
        );
        assert_eq!(semantics, expected);
    }
}

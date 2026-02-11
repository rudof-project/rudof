use std::{collections::HashSet, fmt::Display};

use crate::{node_id::NodeId, record::Record, type_name::LabelName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub labels: HashSet<LabelName>,
    pub properties: Record,
}

impl Node {
    pub fn new(id: NodeId) -> Self {
        Node {
            id,
            labels: HashSet::new(),
            properties: Record::new(),
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.labels.insert(label.to_string());
        self
    }

    pub fn with_labels(mut self, labels: HashSet<LabelName>) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_content(mut self, content: &Record) -> Self {
        self.properties = content.clone();
        self
    }

    pub fn labels(&self) -> &HashSet<LabelName> {
        &self.labels
    }

    pub fn content(&self) -> &Record {
        &self.properties
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({} {:?} [{:?}]", self.id, self.labels, self.properties)
    }
}

#[cfg(test)]
mod tests {
    use tracing::debug;
    use tracing_test::traced_test;

    use crate::{
        card::Card,
        key::Key,
        label_property_spec::LabelPropertySpec,
        pgs::PropertyGraphSchema,
        property_value_spec::{PropertyValue, PropertyValueSpec, TypeSpec},
        value::Value,
    };

    use super::*;

    #[test]
    fn test_simple_record_alice_non_optional() {
        let alice = Node::new(NodeId::new(1)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::int(42))
                .with_key_value("aliases", Value::str("Ally")),
        );

        // Wrong label
        let alice_wrong1 = Node::new(NodeId::new(1)).with_label("Other").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::int(42))
                .with_key_value("aliases", Value::str("Ally")),
        );

        // Wrong age type
        let alice_wrong2 = Node::new(NodeId::new(1)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::str("Other"))
                .with_key_value("aliases", Value::str("Ally")),
        );

        // No age
        let alice_wrong3 = Node::new(NodeId::new(1)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("aliases", Value::str("Ally")),
        );

        let mut graph = PropertyGraphSchema::new();
        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let age = PropertyValue::property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases = PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
        let _ = graph.add_node_spec(
            "PersonType",
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );

        let property_value_spec = graph.get_node_semantics("PersonType").unwrap();
        let semantics = property_value_spec.semantics(&graph).unwrap();
        debug!("Semantics of person type: {:?}", semantics);

        assert!(graph.conforms_node(&"PersonType".to_string(), &alice).is_right());
        assert!(graph.conforms_node(&"PersonType".to_string(), &alice_wrong1).is_left());
        assert!(graph.conforms_node(&"PersonType".to_string(), &alice_wrong2).is_left());
        assert!(graph.conforms_node(&"PersonType".to_string(), &alice_wrong3).is_left())
    }

    #[test]
    fn test_simple_record_alice_wrong() {
        let alice = Node::new(NodeId::new(1)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::str("other")),
        );

        let mut graph = PropertyGraphSchema::new();
        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let age = PropertyValue::property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases = PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
        let _ = graph.add_node_spec(
            "PersonType",
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );
        assert!(graph.conforms_node(&"PersonType".to_string(), &alice).is_left(),)
    }

    #[test]
    fn test_simple_record_alice_optional_age_ok() {
        let alice = Node::new(NodeId::new(1)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::int(42))
                .with_key_value("aliases", Value::str("Ally")),
        );

        let bob = Node::new(NodeId::new(2)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Bob"))
                .with_key_value("aliases", Value::str("Bobby")),
        );

        let mut graph = PropertyGraphSchema::new();
        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let age = PropertyValue::optional_property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases = PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
        let _ = graph.add_node_spec(
            "PersonType",
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );
        assert!(graph.conforms_node(&"PersonType".to_string(), &alice).is_right());
        assert!(graph.conforms_node(&"PersonType".to_string(), &bob).is_right())
    }

    #[traced_test]
    #[test]
    fn test_each_of_one_of() {
        let alice = Node::new(NodeId::new(1)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("aliases", Value::str("Ally")),
        );

        let bob = Node::new(NodeId::new(2)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("first_name", Value::str("Robert"))
                .with_key_value("last_name", Value::str("Smith"))
                .with_key_value("aliases", Value::str("Bob"))
                .with_key_value("aliases", Value::str("Bobby")),
        );

        let wrong1 = Node::new(NodeId::new(3)).with_label("Person").with_content(
            &Record::new()
                .with_key_value("first_name", Value::str("Robert"))
                .with_key_value("name", Value::str("extra_name"))
                .with_key_value("aliases", Value::str("Bob"))
                .with_key_value("aliases", Value::str("Bobby")),
        );

        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let first_name = PropertyValue::property(Key::new("first_name"), TypeSpec::string(Card::One));
        let last_name = PropertyValue::property(Key::new("last_name"), TypeSpec::string(Card::One));
        let age = PropertyValue::optional_property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases = PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(
            PropertyValue::one_of(name, PropertyValue::each_of(first_name, last_name)),
            PropertyValue::each_of(age, aliases),
        );
        let mut graph = PropertyGraphSchema::new();
        let _ = graph.add_node_spec(
            "PersonType",
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );

        assert!(graph.conforms_node(&"PersonType".to_string(), &alice).is_right());

        assert!(graph.conforms_node(&"PersonType".to_string(), &bob).is_right());

        assert!(graph.conforms_node(&"PersonType".to_string(), &wrong1).is_left());
    }
}

use super::Name;
use super::NodeId;
use super::ShEx2UmlConfig;
use super::UmlCardinality;
use super::UmlComponent;
use super::UmlEntry;
use super::UmlError;
use super::UmlLink;
use super::ValueConstraint;
use rudof_viz::backends::plantuml::PlantUmlBackend;
use rudof_viz::{BoxId, Connector, ConnectorKind, Diagram, DiagramBox, DiagramRenderer, Shape};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::*;
use std::hash::Hash;
use std::io::Write;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UmlLabelType {
    Class,
    Or,
    Not,
    And,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UmlLabel {
    Class(String),
    Or(usize),
    Not(usize),
    And(usize),
}

impl UmlLabel {
    pub fn label_type(&self) -> UmlLabelType {
        match self {
            UmlLabel::Class(_) => UmlLabelType::Class,
            UmlLabel::Or(_) => UmlLabelType::Or,
            UmlLabel::Not(_) => UmlLabelType::Not,
            UmlLabel::And(_) => UmlLabelType::And,
        }
    }

    pub fn mk_logical_label(label_type: &UmlLabelType, idx: usize) -> UmlLabel {
        match label_type {
            UmlLabelType::Class => panic!("Cannot create a logical label with type Class for idx {idx}"),
            UmlLabelType::Or => UmlLabel::Or(idx),
            UmlLabelType::Not => UmlLabel::Not(idx),
            UmlLabelType::And => UmlLabel::And(idx),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct LogicalComponent {
    label_type: UmlLabelType,
    members: BTreeSet<NodeId>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Uml {
    /// Counter to generate new node ids
    labels_counter: usize,

    /// Counter to generate new node ids for logical components
    logical_components_counter: usize,

    /// Logical components store
    logical_components: HashMap<LogicalComponent, usize>,

    /// Associates a label with a node
    labels: HashMap<UmlLabel, NodeId>,

    /// Associates a node with an UmlComponent
    components: HashMap<NodeId, UmlComponent>,

    /// List of links
    links: Vec<UmlLink>,

    /// Contains a map that keeps track of all the parents of a node
    extends: HashMap<NodeId, HashSet<NodeId>>,

    /// Outgoing arcs
    outgoing: HashMap<NodeId, HashSet<NodeId>>,

    /// Incoming arcs
    incoming: HashMap<NodeId, HashSet<NodeId>>,
}

impl Uml {
    pub fn new() -> Uml {
        Default::default()
    }

    pub fn get_logical_component_idx(&mut self, nodes: &BTreeSet<NodeId>, label_type: &UmlLabelType) -> usize {
        let logical_component = LogicalComponent {
            label_type: label_type.clone(),
            members: nodes.clone(),
        };
        match self.logical_components.entry(logical_component.clone()) {
            Entry::Occupied(c) => *c.get(),
            Entry::Vacant(v) => {
                self.logical_components_counter += 1;
                v.insert(self.logical_components_counter);
                self.logical_components_counter
            },
        }
    }

    /// Tries to get a node from a label. If it exists returns the node and true, otherwise, adds the node and returns false
    pub fn get_node_adding_label(&mut self, label: &UmlLabel) -> (NodeId, bool) {
        match self.labels.entry(label.clone()) {
            Entry::Occupied(c) => (*c.get(), true),
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let n = NodeId::new(self.labels_counter);
                v.insert(n);
                (n, false)
            },
        }
    }

    /// Search a node from a label. If it does not exist, returno `None``
    pub fn get_node(&self, label: &UmlLabel) -> Option<NodeId> {
        self.labels.get(label).copied()
    }

    pub fn add_component(&mut self, node: NodeId, component: UmlComponent) -> Result<(), UmlError> {
        match self.components.entry(node) {
            Entry::Occupied(_c) => Err(UmlError::NodeIdHasComponent { node_id: node }),
            Entry::Vacant(v) => {
                v.insert(component);
                Ok(())
            },
        }
    }

    pub fn get_component(&self, node: &NodeId) -> Option<&UmlComponent> {
        self.components.get(node)
    }

    pub fn update_component(&mut self, node: NodeId, component: UmlComponent) -> Result<(), UmlError> {
        if let Some(r) = self.components.get_mut(&node) {
            *r = component
        } else {
            self.components.insert(node, component);
        }
        Ok(())
    }

    pub fn children<'a>(&'a self, node: &'a NodeId) -> impl Iterator<Item = (&'a NodeId, &'a UmlComponent)> {
        self.components.iter().filter(|(node_id, _component)| {
            if let Some(es) = self.extends.get(node_id) {
                es.contains(node)
            } else {
                false
            }
        })
    }

    pub fn add_link(
        &mut self,
        source: NodeId,
        target: UmlLabel,
        link_name: Name,
        card: UmlCardinality,
    ) -> Result<(), UmlError> {
        match self.labels.entry(target) {
            Entry::Occupied(entry) => {
                let target = *entry.get();
                self.make_link(source, target, link_name, card);
                Ok(())
            },
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let target_node_id = NodeId::new(self.labels_counter);
                v.insert(target_node_id);
                self.make_link(source, target_node_id, link_name, card);
                Ok(())
            },
        }
    }

    pub fn make_link(&mut self, source: NodeId, target: NodeId, name: Name, card: UmlCardinality) {
        let link = UmlLink::new(source, target, name, card);
        self.links.push(link);
        insert_map(&mut self.outgoing, source, target);
        insert_map(&mut self.incoming, target, source);
    }

    pub fn add_extends(&mut self, source: &NodeId, target: &NodeId) {
        match self.extends.entry(*source) {
            Entry::Occupied(mut v) => {
                v.get_mut().insert(*target);
            },
            Entry::Vacant(vacant) => {
                vacant.insert(HashSet::from([*target]));
            },
        }
    }

    pub fn extends(&self) -> impl Iterator<Item = (&NodeId, &NodeId)> {
        self.extends
            .iter()
            .flat_map(|(n1, vs)| vs.iter().map(move |n2| (n1, n2)))
    }

    /// Builds the technology-agnostic [`Diagram`] for this UML model: one [`Shape::Class`] box
    /// per component (attributes as compartments), one connector per link (cardinality shown as
    /// the target decoration), and one generalization connector per `EXTENDS` relationship.
    pub fn to_diagram(&self, config: &ShEx2UmlConfig) -> Diagram {
        let mut diagram = Diagram::new()
            .with_hide_empty_members(true)
            .with_hide_circles(true)
            .with_direction(*config.direction())
            .with_line_type(*config.line_type())
            .with_shadowing(config.shadowing())
            .with_class_skin(*config.class_skin());

        for (node_id, component) in self.components.iter() {
            diagram.add_box(component_to_diagram_box(node_id, component, config));
        }
        for link in self.links.iter() {
            diagram.add_connector(link_to_connector(link, config));
        }
        for (n1, n2) in self.extends() {
            diagram.add_connector(Connector::new(
                to_box_id(*n1),
                to_box_id(*n2),
                ConnectorKind::Generalization,
            ));
        }
        diagram
    }

    pub fn as_plantuml_all<W: Write>(&self, config: &ShEx2UmlConfig, writer: &mut W) -> Result<(), UmlError> {
        let diagram = self.to_diagram(config);
        PlantUmlBackend::default().render(&diagram, writer)?;
        Ok(())
    }

    pub fn as_plantuml_neighs<W: Write>(
        &self,
        config: &ShEx2UmlConfig,
        writer: &mut W,
        target_node: &NodeId,
    ) -> Result<(), UmlError> {
        let diagram = self.to_diagram(config).scoped_by_id(to_box_id(*target_node));
        PlantUmlBackend::default().render(&diagram, writer)?;
        Ok(())
    }
}

fn to_box_id(node_id: NodeId) -> BoxId {
    BoxId::new(node_id.as_usize())
}

fn component_to_diagram_box(node_id: &NodeId, component: &UmlComponent, config: &ShEx2UmlConfig) -> DiagramBox {
    let id = to_box_id(*node_id);
    match component {
        UmlComponent::UmlClass(class) => {
            let name = if config.replace_iri_by_label() {
                class.label().unwrap_or_else(|| class.name())
            } else {
                class.name()
            };
            let mut b = DiagramBox::new(id, Shape::Class, name).with_stereotype("(S,#FF7700)");
            if let Some(href) = class.href() {
                b = b.with_href(href);
            }
            let compartments = class
                .entries()
                .map(|entry| entry_to_compartment_line(entry, config))
                .collect();
            b.with_compartments(compartments)
        },
        UmlComponent::Or { exprs: _ } => DiagramBox::new(id, Shape::Class, "OR"),
        UmlComponent::Not { expr: _ } => DiagramBox::new(id, Shape::Class, "NOT"),
        UmlComponent::And { exprs: _ } => DiagramBox::new(id, Shape::Class, "AND"),
    }
}

fn link_to_connector(link: &UmlLink, config: &ShEx2UmlConfig) -> Connector {
    Connector::new(
        to_box_id(link.source),
        to_box_id(link.target),
        ConnectorKind::Association,
    )
    .with_target_decoration(card_to_string(&link.card))
    .with_label(name_to_string(&link.name, config))
}

fn entry_to_compartment_line(entry: &UmlEntry, config: &ShEx2UmlConfig) -> String {
    let property = name_to_string(&entry.name, config);
    let value_constraint = value_constraint_to_string(&entry.value_constraint, config);
    let card = card_to_string(&entry.card);
    format!("{property} : {value_constraint} {card}")
}

fn name_to_string(name: &Name, config: &ShEx2UmlConfig) -> String {
    let str = if config.replace_iri_by_label() {
        if let Some(label) = name.label() {
            label
        } else {
            name.name()
        }
    } else {
        name.name()
    };
    if let Some(href) = name.href() {
        format!("[[{href} {str}]]")
    } else {
        name.name()
    }
}

fn value_constraint_to_string(vc: &ValueConstraint, config: &ShEx2UmlConfig) -> String {
    match vc {
        ValueConstraint::Any => ".".to_string(),
        ValueConstraint::Datatype(dt) => name_to_string(dt, config),
        ValueConstraint::Ref(r) => format!("@{}", name_to_string(r, config)),
        ValueConstraint::None => "".to_string(),
        ValueConstraint::ValueSet(values) => {
            let mut str = String::new();
            str.push_str("[ ");
            for name in values {
                let name_str = name_to_string(name, config);
                if !str.is_empty() {
                    str.push(' ');
                }
                str.push_str(name_str.as_str());
            }
            str.push_str(" ]");
            str.to_string()
        },
        ValueConstraint::Facet(names) => {
            let mut str = String::new();
            for name in names {
                let name_str = name_to_string(name, config);
                if !str.is_empty() {
                    str.push(' ');
                }
                str.push_str(name_str.as_str());
            }
            str.to_string()
        },
        ValueConstraint::Kind(name) => name_to_string(name, config),
        ValueConstraint::And { values } => values.iter().fold(String::new(), |mut acc, vc| {
            let vc_str = value_constraint_to_string(vc, config);
            if !acc.is_empty() {
                acc.push_str(" AND ");
            }
            acc.push_str(vc_str.as_str());
            acc
        }),
        ValueConstraint::Or { values } => values.iter().fold(String::new(), |mut acc, vc| {
            let vc_str = value_constraint_to_string(vc, config);
            if !acc.is_empty() {
                acc.push_str(" OR ");
            }
            acc.push_str(vc_str.as_str());
            acc
        }),
        ValueConstraint::Not { value } => {
            let vc_str = value_constraint_to_string(value, config);
            format!("NOT {vc_str}")
        },
    }
}

fn card_to_string(card: &UmlCardinality) -> String {
    match card {
        UmlCardinality::OneOne => " ".to_string(),
        UmlCardinality::Star => "*".to_string(),
        UmlCardinality::Plus => "+".to_string(),
        UmlCardinality::Optional => "?".to_string(),
        UmlCardinality::Range(m, n) => format!("{m}-{n}"),
        UmlCardinality::Fixed(m) => format!("{{{m}}}"),
    }
}

fn insert_map<A, B>(map: &mut HashMap<A, HashSet<B>>, source: A, target: B)
where
    A: Eq + Hash,
    B: Eq + Hash,
{
    match map.entry(source) {
        Entry::Occupied(mut entry) => {
            let set = entry.get_mut();
            set.insert(target);
        },
        Entry::Vacant(v) => {
            v.insert(HashSet::from([target]));
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn person_uml() -> (Uml, NodeId) {
        let mut uml = Uml::new();
        let mut person = super::super::UmlClass::new(Name::new(":Person", Some("http://example.org/Person")));
        person.add_entry(UmlEntry::new(
            Name::new(":name", Some("http://example.org/name")),
            ValueConstraint::datatype(Name::new("xsd:string", Some("http://www.w3.org/2001/XMLSchema#string"))),
            UmlCardinality::OneOne,
        ));
        let (person_node, _found) = uml.get_node_adding_label(&UmlLabel::Class(":Person".to_string()));
        uml.add_component(person_node, UmlComponent::class(person)).unwrap();
        (uml, person_node)
    }

    #[test]
    fn to_diagram_builds_one_class_box_with_a_compartment() {
        let (uml, _person_node) = person_uml();
        let diagram = uml.to_diagram(&ShEx2UmlConfig::default());

        assert_eq!(diagram.boxes().count(), 1);
        let b = diagram.boxes().next().unwrap();
        assert_eq!(b.shape(), Shape::Class);
        assert_eq!(b.title(), ":Person");
        assert_eq!(b.stereotype(), Some("(S,#FF7700)"));
        assert_eq!(b.compartments().len(), 1);
        assert!(b.compartments()[0].contains(":name"));
        assert!(b.compartments()[0].contains("xsd:string"));
        assert!(diagram.hide_empty_members());
        assert!(diagram.hide_circles());
        assert_eq!(diagram.shadowing(), Some(true));
    }

    #[test]
    fn to_diagram_builds_a_generalization_connector_for_extends() {
        let mut uml = Uml::new();
        let (child, _) = uml.get_node_adding_label(&UmlLabel::Class(":Employee".to_string()));
        let (parent, _) = uml.get_node_adding_label(&UmlLabel::Class(":Person".to_string()));
        uml.add_component(
            child,
            UmlComponent::class(super::super::UmlClass::new(Name::new(":Employee", None))),
        )
        .unwrap();
        uml.add_component(
            parent,
            UmlComponent::class(super::super::UmlClass::new(Name::new(":Person", None))),
        )
        .unwrap();
        uml.add_extends(&child, &parent);

        let diagram = uml.to_diagram(&ShEx2UmlConfig::default());
        let connector = diagram.connectors().next().unwrap();
        assert_eq!(connector.kind(), ConnectorKind::Generalization);
        assert_eq!(connector.source(), to_box_id(child));
        assert_eq!(connector.target(), to_box_id(parent));
    }

    #[test]
    fn as_plantuml_all_renders_valid_plantuml_for_a_class() {
        let (uml, _person_node) = person_uml();
        let mut out = Vec::new();
        uml.as_plantuml_all(&ShEx2UmlConfig::default(), &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();

        assert!(text.starts_with("@startuml"));
        assert!(text.trim_end().ends_with("@enduml"));
        assert!(text.contains(":Person"));
        assert!(text.contains("(S,#FF7700)"));
        assert!(text.contains("hide empty members"));
    }

    #[test]
    fn as_plantuml_neighs_scopes_to_the_target_and_its_links() {
        let mut uml = Uml::new();
        let (person, _) = uml.get_node_adding_label(&UmlLabel::Class(":Person".to_string()));
        let (company, _) = uml.get_node_adding_label(&UmlLabel::Class(":Company".to_string()));
        let (unrelated, _) = uml.get_node_adding_label(&UmlLabel::Class(":Unrelated".to_string()));
        uml.add_component(
            person,
            UmlComponent::class(super::super::UmlClass::new(Name::new(":Person", None))),
        )
        .unwrap();
        uml.add_component(
            company,
            UmlComponent::class(super::super::UmlClass::new(Name::new(":Company", None))),
        )
        .unwrap();
        uml.add_component(
            unrelated,
            UmlComponent::class(super::super::UmlClass::new(Name::new(":Unrelated", None))),
        )
        .unwrap();
        uml.make_link(person, company, Name::new(":worksFor", None), UmlCardinality::Star);

        let mut out = Vec::new();
        uml.as_plantuml_neighs(&ShEx2UmlConfig::default(), &mut out, &person)
            .unwrap();
        let text = String::from_utf8(out).unwrap();

        assert!(text.contains(":Person"));
        assert!(text.contains(":Company"));
        assert!(!text.contains(":Unrelated"));
    }
}

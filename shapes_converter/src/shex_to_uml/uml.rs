use super::Name;
use super::NodeId;
use super::ShEx2UmlConfig;
use super::UmlCardinality;
use super::UmlComponent;
use super::UmlEntry;
use super::UmlError;
use super::UmlLink;
use super::ValueConstraint;
use serde::Deserialize;
use serde::Serialize;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Default)]
pub enum LineType {
    Orthogonal,
    Polyline,
    #[default]
    Default,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Default)]
pub enum Direction {
    LeftToRight,
    #[default]
    TopToBottom,
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

    pub fn as_plantuml_all<W: Write>(&self, config: &ShEx2UmlConfig, writer: &mut W) -> Result<(), UmlError> {
        writeln!(writer, "@startuml")?;
        self.preamble(writer, config)?;
        for (node_id, component) in self.components.iter() {
            component2plantuml(node_id, component, config, writer)?;
        }
        for link in self.links.iter() {
            link2plantuml(link, config, writer)?;
        }
        for (n1, n2) in self.extends() {
            writeln!(writer, "{n1} -|> {n2}")?;
        }
        writeln!(writer, "@enduml")?;
        Ok(())
    }

    pub fn as_plantuml_neighs<W: Write>(
        &self,
        config: &ShEx2UmlConfig,
        writer: &mut W,
        target_node: &NodeId,
    ) -> Result<(), UmlError> {
        writeln!(writer, "@startuml")?;
        self.preamble(writer, config)?;

        // Keep track of serialized components to avoid serializing them twice
        let mut serialized_components = HashSet::new();

        // For all components in schema, check if they are neighbours with target_node
        for (node_id, component) in self.components.iter() {
            if node_id == target_node
                || is_in_extends(&self.extends, node_id, target_node)
                || is_in_extends(&self.extends, target_node, node_id)
                || is_in_map(&self.outgoing, target_node, node_id)
                || is_in_map(&self.incoming, target_node, node_id) && !serialized_components.contains(node_id)
            {
                serialized_components.insert(node_id);
                component2plantuml(node_id, component, config, writer)?;
            }
        }
        for link in self.links.iter() {
            if link.source == *target_node || link.target == *target_node {
                link2plantuml(link, config, writer)?;
            }
        }
        for (n1, n2) in self.extends() {
            if n1 == target_node || n2 == target_node {
                writeln!(writer, "{n1} -|> {n2}")?;
            }
        }
        writeln!(writer, "@enduml")?;
        Ok(())
    }

    fn preamble(&self, writer: &mut impl Write, config: &ShEx2UmlConfig) -> Result<(), UmlError> {
        writeln!(writer, "hide empty members")?;

        match config.direction.clone().unwrap_or_default() {
            Direction::LeftToRight => {
                writeln!(writer, "left to right direction")?;
            },
            Direction::TopToBottom => {
                writeln!(writer, "top to bottom direction")?;
            },
        }

        match config.line_type.clone().unwrap_or_default() {
            LineType::Orthogonal => {
                writeln!(writer, "skinparam linetype ortho")?;
            },
            LineType::Polyline => {
                writeln!(writer, "skinparam linetype polyline")?;
            },
            LineType::Default => {},
        }

        // Hide the class attribute icon
        writeln!(writer, "hide circles")?;

        writeln!(writer, "skinparam shadowing {}", config.shadowing.unwrap_or_default())?;

        // The following parameters should be taken from the ocnfig file...
        writeln!(writer, "skinparam class {{")?;
        writeln!(writer, " BorderColor Black")?;
        writeln!(writer, " ArrowColor Black")?;
        writeln!(writer, "}}")?;
        Ok(())
    }
}

fn component2plantuml<W: Write>(
    node_id: &NodeId,
    component: &UmlComponent,
    config: &ShEx2UmlConfig,
    writer: &mut W,
) -> Result<(), UmlError> {
    match component {
        UmlComponent::UmlClass(class) => {
            let name = if config.replace_iri_by_label() {
                if let Some(label) = class.label() {
                    label
                } else {
                    class.name()
                }
            } else {
                class.name()
            };
            let href = if let Some(href) = class.href() {
                format!("[[{href} {name}]]")
            } else {
                "".to_string()
            };
            writeln!(writer, "class \"{name}\" as {node_id} <<(S,#FF7700)>> {href} {{ ")?;
            for entry in class.entries() {
                entry2plantuml(entry, config, writer)?;
            }
            writeln!(writer, "}}")?;
        },
        UmlComponent::Or { exprs: _ } => {
            writeln!(writer, "class \"OR\" as {node_id} {{}}")?;
        },
        UmlComponent::Not { expr: _ } => {
            writeln!(writer, "class \"NOT\" as {node_id} {{}}")?;
        },
        UmlComponent::And { exprs: _ } => {
            writeln!(writer, "class \"AND\" as {node_id} {{}}")?;
        },
    }
    Ok(())
}

fn link2plantuml<W: Write>(link: &UmlLink, config: &ShEx2UmlConfig, writer: &mut W) -> Result<(), UmlError> {
    let source = format!("{}", link.source);
    let card = card2plantuml(&link.card);
    let target = format!("{}", link.target);
    let name = name2plantuml(&link.name, config);
    writeln!(writer, "{source} --> \"{card}\" {target} : {name}")?;
    Ok(())
}

fn entry2plantuml<W: Write>(entry: &UmlEntry, config: &ShEx2UmlConfig, writer: &mut W) -> Result<(), UmlError> {
    let property = name2plantuml(&entry.name, config);
    let value_constraint = value_constraint2plantuml(&entry.value_constraint, config);
    let card = card2plantuml(&entry.card);
    writeln!(writer, "{property} : {value_constraint} {card}")?;
    writeln!(writer, "--")?;
    Ok(())
}

fn name2plantuml(name: &Name, config: &ShEx2UmlConfig) -> String {
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

fn value_constraint2plantuml(vc: &ValueConstraint, config: &ShEx2UmlConfig) -> String {
    match vc {
        ValueConstraint::Any => ".".to_string(),
        ValueConstraint::Datatype(dt) => name2plantuml(dt, config),
        ValueConstraint::Ref(r) => format!("@{}", name2plantuml(r, config)),
        ValueConstraint::None => "".to_string(),
        ValueConstraint::ValueSet(values) => {
            let mut str = String::new();
            str.push_str("[ ");
            for name in values {
                let name_puml = name2plantuml(name, config);
                if !str.is_empty() {
                    str.push(' ');
                }
                str.push_str(name_puml.as_str());
            }
            str.push_str(" ]");
            str.to_string()
        },
        ValueConstraint::Facet(names) => {
            let mut str = String::new();
            for name in names {
                let name_puml = name2plantuml(name, config);
                if !str.is_empty() {
                    str.push(' ');
                }
                str.push_str(name_puml.as_str());
            }
            str.to_string()
        },
        ValueConstraint::Kind(name) => name2plantuml(name, config),
        ValueConstraint::And { values } => values.iter().fold(String::new(), |mut acc, vc| {
            let vc_str = value_constraint2plantuml(vc, config);
            if !acc.is_empty() {
                acc.push_str(" AND ");
            }
            acc.push_str(vc_str.as_str());
            acc
        }),
        ValueConstraint::Or { values } => values.iter().fold(String::new(), |mut acc, vc| {
            let vc_str = value_constraint2plantuml(vc, config);
            if !acc.is_empty() {
                acc.push_str(" OR ");
            }
            acc.push_str(vc_str.as_str());
            acc
        }),
        ValueConstraint::Not { value } => {
            let vc_str = value_constraint2plantuml(value, config);
            format!("NOT {vc_str}")
        },
    }
}

fn card2plantuml(card: &UmlCardinality) -> String {
    match card {
        UmlCardinality::OneOne => " ".to_string(),
        UmlCardinality::Star => "*".to_string(),
        UmlCardinality::Plus => "+".to_string(),
        UmlCardinality::Optional => "?".to_string(),
        UmlCardinality::Range(m, n) => format!("{m}-{n}"),
        UmlCardinality::Fixed(m) => format!("{{{m}}}"),
    }
}

fn is_in_extends(extends: &HashMap<NodeId, HashSet<NodeId>>, node: &NodeId, target: &NodeId) -> bool {
    if let Some(es) = extends.get(node) {
        es.contains(target)
    } else {
        false
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

fn is_in_map<A, B>(map: &HashMap<A, HashSet<B>>, source: &A, target: &B) -> bool
where
    A: Eq + Hash,
    B: Eq + Hash,
{
    if let Some(es) = map.get(source) {
        es.contains(target)
    } else {
        false
    }
}

use super::Name;
use super::NodeId;
use super::ShEx2UmlConfig;
use super::UmlCardinality;
use super::UmlComponent;
use super::UmlEntry;
use super::UmlError;
use super::UmlLink;
use super::ValueConstraint;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::*;
use std::hash::Hash;
use std::io::Write;

#[derive(Debug, PartialEq, Default)]
pub struct Uml {
    labels_counter: usize,

    labels: HashMap<String, NodeId>,

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

    /// Tries to get a node from a label. If it exists returns the node and true, otherwise, adds the node and returns false
    pub fn get_node_adding_label(&mut self, label: &str) -> (NodeId, bool) {
        match self.labels.entry(label.to_string()) {
            Entry::Occupied(c) => (*c.get(), true),
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let n = NodeId::new(self.labels_counter);
                v.insert(n);
                (n, false)
            }
        }
    }

    /// Search a node from a label. If it does not exist, returno `None``
    pub fn get_node(&self, label: &str) -> Option<NodeId> {
        self.labels.get(label).copied()
    }

    pub fn add_component(&mut self, node: NodeId, component: UmlComponent) -> Result<(), UmlError> {
        match self.components.entry(node) {
            Entry::Occupied(_c) => Err(UmlError::NodeIdHasComponent { node_id: node }),
            Entry::Vacant(v) => {
                v.insert(component);
                Ok(())
            }
        }
    }

    pub fn update_component(
        &mut self,
        node: NodeId,
        component: UmlComponent,
    ) -> Result<(), UmlError> {
        if let Some(r) = self.components.get_mut(&node) {
            *r = component
        } else {
            self.components.insert(node, component);
        }
        Ok(())
    }

    pub fn children<'a>(
        &'a self,
        node: &'a NodeId,
    ) -> impl Iterator<Item = (&'a NodeId, &'a UmlComponent)> {
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
        target: Name,
        link_name: Name,
        card: UmlCardinality,
    ) -> Result<(), UmlError> {
        match self.labels.entry(target.name()) {
            Entry::Occupied(entry) => {
                let target = *entry.get();
                self.make_link(source, target, link_name, card);
                Ok(())
            }
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let target_node_id = NodeId::new(self.labels_counter);
                v.insert(target_node_id);
                self.make_link(source, target_node_id, link_name, card);
                Ok(())
            }
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
            }
            Entry::Vacant(vacant) => {
                vacant.insert(HashSet::from([*target]));
            }
        }
    }

    pub fn extends(&self) -> impl Iterator<Item = (&NodeId, &NodeId)> {
        self.extends
            .iter()
            .flat_map(|(n1, vs)| vs.iter().map(move |n2| (n1, n2)))
    }

    pub fn as_plantuml_all<W: Write>(
        &self,
        config: &ShEx2UmlConfig,
        writer: &mut W,
    ) -> Result<(), UmlError> {
        writeln!(writer, "@startuml")?;
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
        let mut serialized_components = HashSet::new();

        // For all components in schema, check if they are neighbours with target_node
        for (node_id, component) in self.components.iter() {
            if node_id == target_node
                || is_in_extends(&self.extends, node_id, target_node)
                || is_in_extends(&self.extends, target_node, node_id)
                || is_in_map(&self.outgoing, target_node, node_id)
                || is_in_map(&self.incoming, target_node, node_id)
                    && !serialized_components.contains(node_id)
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
            writeln!(
                writer,
                "class \"{name}\" as {node_id} <<(S,#FF7700)>> {href} {{ "
            )?;
            for entry in class.entries() {
                entry2plantuml(entry, config, writer)?;
            }
            writeln!(writer, "}}")?;
        }
    }
    Ok(())
}

fn link2plantuml<W: Write>(
    link: &UmlLink,
    config: &ShEx2UmlConfig,
    writer: &mut W,
) -> Result<(), UmlError> {
    let source = format!("{}", link.source);
    let card = card2plantuml(&link.card);
    let target = format!("{}", link.target);
    let name = name2plantuml(&link.name, config);
    writeln!(writer, "{source} --> \"{card}\" {target} : {name}")?;
    Ok(())
}

fn entry2plantuml<W: Write>(
    entry: &UmlEntry,
    config: &ShEx2UmlConfig,
    writer: &mut W,
) -> Result<(), UmlError> {
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
            for value in values {
                let name_puml = name2plantuml(value, config);
                str.push_str(name_puml.as_str());
                str.push_str(", ");
            }
            str.push_str(" ]");
            str.to_string()
        }
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

fn is_in_extends(
    extends: &HashMap<NodeId, HashSet<NodeId>>,
    node: &NodeId,
    target: &NodeId,
) -> bool {
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
        }
        Entry::Vacant(v) => {
            v.insert(HashSet::from([target]));
        }
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

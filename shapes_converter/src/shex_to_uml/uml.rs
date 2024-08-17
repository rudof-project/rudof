use super::Name;
use super::NodeId;
use super::ShEx2UmlConfig;
use super::UmlCardinality;
use super::UmlComponent;
use super::UmlEntry;
use super::UmlError;
use super::UmlLink;
use super::ValueConstraint;
use std::collections::hash_map::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;

#[derive(Debug, PartialEq, Default)]
pub struct Uml {
    labels_counter: usize,
    labels: HashMap<String, NodeId>,
    components: HashMap<NodeId, UmlComponent>,
    links: Vec<UmlLink>,
    extends: HashMap<NodeId, HashSet<NodeId>>,
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

    pub fn add_link(
        &mut self,
        source: NodeId,
        target: Name,
        link_name: Name,
        card: UmlCardinality,
    ) -> Result<(), UmlError> {
        match self.labels.entry(target.name()) {
            Entry::Occupied(entry) => {
                let link = UmlLink::new(source, *entry.get(), link_name, card);
                self.links.push(link);
                Ok(())
            }
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let target_node_id = NodeId::new(self.labels_counter);
                v.insert(target_node_id);
                let link = UmlLink::new(source, target_node_id, link_name, card);
                self.links.push(link);
                Ok(())
            }
        }
    }

    pub fn add_extends(&mut self, source: &NodeId, target: &NodeId) {
        match self.extends.entry(source.clone()) {
            Entry::Occupied(mut v) => {
                v.get_mut().insert(target.clone());
            }
            Entry::Vacant(vacant) => {
                vacant.insert(HashSet::from([target.clone()]));
            }
        }
    }

    pub fn extends(&self) -> impl Iterator<Item = (&NodeId, &NodeId)> {
        self.extends
            .iter()
            .flat_map(|(n1, vs)| vs.iter().map(move |n2| (n1, n2)))
    }

    pub fn as_plantuml<W: Write>(
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
                "class \"{}\" as {} <<(S,#FF7700)>> {} {{ ",
                name, node_id, href
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
    writeln!(writer, "{} : {} {}", property, value_constraint, card)?;
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
        format!("[[{href} {}]]", str)
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
    }
}

fn card2plantuml(card: &UmlCardinality) -> String {
    match card {
        UmlCardinality::OneOne => "".to_string(),
        UmlCardinality::Star => "*".to_string(),
        UmlCardinality::Plus => "+".to_string(),
        UmlCardinality::Optional => "?".to_string(),
        UmlCardinality::Range(m, n) => format!("{m}-{n}"),
        UmlCardinality::Fixed(m) => format!("{{{m}}}"),
    }
}

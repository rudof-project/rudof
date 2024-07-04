use super::Name;
use super::NodeId;
use super::UmlCardinality;
use super::UmlComponent;
use super::UmlEntry;
use super::UmlError;
use super::UmlLink;
use super::ValueConstraint;
use std::collections::hash_map::*;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq, Default)]
pub struct Uml {
    labels_counter: usize,
    labels: HashMap<Name, NodeId>,
    components: HashMap<NodeId, UmlComponent>,
    links: Vec<UmlLink>,
}

impl Uml {
    pub fn new() -> Uml {
        Default::default()
    }

    pub fn add_label(&mut self, label: &Name) -> NodeId {
        match self.labels.entry(label.clone()) {
            Entry::Occupied(c) => c.get().clone(),
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let n = NodeId::new(self.labels_counter);
                v.insert(n);
                n.clone()
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

    pub fn add_link(
        &mut self,
        source: NodeId,
        target: Name,
        link_name: Name,
        card: UmlCardinality,
    ) -> Result<(), UmlError> {
        match self.labels.entry(target) {
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

    pub fn as_plantuml<W: Write>(&self, writer: &mut W) -> Result<(), UmlError> {
        writeln!(writer, "@startuml")?;
        for (node_id, component) in self.components.iter() {
            component2plantuml(node_id, component, writer)?;
        }
        for link in self.links.iter() {
            link2plantuml(link, writer)?;
        }
        writeln!(writer, "@enduml")?;
        Ok(())
    }
}

fn component2plantuml<W: Write>(
    node_id: &NodeId,
    component: &UmlComponent,
    writer: &mut W,
) -> Result<(), UmlError> {
    match component {
        UmlComponent::UmlClass(class) => {
            let name = class.name();
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
                entry2plantuml(entry, writer)?;
            }
            writeln!(writer, "}}")?;
        }
    }
    Ok(())
}

fn link2plantuml<W: Write>(link: &UmlLink, writer: &mut W) -> Result<(), UmlError> {
    let source = format!("{}", link.source);
    let card = card2plantuml(&link.card);
    let target = format!("{}", link.target);
    let name = name2plantuml(&link.name);
    writeln!(writer, "{source} --> \"{card}\" {target} {name}");
    Ok(())
}

fn entry2plantuml<W: Write>(entry: &UmlEntry, writer: &mut W) -> Result<(), UmlError> {
    let property = name2plantuml(&entry.name);
    let value_constraint = value_constraint2plantuml(&entry.value_constraint);
    let card = card2plantuml(&entry.card);
    writeln!(writer, "{} : {} {}", property, value_constraint, card)?;
    writeln!(writer, "--")?;
    Ok(())
}

fn name2plantuml(name: &Name) -> String {
    if let Some(href) = name.href() {
        format!("[[{href} {}]]", name.name())
    } else {
        name.name()
    }
}

fn value_constraint2plantuml(vc: &ValueConstraint) -> String {
    match vc {
        ValueConstraint::Any => ".".to_string(),
        ValueConstraint::Datatype(dt) => name2plantuml(dt),
        ValueConstraint::Ref(r) => format!("@{}", name2plantuml(r)),
        ValueConstraint::None => "".to_string(),
    }
}

fn card2plantuml(card: &UmlCardinality) -> String {
    match card {
        UmlCardinality::OneOne => format!(""),
        UmlCardinality::Star => format!("*"),
        UmlCardinality::Plus => format!("+"),
        UmlCardinality::Optional => format!("?"),
        UmlCardinality::Range(m, n) => format!("{m}-{n}"),
        UmlCardinality::Fixed(m) => format!("{{{m}}}"),
    }
}

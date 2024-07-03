use super::UmlComponent;
use super::UmlLink;
use std::collections::hash_map::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Uml {
    labels_counter: usize,
    labels: HashMap<Name, NodeId>,
    components: HashMap<NodeId, UmlComponent>,
    links: Vec<UmlLink>,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
pub struct Name {
    str: String,
    href: Option<String>,
}

impl Name {
    pub fn new(str: &str, href: Option<&str>) -> Name {
        Name {
            str: str.to_string(),
            href: if let Some(href) = href {
                Some(href.to_string())
            } else {
                None
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct NodeId {
    n: usize,
}

impl NodeId {
    pub fn new(n: usize) -> NodeId {
        NodeId { n }
    }
}

impl Uml {
    pub fn new() -> Uml {
        Default::default()
    }

    pub fn add_label(&mut self, label: Name) -> NodeId {
        match self.labels.entry(label) {
            Entry::Occupied(c) => c.get().clone(),
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let n = NodeId::new(self.labels_counter);
                v.insert(n);
                n.clone()
            }
        }
    }
}

use colored::*;
use prefixmap::PrefixMap;
use rbe::Pending;
use shex_ast::compiled::shape_label::ShapeLabel;
use shex_ast::Node;
use srdf::Object;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Formatter},
};

use crate::{ResultValue, ValidatorError};

#[derive(Debug, Default)]
pub struct ResultMap {
    nodes_prefixmap: PrefixMap,
    schema_prefixmap: PrefixMap,
    ok_map: HashMap<Node, HashSet<ShapeLabel>>,
    fail_map: HashMap<Node, HashMap<ShapeLabel, Vec<ValidatorError>>>,
    pending: HashMap<Node, HashSet<ShapeLabel>>,
}

impl ResultMap {
    pub fn new() -> ResultMap {
        ResultMap::default()
    }

    pub fn add_ok(&mut self, n: Node, s: ShapeLabel) {
        match self.ok_map.entry(n) {
            Entry::Occupied(mut v) => {
                v.get_mut().insert(s);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(HashSet::from([s]));
            }
        }
    }

    pub fn add_fail(&mut self, n: Node, s: ShapeLabel, maybe_err: Option<ValidatorError>) {
        match self.fail_map.entry(n) {
            Entry::Occupied(mut v) => match v.get_mut().entry(s) {
                Entry::Occupied(mut es) => {
                    if let Some(err) = maybe_err {
                        es.get_mut().push(err)
                    }
                }
                Entry::Vacant(vacant) => {
                    let vs = maybe_err.map_or_else(|| vec![], |e| vec![e]);
                    vacant.insert(vs);
                }
            },
            Entry::Vacant(vacant) => {
                let vs = maybe_err.map_or_else(|| vec![], |e| vec![e]);
                vacant.insert(HashMap::from([(s, vs)]));
            }
        }
    }

    pub fn with_nodes_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.nodes_prefixmap = prefixmap;
        self
    }

    pub fn with_schema_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.schema_prefixmap = prefixmap;
        self
    }

    pub fn add_pending(&mut self, n: Node, s: ShapeLabel) {
        match self.pending.entry(n) {
            Entry::Occupied(mut v) => {
                v.get_mut().insert(s);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(HashSet::from([s]));
            }
        }
    }

    pub fn get_result(&self, node: &Node, shape: &ShapeLabel) -> ResultValue {
        if self.is_ok(node, shape) {
            ResultValue::Ok
        } else if self.is_failed(node, shape) {
            ResultValue::Failed
        } else if self.is_pending(node, shape) {
            ResultValue::Pending
        } else {
            ResultValue::Unknown
        }
    }

    pub fn is_ok(&self, node: &Node, shape: &ShapeLabel) -> bool {
        if let Some(hs) = self.ok_map.get(node) {
            hs.contains(shape)
        } else {
            false
        }
    }

    pub fn is_failed(&self, node: &Node, shape: &ShapeLabel) -> bool {
        if let Some(hs) = self.fail_map.get(node) {
            hs.contains_key(shape)
        } else {
            false
        }
    }

    pub fn is_pending(&self, node: &Node, shape: &ShapeLabel) -> bool {
        if let Some(hs) = self.pending.get(node) {
            hs.contains(shape)
        } else {
            false
        }
    }
}

fn show_node(node: &Node, prefixmap: &PrefixMap) -> String {
    match node.as_object() {
        Object::Iri { iri } => prefixmap.qualify(iri),
        _ => format!("{node}"),
    }
}

fn show_shapelabel(shapelabel: &ShapeLabel, prefixmap: &PrefixMap) -> String {
    match shapelabel {
        ShapeLabel::Iri(iri) => prefixmap.qualify(iri),
        ShapeLabel::BNode(str) => format!("_:{str}"),
        ShapeLabel::Start => "Start".to_owned(),
    }
}

impl Display for ResultMap {
    fn fmt(&self, dest: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for (n, hs) in &self.ok_map {
            write!(dest, "{}@+|", show_node(n, &self.nodes_prefixmap))?;
            for s in hs {
                write!(dest, "{}|", show_shapelabel(s, &self.schema_prefixmap))?;
            }
            writeln!(dest)?;
        }
        writeln!(dest)?;
        for (n, hs) in &self.fail_map {
            for (s, errs) in hs.iter() {
                let s = format!(
                    "{}-> NOT {}",
                    show_node(n, &self.nodes_prefixmap),
                    show_shapelabel(s, &self.schema_prefixmap)
                );
                write!(dest, "{}", s.red())?;
                for e in errs {
                    writeln!(dest, "  Err:{e}")?;
                }
            }
            writeln!(dest)?;
        }
        writeln!(dest)?;
        for (n, hs) in &self.pending {
            write!(dest, "{}->Pending |", show_node(n, &self.nodes_prefixmap))?;
            for s in hs {
                write!(dest, "{}|", show_shapelabel(s, &self.schema_prefixmap))?;
            }
            writeln!(dest)?;
        }
        Ok(())
    }
}

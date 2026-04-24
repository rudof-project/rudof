use std::{collections::HashMap, fmt::Display};

use rudof_iri::IriS;
use serde::{Deserialize, Serialize};

use crate::Node;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct MapState {
    map: HashMap<IriS, Node>,
}

impl MapState {
    pub fn insert(&mut self, key: IriS, value: Node) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &IriS) -> Option<&Node> {
        self.map.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Display for MapState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapState {{ map: {:?} }}", self.map)
    }
}

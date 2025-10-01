use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestMap(Vec<ManifestMapEntry>);

impl ManifestMap {
    pub fn entries(&self) -> &Vec<ManifestMapEntry> {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestMapEntry {
    node: String,
    shape: String,
}

impl ManifestMapEntry {
    pub fn new(node: String, shape: String) -> Self {
        ManifestMapEntry { node, shape }
    }

    pub fn node(&self) -> &str {
        &self.node
    }

    pub fn shape(&self) -> &str {
        &self.shape
    }
}

#[cfg(test)]
mod tests {
    use crate::manifest_map::ManifestMap;

    #[test]
    fn check_serialize_deserialize() {
        let str = r#"[
         { "node" : "http://example.org/n1", "shape" : "http://example.org/s1" },
         { "node" : "http://example.org/n2", "shape" : "http://example.org/s2" }
         ]"#;
        let map = serde_json::from_str::<ManifestMap>(str).unwrap();
        assert_eq!(map.0.len(), 2);
        assert_eq!(map.0[0].node, "http://example.org/n1");
        assert_eq!(map.0[0].shape, "http://example.org/s1");
        assert_eq!(map.0[1].node, "http://example.org/n2");
        assert_eq!(map.0[1].shape, "http://example.org/s2");
    }
}

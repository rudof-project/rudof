use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufReader},
    path::Path,
    str::FromStr,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PrefixCC {
    #[serde(rename = "@context")]
    context: HashMap<String, String>,
}

impl PrefixCC {
    pub fn from_reader<R: io::Read>(reader: R) -> Result<PrefixCC, Box<dyn Error>> {
        let p: PrefixCC = serde_json::from_reader(reader)?;
        Ok(p)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<PrefixCC, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let p: PrefixCC = Self::from_reader(reader)?;
        Ok(p)
    }

    pub fn get(&self, alias: &str) -> Option<String> {
        self.context.get(alias).cloned()
    }
}

impl FromStr for PrefixCC {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: PrefixCC = serde_json::from_str(s)?;
        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::PrefixCC;
    use std::str::FromStr;

    #[test]
    fn test_prefixcc_simple() {
        let data = r#"{ "@context": {
        "foaf": "http://xmlns.com/foaf/0.1/",
        "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
        "dbo": "http://dbpedia.org/ontology/",
        "rdfs": "http://www.w3.org/2000/01/rdf-schema#"
       }
     }"#;
        let prefix_cc = PrefixCC::from_str(data).unwrap();
        assert_eq!(
            prefix_cc.get("rdf"),
            Some("http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string())
        )
    }

    #[test]
    fn test_prefixcc_file() {
        let mut current_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let path = Path::new("config/prefix_cc_context.jsonld");
        current_exe.push(path);
        let prefix_cc = PrefixCC::from_path(current_exe).unwrap();
        assert_eq!(
            prefix_cc.get("rdf"),
            Some("http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string())
        )
    }
}

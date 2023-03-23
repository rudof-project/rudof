use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Manifest {
    graph: Vec<ManifestGraph>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestGraph {
    id: String, 
    tipe: String, 
    comment: String,
    entries: Vec<Entry>
}

#[derive(Deserialize, Serialize, Debug)]
struct Entry {
    id: String,
    tipe: String,
    name: String,
    status: String,
    shex: String,
    json: String,
    ttl: String
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;

    #[test]
    fn list_all_files() {
        let manifest_path = Path::new("localTest/schemas/manifest_basic.jsonld");
        let mut manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<Manifest>(&manifest_str).unwrap()
        };

        println!("Manifest read: {:?}", manifest);
        
        assert_eq!(2 + 2, 4);
    }
}

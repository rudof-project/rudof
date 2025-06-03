// Field value generator for synthetic RDF triples

use rand::Rng;

pub struct FieldGenerator;

impl FieldGenerator {
    pub fn new() -> Self {
        FieldGenerator
    }

    /// Generate a dummy value for a field, optionally based on a datatype IRI
    pub fn generate_value(&self, datatype: Option<&str>) -> String {
        match datatype {
            Some(dt) if dt.contains("string") => {
                format!("\"SyntheticString{}\"", rand::thread_rng().gen_range(1..1000))
            }
            Some(dt) if dt.contains("int") || dt.contains("integer") => {
                format!("{}", rand::thread_rng().gen_range(1..1000))
            }
            Some(dt) if dt.contains("date") => {
                "\"2025-06-03\"^^<http://www.w3.org/2001/XMLSchema#date>".to_string()
            }
            Some(dt) if dt.contains("boolean") => {
                if rand::thread_rng().gen_bool(0.5) {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            _ => "\"dummyValue\"".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_value() {
        let fg = FieldGenerator::new();
        let s = fg.generate_value(Some("string"));
        assert!(s.contains("SyntheticString") || s == "\"dummyValue\"");
        let i = fg.generate_value(Some("integer"));
        assert!(i.parse::<i32>().is_ok() || i == "\"dummyValue\"");
        let d = fg.generate_value(Some("date"));
        assert!(d.contains("2025-06-03") || d == "\"dummyValue\"");
    }
}

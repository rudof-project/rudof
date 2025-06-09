// Field value generator for synthetic RDF triples

use rand::Rng;
use rand::seq::SliceRandom; // Added import
use crate::generator::FieldGeneratorTrait;

// Added constants for varied data generation
const SAMPLE_FIRST_NAMES: &[&str] = &["Alice", "Bob", "Charlie", "Diego", "Edward", "Fiona", "George", "Hannah", "Ian", "Julia"];
const SAMPLE_LAST_NAMES: &[&str] = &["Smith", "Jones", "Williams", "Brown", "Davis", "Miller", "Wilson", "Martin Fernandez", "Taylor", "Anderson"];
const SAMPLE_WORDS: &[&str] = &["System", "Project", "Data", "Research", "Development", "Analysis", "Integration", "Framework", "Solution", "Platform"];
const SAMPLE_COURSE_ADJ: &[&str] = &["Introduction to", "Advanced", "Applied", "Theoretical", "Modern", "Computational", "Principles of"];
const SAMPLE_COURSE_NOUN: &[&str] = &["Computer Science", "Physics", "Mathematics", "Literature", "History", "Engineering", "Biology", "Economics"];
const SAMPLE_ORG_PREFIX: &[&str] = &["Global", "National", "International", "Universal", "Advanced", "NextGen", "Innovative"];
const SAMPLE_ORG_SUFFIX: &[&str] = &["Solutions", "Systems", "Corp", "Inc", "Group", "Technologies", "Labs", "Enterprises"];
const DOMAINS: &[&str] = &["example.com", "example.org", "example.net", "company.xyz", "research.edu"];

pub struct BasicFieldGeneratorImpl;

// Implement the new trait for BasicFieldGeneratorImpl
impl FieldGeneratorTrait for BasicFieldGeneratorImpl {
    // generate_field is now part of the trait
    fn generate_field(&self, field_type: &str) -> String {
        let mut rng = rand::thread_rng();
        match field_type {
            "http://www.w3.org/2001/XMLSchema#string" => {
                // Generate a mix of plausible short texts.
                match rng.gen_range(0..4) {
                    0 => format!("{} {}", SAMPLE_FIRST_NAMES.choose(&mut rng).unwrap_or(&"Default"), SAMPLE_LAST_NAMES.choose(&mut rng).unwrap_or(&"Name")).trim().to_string(),
                    1 => format!("{} {}", SAMPLE_COURSE_ADJ.choose(&mut rng).unwrap_or(&"Basic"), SAMPLE_COURSE_NOUN.choose(&mut rng).unwrap_or(&"Subject")).trim().to_string(),
                    2 => format!("{}{} {:03}", SAMPLE_WORDS.choose(&mut rng).unwrap_or(&"Alpha"), SAMPLE_WORDS.choose(&mut rng).unwrap_or(&"Unit"), rng.gen_range(1..999)),
                    _ => format!("{} {}", SAMPLE_ORG_PREFIX.choose(&mut rng).unwrap_or(&"General"), SAMPLE_ORG_SUFFIX.choose(&mut rng).unwrap_or(&"Corp")).trim().to_string(),
                }
            }
            "http://www.w3.org/2001/XMLSchema#date" => {
                let year = rng.gen_range(1950..=2024); 
                let month = rng.gen_range(1..=12);
                let day = rng.gen_range(1..=28); // Simple way to avoid month-specific day counts
                format!("{:04}-{:02}-{:02}", year, month, day)
            }
            "http://www.w3.org/2001/XMLSchema#anyURI" => {
                format!("http://{}/{}", DOMAINS.choose(&mut rng).unwrap_or(&"example.com"), SAMPLE_WORDS.choose(&mut rng).unwrap_or(&"resource").to_lowercase())
            }
            "http://www.w3.org/2001/XMLSchema#integer" | "http://www.w3.org/2001/XMLSchema#int" => {
                format!("{}", rng.gen_range(-1000..10000))
            }
            "http://www.w3.org/2001/XMLSchema#decimal" => {
                format!("{}.{:02}", rng.gen_range(-500..500), rng.gen_range(0..99))
            }
            "http://www.w3.org/2001/XMLSchema#boolean" => {
                if rng.gen_bool(0.5) {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            _ => format!("unknown_type_value_{}", field_type.split(&['#', '/']).last().unwrap_or("generic")),
        }
    }
}

// The struct can still have its own inherent impl block for constructors or other methods
impl BasicFieldGeneratorImpl { // Updated struct name
    pub fn new() -> Self {
        BasicFieldGeneratorImpl // Updated struct name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_field_string() {
        let fg = BasicFieldGeneratorImpl::new();
        let s = fg.generate_field("http://www.w3.org/2001/XMLSchema#string");
        assert!(!s.is_empty(), "Generated string should not be empty");
        assert!(!s.starts_with("unknown_type_value_"), "Generated string should not be a fallback value");
        // Ensure no RDF quotes are part of the raw value
        assert!(!s.starts_with('"') && !s.ends_with('"'), "Generated string should be a plain value without RDF quotes");
        println!("Generated string: {}", s); 
    }

    #[test]
    fn test_generate_field_integer() {
        let fg = BasicFieldGeneratorImpl::new();
        let i_str = fg.generate_field("http://www.w3.org/2001/XMLSchema#integer");
        assert!(i_str.parse::<i64>().is_ok(), "Generated integer '{}' is not a valid integer", i_str);
        println!("Generated integer: {}", i_str);
    }

    #[test]
    fn test_generate_field_date() {
        let fg = BasicFieldGeneratorImpl::new();
        let d = fg.generate_field("http://www.w3.org/2001/XMLSchema#date");
        assert_eq!(d.len(), 10, "Generated date '{}' should be 10 characters long (YYYY-MM-DD)", d);
        assert_eq!(&d[4..5], "-", "Generated date '{}' should have '-' at index 4", d);
        assert_eq!(&d[7..8], "-", "Generated date '{}' should have '-' at index 7", d);
        assert!(d[0..4].parse::<u16>().is_ok(), "Year part of date '{}' should be a number", d);
        assert!(d[5..7].parse::<u8>().is_ok(), "Month part of date '{}' should be a number", d);
        assert!(d[8..10].parse::<u8>().is_ok(), "Day part of date '{}' should be a number", d);
        println!("Generated date: {}", d);
    }
    
    #[test]
    fn test_generate_field_boolean() {
        let fg = BasicFieldGeneratorImpl::new();
        let b = fg.generate_field("http://www.w3.org/2001/XMLSchema#boolean");
        assert!(b == "true" || b == "false", "Generated boolean '{}' is not 'true' or 'false'", b);
        println!("Generated boolean: {}", b);
    }

    #[test]
    fn test_generate_field_anyuri() { // Added test
        let fg = BasicFieldGeneratorImpl::new();
        let u = fg.generate_field("http://www.w3.org/2001/XMLSchema#anyURI");
        assert!(u.starts_with("http://") || u.starts_with("https://"), "Generated URI '{}' should start with http:// or https://", u);
        assert!(DOMAINS.iter().any(|d_iter| u.contains(d_iter)), "Generated URI '{}' should contain a known domain", u);
        println!("Generated URI: {}", u);
    }

    #[test]
    fn test_generate_field_decimal() { // Added test
        let fg = BasicFieldGeneratorImpl::new();
        let dec_str = fg.generate_field("http://www.w3.org/2001/XMLSchema#decimal");
        assert!(dec_str.parse::<f64>().is_ok(), "Generated decimal '{}' is not a valid decimal", dec_str);
        assert!(dec_str.contains('.'), "Generated decimal '{}' should contain a '.'", dec_str);
        println!("Generated decimal: {}", dec_str);
    }

    #[test]
    fn test_generate_field_unknown() {
        let fg = BasicFieldGeneratorImpl::new();
        let f = fg.generate_field("http://example.org/ontology#someUnknownField");
        assert!(f.starts_with("unknown_type_value_someUnknownField"), "Fallback for unknown type is incorrect: {}", f);
        println!("Generated unknown: {}", f);
    }
}

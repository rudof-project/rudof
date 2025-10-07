use crate::Result;
use crate::field_generators::{FieldGenerator, GenerationContext};
use rand::Rng;
use rand::seq::SliceRandom;

// XSD datatypes constants
pub const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
pub const XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
pub const XSD_DECIMAL: &str = "http://www.w3.org/2001/XMLSchema#decimal";
pub const XSD_BOOLEAN: &str = "http://www.w3.org/2001/XMLSchema#boolean";
pub const XSD_DATE: &str = "http://www.w3.org/2001/XMLSchema#date";
pub const XSD_DATETIME: &str = "http://www.w3.org/2001/XMLSchema#dateTime";
pub const XSD_ANYURI: &str = "http://www.w3.org/2001/XMLSchema#anyURI";

/// Basic string generator
pub struct StringGenerator;

impl FieldGenerator for StringGenerator {
    fn generate(&self, context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();

        // Generate based on property context
        let value = if context.property.contains("name") || context.property.contains("Name") {
            generate_name(&mut rng, &context.locale)
        } else if context.property.contains("title") || context.property.contains("Title") {
            generate_title(&mut rng, &context.locale)
        } else if context.property.contains("description")
            || context.property.contains("Description")
        {
            generate_description(&mut rng, &context.locale)
        } else if context.property.contains("email") || context.property.contains("Email") {
            generate_email(&mut rng)
        } else {
            generate_generic_string(&mut rng, &context.locale)
        };

        Ok(value)
    }

    fn name(&self) -> &str {
        "string"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec![XSD_STRING.to_string()]
    }
}

/// Basic integer generator
pub struct IntegerGenerator;

impl FieldGenerator for IntegerGenerator {
    fn generate(&self, context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();

        // Get range from parameters or use defaults
        let min = context
            .parameters
            .get("min")
            .and_then(|v| v.as_i64())
            .unwrap_or(-1000) as i32;

        let max = context
            .parameters
            .get("max")
            .and_then(|v| v.as_i64())
            .unwrap_or(10000) as i32;

        let value = rng.gen_range(min..=max);
        Ok(value.to_string())
    }

    fn name(&self) -> &str {
        "integer"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec![XSD_INTEGER.to_string()]
    }
}

/// Basic decimal generator
pub struct DecimalGenerator;

impl FieldGenerator for DecimalGenerator {
    fn generate(&self, context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();

        let min = context
            .parameters
            .get("min")
            .and_then(|v| v.as_f64())
            .unwrap_or(-500.0);

        let max = context
            .parameters
            .get("max")
            .and_then(|v| v.as_f64())
            .unwrap_or(500.0);

        let precision = context
            .parameters
            .get("precision")
            .and_then(|v| v.as_u64())
            .unwrap_or(2) as usize;

        let value = rng.gen_range(min..=max);
        Ok(format!("{value:.precision$}"))
    }

    fn name(&self) -> &str {
        "decimal"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec![XSD_DECIMAL.to_string()]
    }
}

/// Basic boolean generator
pub struct BooleanGenerator;

impl FieldGenerator for BooleanGenerator {
    fn generate(&self, context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();

        let true_probability = context
            .parameters
            .get("true_probability")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5);

        let value = rng.gen_bool(true_probability);
        Ok(value.to_string())
    }

    fn name(&self) -> &str {
        "boolean"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec![XSD_BOOLEAN.to_string()]
    }
}

/// Basic date generator
pub struct DateGenerator;

impl FieldGenerator for DateGenerator {
    fn generate(&self, context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();

        let start_year = context
            .parameters
            .get("start_year")
            .and_then(|v| v.as_u64())
            .unwrap_or(1950) as i32;

        let end_year = context
            .parameters
            .get("end_year")
            .and_then(|v| v.as_u64())
            .unwrap_or(2024) as i32;

        let year = rng.gen_range(start_year..=end_year);
        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28); // Simplified to avoid month-specific validation

        Ok(format!("{year:04}-{month:02}-{day:02}"))
    }

    fn name(&self) -> &str {
        "date"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec![XSD_DATE.to_string()]
    }
}

/// Basic dateTime generator
pub struct DateTimeGenerator;

impl FieldGenerator for DateTimeGenerator {
    fn generate(&self, context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();

        let start_year = context
            .parameters
            .get("start_year")
            .and_then(|v| v.as_u64())
            .unwrap_or(1950) as i32;

        let end_year = context
            .parameters
            .get("end_year")
            .and_then(|v| v.as_u64())
            .unwrap_or(2024) as i32;

        let year = rng.gen_range(start_year..=end_year);
        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28); // Simplified to avoid month-specific validation
        let hour = rng.gen_range(0..=23);
        let minute = rng.gen_range(0..=59);
        let second = rng.gen_range(0..=59);

        Ok(format!(
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z"
        ))
    }

    fn name(&self) -> &str {
        "datetime"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec![XSD_DATETIME.to_string()]
    }
}

/// Basic URI generator
pub struct UriGenerator;

impl FieldGenerator for UriGenerator {
    fn generate(&self, _context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();

        let domains = [
            "example.com",
            "example.org",
            "example.net",
            "company.xyz",
            "research.edu",
        ];
        let paths = ["resource", "item", "entity", "object", "data"];

        let domain = domains.choose(&mut rng).unwrap();
        let path = paths.choose(&mut rng).unwrap();
        let id = rng.gen_range(1..10000);

        Ok(format!("http://{domain}/{path}/{id}"))
    }

    fn name(&self) -> &str {
        "uri"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec![XSD_ANYURI.to_string()]
    }
}

// Helper functions for generating realistic data

fn generate_name(rng: &mut impl Rng, locale: &str) -> String {
    let first_names = match locale {
        "es" => &[
            "Ana", "Carlos", "María", "Diego", "Elena", "Fernando", "Isabel", "Jorge", "Laura",
            "Miguel",
        ],
        "fr" => &[
            "Alice",
            "Bernard",
            "Claire",
            "David",
            "Élise",
            "François",
            "Gabrielle",
            "Henri",
            "Isabelle",
            "Jacques",
        ],
        _ => &[
            "Alice", "Bob", "Charlie", "Diana", "Edward", "Fiona", "George", "Hannah", "Ian",
            "Julia",
        ],
    };

    let last_names = match locale {
        "es" => &[
            "García",
            "Rodríguez",
            "González",
            "Fernández",
            "López",
            "Martínez",
            "Sánchez",
            "Pérez",
            "Gómez",
            "Ruiz",
        ],
        "fr" => &[
            "Martin", "Bernard", "Dubois", "Thomas", "Robert", "Richard", "Petit", "Durand",
            "Leroy", "Moreau",
        ],
        _ => &[
            "Smith",
            "Johnson",
            "Williams",
            "Brown",
            "Jones",
            "Garcia",
            "Miller",
            "Davis",
            "Rodriguez",
            "Martinez",
        ],
    };

    let first = first_names.choose(rng).unwrap();
    let last = last_names.choose(rng).unwrap();

    format!("{first} {last}")
}

fn generate_title(rng: &mut impl Rng, locale: &str) -> String {
    let adjectives = match locale {
        "es" => &[
            "Moderno",
            "Avanzado",
            "Eficiente",
            "Innovador",
            "Práctico",
            "Teórico",
            "Aplicado",
        ],
        "fr" => &[
            "Moderne",
            "Avancé",
            "Efficace",
            "Innovant",
            "Pratique",
            "Théorique",
            "Appliqué",
        ],
        _ => &[
            "Modern",
            "Advanced",
            "Efficient",
            "Innovative",
            "Practical",
            "Theoretical",
            "Applied",
        ],
    };

    let nouns = match locale {
        "es" => &[
            "Sistema",
            "Proyecto",
            "Análisis",
            "Investigación",
            "Desarrollo",
            "Plataforma",
        ],
        "fr" => &[
            "Système",
            "Projet",
            "Analyse",
            "Recherche",
            "Développement",
            "Plateforme",
        ],
        _ => &[
            "System",
            "Project",
            "Analysis",
            "Research",
            "Development",
            "Platform",
        ],
    };

    let adj = adjectives.choose(rng).unwrap();
    let noun = nouns.choose(rng).unwrap();

    format!("{adj} {noun}")
}

fn generate_description(rng: &mut impl Rng, _locale: &str) -> String {
    let templates = [
        "This is a comprehensive overview of the subject matter.",
        "An innovative approach to solving complex problems.",
        "A detailed analysis of current methodologies and practices.",
        "Research findings and their practical applications.",
        "Advanced techniques for modern challenges.",
    ];

    templates.choose(rng).unwrap().to_string()
}

fn generate_email(rng: &mut impl Rng) -> String {
    let prefixes = [
        "user",
        "admin",
        "contact",
        "info",
        "support",
        "sales",
        "john.doe",
        "jane.smith",
    ];
    let domains = ["example.com", "company.org", "research.edu", "business.net"];

    let prefix = prefixes.choose(rng).unwrap();
    let domain = domains.choose(rng).unwrap();
    let number = rng.gen_range(1..1000);

    if rng.gen_bool(0.3) {
        format!("{prefix}{number}@{domain}")
    } else {
        format!("{prefix}@{domain}")
    }
}

fn generate_generic_string(rng: &mut impl Rng, _locale: &str) -> String {
    let words = [
        "Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta",
    ];
    let word = words.choose(rng).unwrap();
    let number = rng.gen_range(1..1000);

    format!("{word}{number:03}")
}

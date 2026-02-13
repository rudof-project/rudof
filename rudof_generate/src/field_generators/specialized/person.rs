use crate::Result;
use crate::field_generators::{FieldGenerator, GenerationContext};
use rand::Rng;
use rand::seq::SliceRandom;

/// Specialized generator for person-related data
pub struct PersonGenerator;

impl FieldGenerator for PersonGenerator {
    fn generate(&self, context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();

        if context.property.contains("firstName") || context.property.ends_with("fname") {
            Ok(generate_first_name(&mut rng, &context.locale))
        } else if context.property.contains("lastName") || context.property.ends_with("lname") {
            Ok(generate_last_name(&mut rng, &context.locale))
        } else if context.property.contains("fullName") || context.property.contains("name") {
            let first = generate_first_name(&mut rng, &context.locale);
            let last = generate_last_name(&mut rng, &context.locale);
            Ok(format!("{first} {last}"))
        } else if context.property.contains("email") {
            Ok(generate_person_email(&mut rng, &context.locale))
        } else if context.property.contains("phone") {
            Ok(generate_phone_number(&mut rng, &context.locale))
        } else if context.property.contains("address") {
            Ok(generate_address(&mut rng, &context.locale))
        } else {
            // Fallback to generic person name
            let first = generate_first_name(&mut rng, &context.locale);
            let last = generate_last_name(&mut rng, &context.locale);
            Ok(format!("{first} {last}"))
        }
    }

    fn name(&self) -> &str {
        "person"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec!["http://www.w3.org/2001/XMLSchema#string".to_string()]
    }
}

fn generate_first_name(rng: &mut impl Rng, locale: &str) -> String {
    let names = match locale {
        "es" => &[
            "Alejandro",
            "Ana",
            "Antonio",
            "Carmen",
            "Carlos",
            "Cristina",
            "Diego",
            "Elena",
            "Fernando",
            "Isabel",
            "Javier",
            "Laura",
            "Manuel",
            "María",
            "Miguel",
            "Pilar",
            "Rafael",
            "Rosa",
            "Sergio",
            "Teresa",
        ],
        "fr" => &[
            "Alexandre",
            "Alice",
            "Antoine",
            "Catherine",
            "Charles",
            "Christine",
            "David",
            "Élise",
            "François",
            "Isabelle",
            "Jacques",
            "Julie",
            "Louis",
            "Marie",
            "Michel",
            "Nicole",
            "Pierre",
            "Sophie",
            "Thomas",
            "Valérie",
        ],
        _ => &[
            "Alexander",
            "Alice",
            "Andrew",
            "Catherine",
            "Charles",
            "Christine",
            "David",
            "Elizabeth",
            "Frank",
            "Isabella",
            "James",
            "Julia",
            "Michael",
            "Maria",
            "Robert",
            "Nicole",
            "Peter",
            "Sophie",
            "Thomas",
            "Victoria",
        ],
    };

    names.choose(rng).unwrap().to_string()
}

fn generate_last_name(rng: &mut impl Rng, locale: &str) -> String {
    let names = match locale {
        "es" => &[
            "García",
            "González",
            "Rodríguez",
            "Fernández",
            "López",
            "Martínez",
            "Sánchez",
            "Pérez",
            "Gómez",
            "Martín",
            "Jiménez",
            "Ruiz",
            "Hernández",
            "Díaz",
            "Moreno",
            "Muñoz",
            "Álvarez",
            "Romero",
            "Alonso",
            "Gutiérrez",
        ],
        "fr" => &[
            "Martin", "Bernard", "Dubois", "Thomas", "Robert", "Richard", "Petit", "Durand", "Leroy", "Moreau",
            "Simon", "Laurent", "Lefebvre", "Michel", "Garcia", "David", "Bertrand", "Roux", "Vincent", "Fournier",
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
            "Hernandez",
            "Lopez",
            "Gonzalez",
            "Wilson",
            "Anderson",
            "Thomas",
            "Taylor",
            "Moore",
            "Jackson",
            "Martin",
        ],
    };

    names.choose(rng).unwrap().to_string()
}

fn generate_person_email(rng: &mut impl Rng, locale: &str) -> String {
    let first = generate_first_name(rng, locale).to_lowercase();
    let last = generate_last_name(rng, locale).to_lowercase();

    let domains = ["gmail.com", "yahoo.com", "hotmail.com", "outlook.com", "company.com"];
    let domain = domains.choose(rng).unwrap();

    let separators = [".", "_", ""];
    let separator = separators.choose(rng).unwrap();

    if rng.gen_bool(0.2) {
        let number = rng.gen_range(1..100);
        format!("{first}{separator}{last}{number}@{domain}")
    } else {
        format!("{first}{separator}{separator}{last}@{domain}")
    }
}

fn generate_phone_number(rng: &mut impl Rng, locale: &str) -> String {
    match locale {
        "es" => {
            // Spanish phone number format
            let prefix = rng.gen_range(600..800);
            let number = rng.gen_range(100000..1000000);
            format!("+34 {prefix} {number}")
        },
        "fr" => {
            // French phone number format
            let area = rng.gen_range(1..6);
            let number = rng.gen_range(10000000..100000000);
            format!(
                "+33 {} {} {} {}",
                area,
                number / 1000000,
                (number / 1000) % 1000,
                number % 1000
            )
        },
        _ => {
            // US phone number format
            let area = rng.gen_range(200..1000);
            let exchange = rng.gen_range(200..1000);
            let number = rng.gen_range(1000..10000);
            format!("({area}) {exchange}-{number}")
        },
    }
}

fn generate_address(rng: &mut impl Rng, locale: &str) -> String {
    match locale {
        "es" => {
            let streets = [
                "Calle Mayor",
                "Avenida de la Constitución",
                "Plaza España",
                "Calle Real",
            ];
            let street = streets.choose(rng).unwrap();
            let number = rng.gen_range(1..200);
            format!("{street} {number}")
        },
        "fr" => {
            let streets = [
                "Rue de la Paix",
                "Avenue des Champs",
                "Boulevard Saint-Germain",
                "Place de la République",
            ];
            let street = streets.choose(rng).unwrap();
            let number = rng.gen_range(1..200);
            format!("{number} {street}")
        },
        _ => {
            let streets = ["Main Street", "Oak Avenue", "Pine Road", "Elm Street", "First Avenue"];
            let street = streets.choose(rng).unwrap();
            let number = rng.gen_range(100..9999);
            format!("{number} {street}")
        },
    }
}

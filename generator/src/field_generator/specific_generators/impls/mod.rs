use oxiri::IriRef;
use srdf::literal::Literal;
use srdf::literal::Literal::BooleanLiteral;
use srdf::literal::Literal::NumericLiteral;
use srdf::literal::Literal::DatatypeLiteral;
use srdf::numeric_literal::NumericLiteral::Double;
use srdf::numeric_literal::NumericLiteral::Integer;
use srdf::numeric_literal::NumericLiteral::Decimal;
use rust_decimal::prelude::*;

use prefixmap:: IriRef;

use rand::Rng;


use super::RandomField;  

impl RandomField for Literal {
    fn generate_random(&self) -> Self {
        match self  {
            Self::BooleanLiteral(_) => generate_random_boolean_literal(),
            Self::NumericLiteral(_) => generate_random_numeric_literal(),
            Self::DatatypeLiteral {..} => generate_random_datatype_literal(),
            _ => unimplemented!()
        }
    }
}

// ------------------------------------------- GENERATE RANDOM DATATYPE LITERAL --------------------------------------------

fn generate_random_datatype_literal() -> Literal {
    // Generate a random value (this is a simple example, you could make it more complex)
    let lexical_form = generate_random_value(); // Function that generates a random value as a string
    
    // Generate a random datatype (e.g., "xsd:string", "xsd:int", etc.)
    let datatype = generate_random_datatype(); // Function that generates a random datatype as a string

    DatatypeLiteral{lexical_form, datatype}
}


fn generate_random_value() -> String {
    // Generate a random string value (for simplicity, just a random number as string)
    let random_value = rand::random::<i64>(); 
    let mut result= String::from("value");
    result.push_str(&random_value.to_string());
    result
}

fn generate_random_iri_ref() -> IriRef {
    // Generate a random string that serves as the path for the IRI
    let random_suffix: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)  // Create a 10-character random string
        .map(char::from)
        .collect();

    // Define a base IRI prefix (you can customize this)
    let base_iri = "http://example.org/";

    // Combine the base IRI and random suffix to form the full IRI
    let full_iri = format!("{}{}", base_iri, random_suffix);

    // Create a new IriRef from the generated string
    IriRef::new(&full_iri).unwrap()  // Using unwrap() here for simplicity
}


// ------------------------------------------- GENERATE RANDOM BOOLEAN LITERAL --------------------------------------------
fn generate_random_boolean_literal() -> Literal {
    BooleanLiteral(rand::random::<bool>())
}


// ------------------------------------------- GENERATE RANDOM NUMERIC LITERAL --------------------------------------------

fn generate_random_numeric_literal() -> Literal {
    
    let mut rng = rand::thread_rng();

    let choice = rng.gen_range(0..3);

    match choice {
        0 => generate_random_numeric_literal_double(), 
        1 => generate_random_numeric_literal_integer(),  
        2 => generate_random_numeric_literal_decimal(), 
        _ =>unreachable!()  
    }

}

fn generate_random_numeric_literal_double() -> Literal {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..100.0); // Random f64 between 0 and 100
    NumericLiteral(Double(random_number))
}

fn generate_random_numeric_literal_integer() -> Literal {
    let mut rng = rand::thread_rng();
    let random_number: isize = rng.gen_range(0..100); // Random integer between 0 and 99
    NumericLiteral(Integer(random_number))
}

fn generate_random_numeric_literal_decimal() -> Literal {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen_range(0.0..100.0); // Random f64 between 0.0 and 100.0
    let decimal_value = rust_decimal::Decimal::from_f64(random_number).unwrap(); // Convert f64 to Decimal
    Literal::NumericLiteral(Decimal(decimal_value))
}


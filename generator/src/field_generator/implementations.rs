
use srdf::lang::Lang;
use srdf::literal::Literal;
use srdf::literal::Literal::BooleanLiteral;
use srdf::literal::Literal::NumericLiteral;
use srdf::literal::Literal::DatatypeLiteral;
use srdf::literal::Literal::StringLiteral;
use srdf::numeric_literal::NumericLiteral::Double;
use srdf::numeric_literal::NumericLiteral::Integer;
use srdf::numeric_literal::NumericLiteral::Decimal;
use rust_decimal::prelude::*;

use prefixmap:: IriRef;

use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::thread_rng;


use super::RandomField;  

impl RandomField for Literal {
    fn generate_random(&self) -> Self {
        match self  {
            BooleanLiteral(_) => generate_random_boolean_literal(),
            NumericLiteral(_) => generate_random_numeric_literal(),
            DatatypeLiteral {..} => generate_random_datatype_literal(),
            StringLiteral {.. } => generate_random_string_literal () ,
            
        }
    }
}




fn generate_random_string_literal () -> Literal{

    let mut rng = rand::thread_rng();

    // Generate a random string for lexical_form
    let lexical_form_length = rng.gen_range(5..20); // random length between 5 and 20
    let lexical_form: String = (0..lexical_form_length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();

    // Optionally generate a language
    let lang = if rng.gen_bool(0.5) {
        Some(Lang::new("en")) // Generate a random Lang
    } else {
        None // or no language
    };
    
    StringLiteral {
        lexical_form,
        lang,
    }
}



// ------------------------------------------- GENERATE RANDOM DATATYPE LITERAL --------------------------------------------

fn generate_random_datatype_literal() -> Literal {
    let lexical_form = generate_random_value_string(); 
    let datatype = generate_random_datatype_iriref(); 
    DatatypeLiteral{lexical_form, datatype}
}


fn generate_random_value_string() -> String {
    // Generate a random string value (for simplicity, just a random number as string)
    let random_value = rand::random::<i64>(); 
    let mut result= String::from("value");
    result.push_str(&random_value.to_string());
    result
}

fn generate_random_datatype_iriref() -> IriRef {
    // Generate a random string that serves as the path for the IRI
    let random_suffix: String =  thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)  // Create a 10-character random string
        .map(char::from)
        .collect();

   
    let base_iri = "http://example.org/";


    IriRef::prefixed(base_iri, random_suffix.as_str())
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


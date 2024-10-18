use srdf::literal::Literal;
use srdf::literal::Literal::BooleanLiteral;
use srdf::literal::Literal::NumericLiteral;
use srdf::numeric_literal::NumericLiteral::Double;
use srdf::numeric_literal::NumericLiteral::Integer;
use srdf::numeric_literal::NumericLiteral::Decimal;
use rust_decimal::prelude::*;

use rand::Rng;


use super::RandomField;  

impl RandomField for Literal {
    fn generate_random(&self) -> Self {
        match self  {
            Self::BooleanLiteral(_) => generate_random_boolean_literal(),
            Self::NumericLiteral(_) => generate_random_numeric_literal(),
            _ => unimplemented!()
        }
    }
}

fn generate_random_boolean_literal() -> Literal {
    BooleanLiteral(rand::random::<bool>())
}

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


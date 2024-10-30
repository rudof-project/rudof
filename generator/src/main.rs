
use generator::field_generator::RandomLiteral;
use srdf::literal::Literal;

fn main() {
    // Create an example Literal
    let example_literal = Literal::StringLiteral {
        lexical_form: String::new(),
        lang: None,
    };

    // Generate a random literal and print it
    let random_literal = example_literal.generate_random();
    println!("Random Literal: {:?}", random_literal);
}

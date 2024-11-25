
use generator::{field_generator::RandomLiteral, traverse::implementations::ShexVisitor};
use iri_s::IriS;
use shex_ast::{Schema, Shape, ShapeExpr, ShapeExprLabel, TripleExpr};
use shex_compact::ShExParser;
use srdf::literal::Literal;
use srdf::{SRDFBuilder, SRDFGraph};
use std::io;
use serde_json;
use generator::utils::file_reader::read_and_parse_schema;
use generator::traverse::Traversable;

fn main1() {
    // Create an example Literal
    let example_literal = Literal::StringLiteral {
        lexical_form: String::new(),
        lang: None,
    };

    // Generate a random literal and print it
    let random_literal = example_literal.generate_random();
    println!("Random Literal: {:?}", random_literal);
}

fn main2(){
    let str = r#"{
        "type": "Shape",
        "expression": {
          "type": "TripleConstraint",
          "predicate": "http://a.example/p1"
        }
      }"#;
    let se = serde_json::from_str::<ShapeExpr>(str).unwrap();
    let expected = ShapeExpr::Shape(Shape::default().with_expression(
        TripleExpr::TripleConstraint {
            id: None,
            negated: None,
            inverse: None,
            predicate: IriS::new_unchecked("http://a.example/p1").into(),
            value_expr: None,
            min: None,
            max: None,
            sem_acts: None,
            annotations: None,
        },
    ));
    println!("Test:{:?}", se)
}

fn main3(){
    let str = r#"prefix : <http://example.org/>
    prefix xsd: <http://www.w3.org/2001/XMLSchema#>
    
    :Person { :name       xsd:string  ;
              :birthdate  xsd:date  ? ;
              :enrolledIn @:Course *
    }
    :Course { :name xsd:string }"#;

    let schema = ShExParser::parse(str, None).unwrap();
    let mut expected = Schema::new();
    expected.add_prefix("", &IriS::new_unchecked("http://example.org/"));
    expected.add_shape(
        ShapeExprLabel::iri_unchecked("http://example.org/S"),
        ShapeExpr::empty_shape(),
        false
    );



    assert_eq!(schema, expected);
}

fn main() -> io::Result<()> {
    // Read and parse the schema file
    let file_path = "examples/schema.shex";
    let schema = read_and_parse_schema(file_path)?;
    // Create a visitor and traverse the schema 
    let mut visitor = ShexVisitor::new(SRDFGraph::empty());  
    schema.accept(&mut visitor); 
       
    print!("{:?}", visitor.rdf);
    


    Ok(())
}
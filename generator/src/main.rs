
use generator::{field_generator::RandomLiteral, traverse::implementations::ShexVisitor};
use iri_s::IriS;
use shex_ast::{Schema, Shape, ShapeExpr, ShapeExprLabel, TripleExpr};
use shex_compact::ShExParser;
use srdf::literal::Literal;
use srdf::{srdf_graph, SRDFBuilder, SRDFGraph, Subject};
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

fn main() -> io::Result<()> {
    // Read and parse the schema file
    let file_path = "examples/schema.shex";
    let schema = read_and_parse_schema(file_path)?;
    // Create a visitor and traverse the schema 
    let mut visitor = ShexVisitor::new();  
    schema.accept(&mut visitor); 
       

    


    let shapes = &visitor.shapes;

    let mut srdf_graph = SRDFGraph::new();

    srdf_graph.add_prefix_map(visitor.prefixmap).unwrap();

    if let Some(first_shape) = shapes.first() {
        println!("First Shape: {:?}", first_shape);
       


        
        let subject: Subject = srdf_graph

        // Add a triple with the subject, predicate, and object to the graph
        //srdf_graph.add_triple(subj, pred, obj)?;
        
    } else {
        println!("No shapes found");
    }


    Ok(())
}
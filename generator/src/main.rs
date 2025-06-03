use std::env;
use std::process;
use generator::generator::{Generator, FieldGenerator, GraphGenerator};
use generator::generator::graph_generator::BasicGraphGenerator;


struct DummyFieldGenerator;
impl FieldGenerator for DummyFieldGenerator {
    fn generate_value(&self, _predicate: &str, _datatype: Option<&str>) -> String {
        "\"dummyValue\"".to_string()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <path-to-shex-file> <number-of-entities>", args[0]);
        process::exit(1);
    }
    let shex_path = &args[1];
    let num_entities: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: <number-of-entities> must be an integer");
            process::exit(1);
        }
    };

    let graph_generator = Box::new(BasicGraphGenerator::new());
    let field_generator = DummyFieldGenerator;
    let mut generator = Generator::new(graph_generator, &field_generator);
    generator.load(shex_path);
    generator.generate(num_entities)
        .unwrap_or_else(|e| {
            eprintln!("Error generating entities: {e}");
            process::exit(1);
        });
   
}


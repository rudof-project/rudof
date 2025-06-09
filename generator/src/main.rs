use std::env;
use std::process;
use std::time::Instant; // Added for timing
use generator::generator::{Generator};
use generator::generator::graph_generator::BasicGraphGeneratorImpl;
use generator::generator::field_generator::BasicFieldGeneratorImpl;
use srdf::SRDFBuilder;
use srdf::RDFFormat;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <path-to-shex-file> <number-of-entities> <output-file-path>", args[0]);
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
    let output_path = &args[3];

    // Instantiate the concrete BasicFieldGeneratorImpl
    let field_generator = Box::new(BasicFieldGeneratorImpl::new());
    // Pass the field_generator to BasicGraphGeneratorImpl::new
    let graph_generator = Box::new(BasicGraphGeneratorImpl::new(field_generator));
    let mut generator = Generator::new(graph_generator);
    
    let start_time = Instant::now(); // Start timing

    generator.load(shex_path);
    generator.generate(num_entities)
        .unwrap_or_else(|e| {
            eprintln!("Error generating entities: {e}");
            process::exit(1);
        });
    
    let duration = start_time.elapsed(); // Calculate duration

    // Save the generated SRDFGraph in Turtle format
    let graph = generator.get_graph();
    let triple_count = graph.len();
    let mut out = std::fs::File::create(output_path).expect("Could not create output file");
    graph.serialize(&RDFFormat::Turtle, &mut out).unwrap();
    println!("Graph with {num_entities} entities and {triple_count} triples was generated in {:?} and saved to {output_path}.", duration); // Updated message
}


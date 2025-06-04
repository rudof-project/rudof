use shex_ast::ast::ShapeDecl;

pub mod graph_generator;
pub mod dependency_graph;




pub struct Generator<'a> {
    pub graph_generator: Box<dyn GraphGenerator>,
    pub field_generator: &'a dyn FieldGenerator,
}

impl<'a> Generator<'a> {
    pub fn new(graph_generator: Box<dyn GraphGenerator>, field_generator: &'a dyn FieldGenerator) -> Self {
        Self { graph_generator, field_generator }
    }
    pub fn load(&mut self, shex_path: &str) {
        let shapes = crate::utils::extract_shapes_from_shex_file(shex_path)
            .unwrap_or_else(|e| {
                eprintln!("Error loading shapes: {e}");
                Vec::new()
            });
        self.graph_generator.set_shapes(shapes);
    }
    pub fn generate(&mut self, num_entities: usize) -> Result<(), String> {
        self.graph_generator.generate(num_entities)
    }

    pub fn get_graph(&self) -> &srdf::srdf_graph::SRDFGraph {
        self.graph_generator.get_graph()
    }
}


pub trait GraphGenerator: Send + Sync {
    fn set_shapes(&mut self, shapes: Vec<ShapeDecl>);
    fn generate(&mut self, num_entities: usize) -> Result<(), String>;
    fn get_graph(&self) -> &srdf::srdf_graph::SRDFGraph;
}


pub trait FieldGenerator: Send + Sync {
    fn generate_value(&self, predicate: &str, datatype: Option<&str>) -> String;
}


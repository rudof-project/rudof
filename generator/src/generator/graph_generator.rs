
use shex_ast::ast::ShapeDecl;
use crate::generator::GraphGenerator;
use crate::generator::dependency_graph::DependencyGraph;

pub struct BasicGraphGenerator {
    pub dep_graph: DependencyGraph,
}


impl BasicGraphGenerator {
    pub fn new() -> Self {
        BasicGraphGenerator { dep_graph: DependencyGraph::new() }
    }
    pub fn set_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        self.dep_graph.load_shapes(shapes);
    }

    /// Generate triples for a given shape (for a single entity)
    pub fn generate_triples_for_shape(&self, shape: &ShapeDecl) {
        println!("Shape: {}", shape.id);
        let node = format!("<{}>", shape.id);
        // Print a type triple
        println!("{} a <{}> .", node, shape.id);
        if let shex_ast::ast::ShapeExpr::Shape(s) = &shape.shape_expr {
            if let Some(expr) = &s.expression {
                use shex_ast::ast::TripleExpr;
                fn print_triples(expr: &shex_ast::ast::TripleExpr, node: &str) {
                    match expr {
                        TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
                            for e in expressions {
                                print_triples(&e.te, node);
                            }
                        }
                        TripleExpr::TripleConstraint { predicate, .. } => {
                            println!("{} {} \"dummyValue\" .", node, predicate);
                        }
                        TripleExpr::TripleExprRef(label) => {
                            println!("# TripleExprRef: {}", label);
                        }
                    }
                }
                print_triples(&expr.te, &node);
            }
        }
    }
}


impl GraphGenerator for BasicGraphGenerator {
    fn set_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        self.set_shapes(shapes);
    }
    fn generate(&self, num_entities: usize) -> Result<(), String> {
        println!("Loaded shapes:");
        for shape in self.dep_graph.shapes.values() {
            for _i in 1..=num_entities {
                self.generate_triples_for_shape(shape);
            }
        }
        Ok(())
    }
}

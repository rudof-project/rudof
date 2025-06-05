use shex_ast::ast::{ShapeDecl, ShapeExpr, TripleExpr};
use crate::generator::GraphGenerator;
use crate::generator::dependency_graph::DependencyGraph;
use srdf::srdf_graph::SRDFGraph;
use srdf::SRDFBuilder;

pub struct BasicGraphGenerator {
    pub dep_graph: DependencyGraph,
    pub graph: SRDFGraph,
}


impl BasicGraphGenerator {
    pub fn new() -> Self {
        BasicGraphGenerator {
            dep_graph: DependencyGraph::new(),
            graph: SRDFGraph::default(),
        }
    }
    pub fn set_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        self.dep_graph.load_shapes(shapes);
    }

    /// Generate triples for a given shape (for a single entity)
    pub fn generate_triples_for_shape(&self, shape: &ShapeDecl, entity_index: usize) -> Vec<oxrdf::Triple> {
        use oxrdf::{NamedNode, Subject, Term, Triple};
        let mut triples = Vec::new();
        // Generate a unique node IRI for each entity
        let node_iri = format!("{}-{}", shape.id.to_string(), entity_index);
        let node = NamedNode::new_unchecked(&node_iri);
        // Type triple
        triples.push(Triple::new(
            Subject::NamedNode(node.clone()),
            NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
            Term::NamedNode(NamedNode::new_unchecked(&shape.id.to_string())),
        ));
        if let ShapeExpr::Shape(s) = &shape.shape_expr {
            if let Some(expr) = &s.expression {
                fn collect_triples(expr: &TripleExpr, node: &NamedNode, triples: &mut Vec<oxrdf::Triple>) {
                    match expr {
                        TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
                            for e in expressions {
                                collect_triples(&e.te, node, triples);
                            }
                        }
                        TripleExpr::TripleConstraint { predicate, value_expr, .. } => {
                            // Skip if value_expr is a reference to another shape
                            let is_shape_ref = match value_expr {
                                Some(sexpr) => matches!(**sexpr, shex_ast::ast::ShapeExpr::Ref(_)),
                                None => false,
                            };
                            if is_shape_ref {
                                // Do not generate a triple for shape references
                                return;
                            }
                            let pred = NamedNode::new_unchecked(&predicate.to_string());
                            triples.push(Triple::new(
                                Subject::NamedNode(node.clone()),
                                pred,
                                Term::Literal(oxrdf::Literal::from("dummyValue")),
                            ));
                        }
                        TripleExpr::TripleExprRef(_label) => {
                            // Reference, skip or handle as needed
                        }
                    }
                }
                collect_triples(&expr.te, &node, &mut triples);
            }
        }
        triples
    }
}


impl GraphGenerator for BasicGraphGenerator {
    fn set_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        self.set_shapes(shapes);
    }
    fn generate(&mut self, num_entities: usize) -> Result<(), String> {

        //Generating triples for entities based on the shapes in the dependency graph

        let num_shapes = self.dep_graph.shapes.len();
        if num_shapes == 0 {
            return Ok(());
        }
        let base_entities_per_shape = num_entities / num_shapes;
        let mut remainder = num_entities % num_shapes;
        for shape in self.dep_graph.shapes.values() {
            // Distribute the remainder: some shapes get one extra entity
            let mut entities_for_this_shape = base_entities_per_shape;
            if remainder > 0 {
                entities_for_this_shape += 1;
                remainder -= 1;
            }
            for i in 1..=entities_for_this_shape {
                let triples = self.generate_triples_for_shape(shape, i);
                for triple in triples {
                    // Add each triple to the SRDFGraph stored in self
                    self.graph.add_triple(
                        triple.subject.clone(),
                        triple.predicate.clone(),
                        triple.object.clone(),
                    ).map_err(|e| format!("Failed to add triple: {e}"))?;
                }
            }
        }

        //Generating triples for relations between entities

        Ok(())
    }
    fn get_graph(&self) -> &SRDFGraph {
        &self.graph
    }
}

use shex_ast::ast::{ShapeDecl, ShapeExpr, TripleExpr};
use crate::generator::GraphGenerator;
use crate::generator::dependency_graph::DependencyGraph;
use srdf::srdf_graph::SRDFGraph;
use srdf::SRDFBuilder;

pub struct BasicGraphGenerator {
    pub dep_graph: DependencyGraph,
    pub graph: SRDFGraph,
    pub generated_entities: std::collections::HashMap<String, Vec<String>>,
}


impl BasicGraphGenerator {
    pub fn new() -> Self {
        BasicGraphGenerator {
            dep_graph: DependencyGraph::new(),
            graph: SRDFGraph::default(),
            generated_entities: std::collections::HashMap::new(),
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
                                // Do not generate a triple for shape references because they are handled separately
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

    /// Generate all entities for each shape and store their IRIs
    pub fn generate_entities(&mut self, num_entities: usize) -> Result<(), String> {
        let num_shapes = self.dep_graph.shapes.len();
        if num_shapes == 0 {
            return Ok(());
        }
        let shape_labels: Vec<_> = self.dep_graph.shapes.keys().cloned().collect();
        let base_entities_per_shape = num_entities / num_shapes;
        let mut remainder = num_entities % num_shapes;
        self.generated_entities = std::collections::HashMap::new();
        for label in shape_labels {
            let shape = self.dep_graph.shapes.get(&label).unwrap();
            let mut entities_for_this_shape = base_entities_per_shape;
            if remainder > 0 {
                entities_for_this_shape += 1;
                remainder -= 1;
            }
            let mut entity_iris = Vec::new();
            for i in 1..=entities_for_this_shape {
                let node_iri = format!("{}-{}", shape.id.to_string(), i);
                entity_iris.push(node_iri.clone());
                let triples = self.generate_triples_for_shape(shape, i);
                for triple in triples {
                    self.graph.add_triple(
                        triple.subject.clone(),
                        triple.predicate.clone(),
                        triple.object.clone(),
                    ).map_err(|e| format!("Failed to add triple: {e}"))?;
                }
            }
            self.generated_entities.insert(shape.id.to_string(), entity_iris);
        }
        Ok(())
    }

    /// Generate triples for relations between entities using cardinalities
    pub fn generate_relations(&mut self) -> Result<(), String> {
        use oxrdf::{NamedNode, Subject, Term, Triple};
        for (shape_label, deps) in &self.dep_graph.dependencies {
            let from_entities = self.generated_entities.get(shape_label).unwrap();
            for (target_shape, property, min, max) in deps {
                let to_entities = self.generated_entities.get(target_shape).unwrap();
                for (idx, from_iri) in from_entities.iter().enumerate() {
                    let min = min.unwrap_or(1).max(0);
                    let max = match max {
                        Some(-1) => to_entities.len() as i32, // unbounded
                        Some(m) => *m,
                        None => 1,
                    };
                    let max = max.max(min);
                    let num_links = if min == max {
                        min
                    } else {
                        min + ((idx as i32) % (max - min + 1))
                    };
                    let mut chosen = Vec::new();
                    for offset in 0..num_links {
                        let target_idx = (idx + offset as usize) % to_entities.len();
                        chosen.push(&to_entities[target_idx]);
                    }
                    for to_iri in chosen {
                        let triple = Triple::new(
                            Subject::NamedNode(NamedNode::new_unchecked(from_iri)),
                            NamedNode::new_unchecked(property),
                            Term::NamedNode(NamedNode::new_unchecked(to_iri)),
                        );
                        self.graph.add_triple(
                            triple.subject.clone(),
                            triple.predicate.clone(),
                            triple.object.clone(),
                        ).map_err(|e| format!("Failed to add relation triple: {e}"))?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn generate_entities_and_relations(&mut self, num_entities: usize) -> Result<(), String> {
        self.generate_entities(num_entities)?;
        self.generate_relations()
    }
}


impl GraphGenerator for BasicGraphGenerator {
    fn set_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        self.set_shapes(shapes);
    }
    fn generate(&mut self, num_entities: usize) -> Result<(), String> {
        self.generate_entities_and_relations(num_entities)
    }
    fn get_graph(&self) -> &SRDFGraph {
        &self.graph
    }
}

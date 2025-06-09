use shex_ast::ast::{ShapeDecl, ShapeExpr, TripleExpr};
use crate::generator::GraphGenerator;
use crate::generator::dependency_graph::DependencyGraph;
use srdf::srdf_graph::SRDFGraph;
use srdf::SRDFBuilder;
use crate::generator::FieldGeneratorTrait; 
use oxrdf::{NamedNode, Subject, Term, Triple};

pub struct BasicGraphGeneratorImpl { // Renamed from BasicGraphGenerator
    pub dep_graph: DependencyGraph,
    pub graph: SRDFGraph,
    pub generated_entities: std::collections::HashMap<String, Vec<String>>,
    pub field_generator: Box<dyn FieldGeneratorTrait>,
}


impl BasicGraphGeneratorImpl { // Updated struct name
    pub fn new(field_generator: Box<dyn FieldGeneratorTrait>) -> Self {
        BasicGraphGeneratorImpl { // Updated struct name
            dep_graph: DependencyGraph::new(),
            graph: SRDFGraph::default(),
            generated_entities: std::collections::HashMap::new(),
            field_generator,
        }
    }
    pub fn set_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        self.dep_graph.load_shapes(shapes);
    }

    /// Generate triples for a given shape (for a single entity)
    // This method would now potentially use self.field_generator to create literal values
    pub fn generate_triples_for_shape(&self, shape: &ShapeDecl, entity_index: usize) -> Vec<oxrdf::Triple> {
        let mut triples = Vec::new();
        let node_iri = format!("{}-{}", shape.id.to_string(), entity_index);
        let node = NamedNode::new_unchecked(&node_iri);
        triples.push(Triple::new(
            Subject::NamedNode(node.clone()),
            NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
            Term::NamedNode(NamedNode::new_unchecked(&shape.id.to_string())),
        ));
        if let ShapeExpr::Shape(s) = &shape.shape_expr {
            if let Some(expr) = &s.expression {
                self.collect_triples_recursive(&expr.te, &node, &mut triples);
            }
        }
        triples
    }

    // Helper function to recursively collect triples, now a method to access field_generator
    fn collect_triples_recursive(&self, expr: &TripleExpr, node: &NamedNode, triples: &mut Vec<oxrdf::Triple>) {
        match expr {
            TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
                for e in expressions {
                    self.collect_triples_recursive(&e.te, node, triples);
                }
            }
            TripleExpr::TripleConstraint { predicate, value_expr, .. } => {
                let is_shape_ref = match value_expr {
                    Some(sexpr) => matches!(**sexpr, shex_ast::ast::ShapeExpr::Ref(_)),
                    None => false,
                };
                if is_shape_ref {
                    return;
                }
                let pred = NamedNode::new_unchecked(&predicate.to_string());
                
                // Placeholder for datatype extraction due to private field issue
                // This needs to be resolved by finding a public API in shex_ast or adjusting FieldGenerator
                let field_type_placeholder = "http://www.w3.org/2001/XMLSchema#string"; 
                // Call the method on the stored field_generator instance
                let literal_value = self.field_generator.generate_field(field_type_placeholder);
                
                let literal_term = Term::Literal(oxrdf::Literal::new_simple_literal(literal_value));

                triples.push(Triple::new(
                    Subject::NamedNode(node.clone()),
                    pred,
                    literal_term,
                ));
            }
            TripleExpr::TripleExprRef(_label) => {
                // Reference, skip or handle as needed
            }
        }
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
            // deps is &Vec<(String, String, Option<i32>, Option<i32>)>
            // so min_card_opt and max_card_opt are &Option<i32>
            for (target_shape, property, min_card_opt, max_card_opt) in deps {
                let to_entities = self.generated_entities.get(target_shape).unwrap();
                
                for (idx, from_iri) in from_entities.iter().enumerate() {
                    let num_links: i32;
                    if to_entities.is_empty() {
                        num_links = 0;
                    } else {
                        // Dereference min_card_opt to get Option<i32>, then unwrap or default.
                        let current_min = (*min_card_opt).unwrap_or(1).max(0);

                        // Dereference max_card_opt to get Option<i32> for the match.
                        let mut current_max = match *max_card_opt {
                            Some(-1) => { // Unbounded cardinality ('*' or '+')
                                let cap = 20;
                                cap.min(to_entities.len() as i32)
                            }
                            Some(m_val) => { // m_val is i32 here
                                m_val.min(to_entities.len() as i32)
                            }
                            None => { // Default cardinality (1,1)
                                1.min(to_entities.len() as i32)
                            }
                        };

                        current_max = current_max.max(current_min);

                        if current_min == current_max {
                            num_links = current_min;
                        } else {
                            num_links = current_min + ((idx as i32) % (current_max - current_min + 1));
                        }
                    }
                    
                    let mut chosen = Vec::new();
                    if !to_entities.is_empty() { // Ensure to_entities is not empty before trying to pick from it
                        for offset in 0..num_links {
                            let target_idx = (idx + offset as usize) % to_entities.len();
                            chosen.push(&to_entities[target_idx]);
                        }
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


impl GraphGenerator for BasicGraphGeneratorImpl { // Updated struct name
    fn set_shapes(&mut self, shapes: Vec<ShapeDecl>) {
        self.dep_graph.load_shapes(shapes);
    }
    fn generate(&mut self, num_entities: usize) -> Result<(), String> {
        self.generate_entities_and_relations(num_entities)
    }
    fn get_graph(&self) -> &SRDFGraph {
        &self.graph
    }
}

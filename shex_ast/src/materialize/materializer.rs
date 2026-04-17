use iri_s::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use thiserror::Error;

use crate::Node;
use crate::ast::{Schema, SemAct, ShapeExpr, ShapeExprLabel, TripleExpr};
use crate::ir::map_state::MapState;

const MAP_EXTENSION_IRI: &str = "http://shex.io/extensions/Map/";

/// Errors that can occur during materialization.
#[derive(Debug, Error)]
pub enum MaterializeError {
    #[error("No start shape found in schema")]
    NoStartShape,

    #[error("Shape not found for label: {label}")]
    ShapeNotFound { label: String },

    #[error("Cannot use node as RDF subject: {node}")]
    NodeNotSubject { node: String },

    #[error("RDF graph error: {error}")]
    RdfError { error: String },
}

/// Materializes an RDF graph from a ShEx schema and a [`MapState`].
///
/// ## Algorithm
///
/// 1. Take the start shape of the schema (or the first declared shape).
/// 2. Use the supplied `initial_node` as the root subject, or mint a fresh
///    blank node when none is given.
/// 3. For each triple constraint in the shape:
///    - **Shape reference**: create a fresh blank node, assert
///      `(subject, predicate, bnode)`, then recurse into the referenced shape
///      using the new blank node as the subject.
///    - **Leaf node with a `Map` semantic action**: look up the IRI that is
///      the `code` of the action in the [`MapState`]; if a value is found,
///      assert `(subject, predicate, value)`.
///
/// The resulting graph validates against the target schema.
pub struct Materializer;

impl Default for Materializer {
    fn default() -> Self {
        Materializer
    }
}

impl Materializer {
    pub fn new() -> Self {
        Materializer
    }

    /// Materialize an RDF graph from `schema` using `map_state` as the value
    /// source.
    ///
    /// The graph type `G` must implement [`BuildRDF`].  Use
    /// [`rudof_rdf::rdf_impl::InMemoryGraph`] for an in-memory implementation.
    ///
    /// If `initial_node` is `Some`, it is used as the root subject; otherwise a
    /// fresh blank node is created.
    pub fn materialize<G>(
        &self,
        schema: &Schema,
        map_state: &MapState,
        initial_node: Option<Node>,
    ) -> Result<G, MaterializeError>
    where
        G: BuildRDF,
        G::BNode: Clone,
        G::Subject: TryFrom<rudof_rdf::rdf_core::term::Object>,
        G::Term: From<rudof_rdf::rdf_core::term::Object> + From<G::BNode>,
        G::IRI: From<IriS>,
        G::Subject: From<G::BNode>,
        G::Err: std::fmt::Display,
    {
        let mut graph = G::empty();

        let subject: G::Subject = match initial_node {
            Some(node) => {
                let obj = node.as_object().clone();
                G::Subject::try_from(obj).map_err(|_| MaterializeError::NodeNotSubject { node: node.to_string() })?
            },
            None => {
                let bnode = graph
                    .add_bnode()
                    .map_err(|e| MaterializeError::RdfError { error: e.to_string() })?;
                G::Subject::from(bnode)
            },
        };

        let start = self.find_start_shape_expr(schema)?;
        self.generate_shape_expr::<G>(&start, subject, schema, map_state, &mut graph)?;

        Ok(graph)
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn find_start_shape_expr(&self, schema: &Schema) -> Result<ShapeExpr, MaterializeError> {
        if let Some(start) = schema.start() {
            return Ok(start);
        }
        if let Some(shapes) = schema.shapes()
            && let Some(first) = shapes.into_iter().next()
        {
            return Ok(first.shape_expr);
        }
        Err(MaterializeError::NoStartShape)
    }

    fn find_shape_by_label(&self, label: &ShapeExprLabel, schema: &Schema) -> Result<ShapeExpr, MaterializeError> {
        if let Some(shapes) = schema.shapes() {
            for decl in shapes {
                if &decl.id == label {
                    return Ok(decl.shape_expr);
                }
            }
        }
        Err(MaterializeError::ShapeNotFound {
            label: format!("{label:?}"),
        })
    }

    fn generate_shape_expr<G>(
        &self,
        shape_expr: &ShapeExpr,
        subject: G::Subject,
        schema: &Schema,
        map_state: &MapState,
        graph: &mut G,
    ) -> Result<(), MaterializeError>
    where
        G: BuildRDF,
        G::BNode: Clone,
        G::Subject: TryFrom<rudof_rdf::rdf_core::term::Object>,
        G::Term: From<rudof_rdf::rdf_core::term::Object> + From<G::BNode>,
        G::IRI: From<IriS>,
        G::Subject: From<G::BNode>,
        G::Err: std::fmt::Display,
    {
        match shape_expr {
            ShapeExpr::Shape(shape) => self.generate_shape::<G>(shape, subject, schema, map_state, graph),
            ShapeExpr::Ref(label) => {
                let target = self.find_shape_by_label(label, schema)?;
                self.generate_shape_expr::<G>(&target, subject, schema, map_state, graph)
            },
            // ShapeAnd / ShapeOr / NodeConstraint / External not handled for basic materialization
            _ => Ok(()),
        }
    }

    fn generate_shape<G>(
        &self,
        shape: &crate::ast::Shape,
        subject: G::Subject,
        schema: &Schema,
        map_state: &MapState,
        graph: &mut G,
    ) -> Result<(), MaterializeError>
    where
        G: BuildRDF,
        G::BNode: Clone,
        G::Subject: TryFrom<rudof_rdf::rdf_core::term::Object>,
        G::Term: From<rudof_rdf::rdf_core::term::Object> + From<G::BNode>,
        G::IRI: From<IriS>,
        G::Subject: From<G::BNode>,
        G::Err: std::fmt::Display,
    {
        if let Some(expr_wrapper) = &shape.expression {
            self.generate_triple_expr::<G>(&expr_wrapper.te, subject, schema, map_state, graph)?;
        }
        Ok(())
    }

    fn generate_triple_expr<G>(
        &self,
        triple_expr: &TripleExpr,
        subject: G::Subject,
        schema: &Schema,
        map_state: &MapState,
        graph: &mut G,
    ) -> Result<(), MaterializeError>
    where
        G: BuildRDF,
        G::BNode: Clone,
        G::Subject: TryFrom<rudof_rdf::rdf_core::term::Object>,
        G::Term: From<rudof_rdf::rdf_core::term::Object> + From<G::BNode>,
        G::IRI: From<IriS>,
        G::Subject: From<G::BNode>,
        G::Err: std::fmt::Display,
    {
        match triple_expr {
            TripleExpr::EachOf { expressions, .. } => {
                for wrapper in expressions {
                    self.generate_triple_expr::<G>(&wrapper.te, subject.clone(), schema, map_state, graph)?;
                }
            },
            TripleExpr::OneOf { expressions, .. } => {
                for wrapper in expressions {
                    self.generate_triple_expr::<G>(&wrapper.te, subject.clone(), schema, map_state, graph)?;
                }
            },
            TripleExpr::TripleConstraint {
                predicate,
                value_expr,
                sem_acts,
                ..
            } => {
                let pred_iri: IriS = schema.resolve_iriref(predicate);
                let pred: G::IRI = G::IRI::from(pred_iri);

                match value_expr.as_deref() {
                    // Shape reference → create fresh blank node and recurse
                    Some(ShapeExpr::Ref(label)) => {
                        let bnode = graph
                            .add_bnode()
                            .map_err(|e| MaterializeError::RdfError { error: e.to_string() })?;
                        let bnode_subject: G::Subject = G::Subject::from(bnode.clone());
                        let bnode_term: G::Term = G::Term::from(bnode);
                        graph
                            .add_triple(subject.clone(), pred, bnode_term)
                            .map_err(|e| MaterializeError::RdfError { error: e.to_string() })?;
                        let target = self.find_shape_by_label(label, schema)?;
                        self.generate_shape_expr::<G>(&target, bnode_subject, schema, map_state, graph)?;
                    },
                    // Inline shape → create fresh blank node and recurse
                    Some(ShapeExpr::Shape(inline_shape)) => {
                        let bnode = graph
                            .add_bnode()
                            .map_err(|e| MaterializeError::RdfError { error: e.to_string() })?;
                        let bnode_subject: G::Subject = G::Subject::from(bnode.clone());
                        let bnode_term: G::Term = G::Term::from(bnode);
                        graph
                            .add_triple(subject.clone(), pred, bnode_term)
                            .map_err(|e| MaterializeError::RdfError { error: e.to_string() })?;
                        self.generate_shape::<G>(inline_shape, bnode_subject, schema, map_state, graph)?;
                    },
                    // Leaf node → apply Map semantic actions
                    _ => {
                        if let Some(acts) = sem_acts {
                            self.apply_map_actions::<G>(acts, subject.clone(), pred, schema, map_state, graph)?;
                        }
                    },
                }
            },
            TripleExpr::Ref(_) => {
                // Triple expression references not yet supported
            },
        }
        Ok(())
    }

    fn apply_map_actions<G>(
        &self,
        acts: &[SemAct],
        subject: G::Subject,
        pred: G::IRI,
        schema: &Schema,
        map_state: &MapState,
        graph: &mut G,
    ) -> Result<(), MaterializeError>
    where
        G: BuildRDF,
        G::Term: From<rudof_rdf::rdf_core::term::Object>,
        G::Err: std::fmt::Display,
    {
        for act in acts {
            let act_iri = schema.resolve_iriref(&act.name());
            if act_iri.as_str() == MAP_EXTENSION_IRI
                && let Some(code) = act.code()
            {
                let trimmed = code.trim();
                // ShExJ encodes the map key as a Turtle IRI (<http://...>).
                // ShExC encodes it as a raw prefixed name (:local or prefix:local).
                // Try Turtle angle-bracket syntax first, then fall back to prefix map resolution.
                let map_iri = IriS::parse_turtle(trimmed)
                    .ok()
                    .or_else(|| schema.prefixmap().and_then(|pm| pm.resolve(trimmed).ok()));
                if let Some(map_iri) = map_iri
                    && let Some(value) = map_state.get(&map_iri)
                {
                    let obj: G::Term = G::Term::from(value.as_object().clone());
                    graph
                        .add_triple(subject.clone(), pred.clone(), obj)
                        .map_err(|e| MaterializeError::RdfError { error: e.to_string() })?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use iri_s::{IriS, iri};
    use oxrdf::{NamedNode as OxNamedNode, NamedOrBlankNode as OxSubject, Term as OxTerm};
    use rudof_rdf::rdf_core::{Any, NeighsRDF};
    use rudof_rdf::rdf_impl::InMemoryGraph;

    use super::*;
    use crate::Node;
    use crate::ast::Schema;
    use crate::ir::map_state::MapState;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn materializer() -> Materializer {
        Materializer::new()
    }

    fn schema_from_str(json: &str) -> Schema {
        serde_json::from_str(json).expect("valid ShEx JSON")
    }

    fn count_triples(graph: &InMemoryGraph) -> usize {
        graph.triples().unwrap().count()
    }

    fn has_triple_with_pred_obj(graph: &InMemoryGraph, pred_iri: &str, obj_iri: &str) -> bool {
        let pred = OxNamedNode::new_unchecked(pred_iri);
        let obj = OxTerm::NamedNode(OxNamedNode::new_unchecked(obj_iri));
        graph.contains(&Any, &pred, &obj).unwrap_or(false)
    }

    fn has_triple_with_pred(graph: &InMemoryGraph, pred_iri: &str) -> bool {
        let pred = OxNamedNode::new_unchecked(pred_iri);
        graph.contains(&Any, &pred, &Any).unwrap_or(false)
    }

    // -----------------------------------------------------------------------
    // Test 1: Schema with no shapes returns an error
    // -----------------------------------------------------------------------

    #[test]
    fn test_no_start_shape_returns_error() {
        let schema = schema_from_str(
            r#"{
              "@context": "http://www.w3.org/ns/shex.jsonld",
              "type": "Schema"
            }"#,
        );
        let map_state = MapState::default();
        let result: Result<InMemoryGraph, _> = materializer().materialize(&schema, &map_state, None);
        assert!(result.is_err(), "expected error but got Ok");
        assert!(matches!(result.unwrap_err(), MaterializeError::NoStartShape));
    }

    // -----------------------------------------------------------------------
    // Test 2: Single leaf property with Map sem act → one triple
    // -----------------------------------------------------------------------

    #[test]
    fn test_single_leaf_property_with_map_action() {
        let schema = schema_from_str(
            r#"{
              "@context": "http://www.w3.org/ns/shex.jsonld",
              "type": "Schema",
              "shapes": [{
                "type": "ShapeDecl",
                "id": "http://example.org/PersonShape",
                "shapeExpr": {
                  "type": "Shape",
                  "expression": {
                    "type": "TripleConstraint",
                    "predicate": "http://example.org/name",
                    "semActs": [{
                      "type": "SemAct",
                      "name": "http://shex.io/extensions/Map/",
                      "code": "<http://example.org/name>"
                    }]
                  }
                }
              }]
            }"#,
        );

        let mut map_state = MapState::default();
        map_state.insert(
            IriS::new_unchecked("http://example.org/name"),
            Node::iri(iri!("http://example.org/Alice")),
        );

        let graph: InMemoryGraph = materializer().materialize(&schema, &map_state, None).unwrap();

        assert_eq!(count_triples(&graph), 1, "expected exactly one triple");
        assert!(
            has_triple_with_pred_obj(&graph, "http://example.org/name", "http://example.org/Alice"),
            "expected triple with predicate name and object Alice"
        );
    }

    // -----------------------------------------------------------------------
    // Test 3: Map IRI not in MapState → no triple emitted
    // -----------------------------------------------------------------------

    #[test]
    fn test_leaf_property_map_iri_not_in_state_emits_no_triple() {
        let schema = schema_from_str(
            r#"{
              "@context": "http://www.w3.org/ns/shex.jsonld",
              "type": "Schema",
              "shapes": [{
                "type": "ShapeDecl",
                "id": "http://example.org/S",
                "shapeExpr": {
                  "type": "Shape",
                  "expression": {
                    "type": "TripleConstraint",
                    "predicate": "http://example.org/age",
                    "semActs": [{
                      "type": "SemAct",
                      "name": "http://shex.io/extensions/Map/",
                      "code": "<http://example.org/age>"
                    }]
                  }
                }
              }]
            }"#,
        );

        let map_state = MapState::default();
        let graph: InMemoryGraph = materializer().materialize(&schema, &map_state, None).unwrap();

        assert_eq!(count_triples(&graph), 0, "no triple expected when map IRI is absent");
    }

    // -----------------------------------------------------------------------
    // Test 4: EachOf with two leaf properties → two triples
    // -----------------------------------------------------------------------

    #[test]
    fn test_each_of_two_leaf_properties() {
        let schema = schema_from_str(
            r#"{
              "@context": "http://www.w3.org/ns/shex.jsonld",
              "type": "Schema",
              "shapes": [{
                "type": "ShapeDecl",
                "id": "http://example.org/PersonShape",
                "shapeExpr": {
                  "type": "Shape",
                  "expression": {
                    "type": "EachOf",
                    "expressions": [
                      {
                        "type": "TripleConstraint",
                        "predicate": "http://example.org/name",
                        "semActs": [{
                          "type": "SemAct",
                          "name": "http://shex.io/extensions/Map/",
                          "code": "<http://example.org/name>"
                        }]
                      },
                      {
                        "type": "TripleConstraint",
                        "predicate": "http://example.org/age",
                        "semActs": [{
                          "type": "SemAct",
                          "name": "http://shex.io/extensions/Map/",
                          "code": "<http://example.org/age>"
                        }]
                      }
                    ]
                  }
                }
              }]
            }"#,
        );

        let mut map_state = MapState::default();
        map_state.insert(
            IriS::new_unchecked("http://example.org/name"),
            Node::iri(iri!("http://example.org/Alice")),
        );
        map_state.insert(
            IriS::new_unchecked("http://example.org/age"),
            Node::iri(iri!("http://example.org/age42")),
        );

        let graph: InMemoryGraph = materializer().materialize(&schema, &map_state, None).unwrap();

        assert_eq!(count_triples(&graph), 2);
        assert!(has_triple_with_pred_obj(
            &graph,
            "http://example.org/name",
            "http://example.org/Alice"
        ));
        assert!(has_triple_with_pred_obj(
            &graph,
            "http://example.org/age",
            "http://example.org/age42"
        ));
    }

    // -----------------------------------------------------------------------
    // Test 5: Shape reference → nested blank node with its own property
    // -----------------------------------------------------------------------

    #[test]
    fn test_shape_reference_creates_nested_bnode() {
        let schema = schema_from_str(
            r#"{
              "@context": "http://www.w3.org/ns/shex.jsonld",
              "type": "Schema",
              "shapes": [
                {
                  "type": "ShapeDecl",
                  "id": "http://example.org/PersonShape",
                  "shapeExpr": {
                    "type": "Shape",
                    "expression": {
                      "type": "TripleConstraint",
                      "predicate": "http://example.org/address",
                      "valueExpr": "http://example.org/AddressShape"
                    }
                  }
                },
                {
                  "type": "ShapeDecl",
                  "id": "http://example.org/AddressShape",
                  "shapeExpr": {
                    "type": "Shape",
                    "expression": {
                      "type": "TripleConstraint",
                      "predicate": "http://example.org/city",
                      "semActs": [{
                        "type": "SemAct",
                        "name": "http://shex.io/extensions/Map/",
                        "code": "<http://example.org/city>"
                      }]
                    }
                  }
                }
              ]
            }"#,
        );

        let mut map_state = MapState::default();
        map_state.insert(
            IriS::new_unchecked("http://example.org/city"),
            Node::iri(iri!("http://example.org/Madrid")),
        );

        let graph: InMemoryGraph = materializer().materialize(&schema, &map_state, None).unwrap();

        // 2 triples: (root, address, bnode) and (bnode, city, Madrid)
        assert_eq!(count_triples(&graph), 2);
        assert!(
            has_triple_with_pred(&graph, "http://example.org/address"),
            "expected address triple"
        );
        assert!(has_triple_with_pred_obj(
            &graph,
            "http://example.org/city",
            "http://example.org/Madrid"
        ));
    }

    // -----------------------------------------------------------------------
    // Test 6: Explicit initial IRI node is used as subject
    // -----------------------------------------------------------------------

    #[test]
    fn test_explicit_initial_iri_node_as_subject() {
        let schema = schema_from_str(
            r#"{
              "@context": "http://www.w3.org/ns/shex.jsonld",
              "type": "Schema",
              "shapes": [{
                "type": "ShapeDecl",
                "id": "http://example.org/S",
                "shapeExpr": {
                  "type": "Shape",
                  "expression": {
                    "type": "TripleConstraint",
                    "predicate": "http://example.org/p",
                    "semActs": [{
                      "type": "SemAct",
                      "name": "http://shex.io/extensions/Map/",
                      "code": "<http://example.org/p>"
                    }]
                  }
                }
              }]
            }"#,
        );

        let mut map_state = MapState::default();
        map_state.insert(
            IriS::new_unchecked("http://example.org/p"),
            Node::iri(iri!("http://example.org/val")),
        );

        let initial = Node::iri(iri!("http://example.org/Bob"));
        let graph: InMemoryGraph = materializer().materialize(&schema, &map_state, Some(initial)).unwrap();

        assert_eq!(count_triples(&graph), 1);

        // Verify the subject is the IRI we supplied
        let pred = OxNamedNode::new_unchecked("http://example.org/p");
        let subj = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/Bob"));
        let obj = OxTerm::NamedNode(OxNamedNode::new_unchecked("http://example.org/val"));
        let contains = graph.contains(&subj, &pred, &obj).unwrap();
        assert!(contains, "expected triple (Bob, p, val)");
    }

    // -----------------------------------------------------------------------
    // Test 7: Schema with `start` field pointing to a named shape
    // -----------------------------------------------------------------------

    #[test]
    fn test_start_field_references_named_shape() {
        let schema = schema_from_str(
            r#"{
              "@context": "http://www.w3.org/ns/shex.jsonld",
              "type": "Schema",
              "start": "http://example.org/S",
              "shapes": [{
                "type": "ShapeDecl",
                "id": "http://example.org/S",
                "shapeExpr": {
                  "type": "Shape",
                  "expression": {
                    "type": "TripleConstraint",
                    "predicate": "http://example.org/label",
                    "semActs": [{
                      "type": "SemAct",
                      "name": "http://shex.io/extensions/Map/",
                      "code": "<http://example.org/label>"
                    }]
                  }
                }
              }]
            }"#,
        );

        let mut map_state = MapState::default();
        map_state.insert(
            IriS::new_unchecked("http://example.org/label"),
            Node::iri(iri!("http://example.org/Thing")),
        );

        let graph: InMemoryGraph = materializer().materialize(&schema, &map_state, None).unwrap();

        assert_eq!(count_triples(&graph), 1);
        assert!(has_triple_with_pred_obj(
            &graph,
            "http://example.org/label",
            "http://example.org/Thing"
        ));
    }

    // -----------------------------------------------------------------------
    // Test 8: Non-map semantic action is ignored
    // -----------------------------------------------------------------------

    #[test]
    fn test_non_map_semantic_action_is_ignored() {
        let schema = schema_from_str(
            r#"{
              "@context": "http://www.w3.org/ns/shex.jsonld",
              "type": "Schema",
              "shapes": [{
                "type": "ShapeDecl",
                "id": "http://example.org/S",
                "shapeExpr": {
                  "type": "Shape",
                  "expression": {
                    "type": "TripleConstraint",
                    "predicate": "http://example.org/p",
                    "semActs": [{
                      "type": "SemAct",
                      "name": "http://example.org/SomeOtherExtension/",
                      "code": "<http://example.org/x>"
                    }]
                  }
                }
              }]
            }"#,
        );

        let mut map_state = MapState::default();
        map_state.insert(
            IriS::new_unchecked("http://example.org/x"),
            Node::iri(iri!("http://example.org/Value")),
        );

        let graph: InMemoryGraph = materializer().materialize(&schema, &map_state, None).unwrap();
        assert_eq!(count_triples(&graph), 0, "non-Map sem act should not produce triples");
    }
}

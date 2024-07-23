use std::collections::HashSet;

use oxrdf::Term;
use shacl_ast::{
    node_shape::NodeShape, property_shape::PropertyShape, shape::Shape, target::Target, Schema,
};
use srdf::{Object, SRDFGraph, RDF_TYPE, SRDF};

use crate::{
    constraints::ConstraintFactory,
    validate_error::{
        ValidateError, ValidateError::TargetClassBlankNode, ValidateError::TargetClassLiteral,
        ValidateError::TargetNodeBlankNode,
    },
    validation_report::{report::ValidationReport, result::ValidationResult},
};

trait Validate {
    fn validate(&self, data_graph: &SRDFGraph) -> Option<Vec<ValidationResult>>;

    fn focus_nodes(
        &self,
        data_graph: &SRDFGraph,
        targets: &Vec<Target>,
    ) -> Result<HashSet<Term>, ValidateError> {
        let mut results = HashSet::new();
        for target in targets.to_vec() {
            match target {
                Target::TargetNode(node) => {
                    let ans = match node {
                        Object::Iri(iri) => Term::NamedNode(iri.as_named_node().clone()),
                        Object::BlankNode(_) => return Err(TargetNodeBlankNode),
                        Object::Literal(literal) => Term::Literal(literal.into()),
                    };
                    results.insert(ans);
                }
                Target::TargetClass(class) => {
                    let object = match class {
                        Object::Iri(iri) => Term::NamedNode(iri.as_named_node().to_owned()),
                        Object::BlankNode(_) => return Err(TargetClassBlankNode),
                        Object::Literal(_) => return Err(TargetClassLiteral),
                    };
                    results.extend(
                        data_graph
                            .subjects_with_predicate_object(RDF_TYPE.as_named_node(), &object)?
                            .into_iter()
                            .map(|subject| subject.into())
                            .collect::<HashSet<Term>>(),
                    )
                }
                Target::TargetSubjectsOf(predicate) => results.extend(
                    data_graph
                        .triples_with_predicate(&match predicate.get_iri() {
                            Ok(iri_s) => iri_s.as_named_node().to_owned(),
                            Err(_) => todo!(),
                        })?
                        .into_iter()
                        .map(|triple| triple.subj().into())
                        .collect::<HashSet<Term>>(),
                ),
                Target::TargetObjectsOf(predicate) => results.extend(
                    data_graph
                        .triples_with_predicate(&match predicate.get_iri() {
                            Ok(iri_s) => iri_s.as_named_node().to_owned(),
                            Err(_) => todo!(),
                        })?
                        .into_iter()
                        .map(|triple| triple.obj())
                        .collect::<HashSet<Term>>(),
                ),
            }
        }
        Ok(results)
    }
}

impl Validate for Shape {
    fn validate(&self, data_graph: &SRDFGraph) -> Option<Vec<ValidationResult>> {
        match self {
            Shape::NodeShape(node_shape) => node_shape.validate(data_graph),
            Shape::PropertyShape(property_shape) => property_shape.validate(data_graph),
        }
    }
}

impl Validate for NodeShape {
    fn validate(&self, data_graph: &SRDFGraph) -> Option<Vec<ValidationResult>> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return None;
        }

        let mut results = Vec::new();

        for component in self.components() {
            let constraint = ConstraintFactory::new_constraint(component);
            let focus_nodes = self.focus_nodes(data_graph, self.targets());

            let value_nodes = match focus_nodes {
                Ok(focus_nodes) => focus_nodes,
                Err(_) => todo!(),
            };

            if let Some(result) = constraint.evaluate(data_graph, value_nodes) {
                results.push(result)
            }
        }

        if results.len() > 0 {
            Some(results)
        } else {
            None
        }
    }
}

impl Validate for PropertyShape {
    fn validate(&self, data_graph: &SRDFGraph) -> Option<Vec<ValidationResult>> {
        if *self.is_deactivated() {
            return None;
        }

        let mut results = Vec::new();

        for component in self.components() {
            let constraint = ConstraintFactory::new_constraint(component);
            let focus_nodes = match self.focus_nodes(data_graph, self.targets()) {
                Ok(focus_nodes) => focus_nodes,
                Err(_) => todo!(),
            };

            let mut value_nodes = HashSet::new();
            for focus_node in focus_nodes {
                match self.path() {
                    srdf::SHACLPath::Predicate { pred: _ } => value_nodes
                        .extend(self.get_value_nodes(data_graph, &focus_node, self.path())),
                    srdf::SHACLPath::Alternative { paths } => value_nodes.extend(
                        paths
                            .iter()
                            .flat_map(|path| self.get_value_nodes(data_graph, &focus_node, path))
                            .collect::<HashSet<_>>(),
                    ),
                    srdf::SHACLPath::Sequence { paths } => todo!(),
                    srdf::SHACLPath::Inverse { path } => todo!(),
                    srdf::SHACLPath::ZeroOrMore { path } => todo!(),
                    srdf::SHACLPath::OneOrMore { path } => todo!(),
                    srdf::SHACLPath::ZeroOrOne { path } => todo!(),
                }
            }

            if let Some(result) = constraint.evaluate(data_graph, value_nodes) {
                results.push(result)
            }
        }

        if results.len() > 0 {
            Some(results)
        } else {
            None
        }
    }
}

pub fn validate(
    data_graph: &SRDFGraph,
    shapes_graph: Schema,
) -> Result<ValidationReport, ValidateError> {
    let mut validation_report = ValidationReport::default(); // conforming by default...
    for (_, shape) in shapes_graph.iter() {
        let result = shape.validate(data_graph); // TODO: Extend traits
        if let Some(result) = result {
            validation_report.set_non_conformant();
            validation_report.extend_results(result);
        }
    }
    Ok(validation_report)
}

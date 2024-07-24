use std::collections::HashSet;

use prefixmap::IriRef;
use shacl_ast::{
    node_shape::NodeShape, property_shape::PropertyShape, shape::Shape, target::Target, Schema,
};
use srdf::{Object, RDFNode, SRDFGraph, RDF_TYPE, SRDF};

use crate::{
    constraints::ConstraintFactory,
    helper::oxrdf::{subject_to_node, term_to_node},
    validate_error::ValidateError::{
        self, TargetClassBlankNode, TargetClassLiteral, TargetNodeBlankNode,
    },
    validation_report::report::ValidationReport,
};

trait Validate {
    fn validate(&self, data_graph: &SRDFGraph, report: &mut ValidationReport);

    fn focus_nodes(
        &self,
        graph: &SRDFGraph,
        targets: &Vec<Target>,
    ) -> Result<HashSet<RDFNode>, ValidateError> {
        let mut ans = HashSet::new();
        for target in targets.to_vec() {
            match target {
                Target::TargetNode(node) => self.target_node(node, &mut ans)?,
                Target::TargetClass(class) => self.target_class(graph, class, &mut ans)?,
                Target::TargetSubjectsOf(pred) => self.target_subject_of(graph, pred, &mut ans)?,
                Target::TargetObjectsOf(pred) => self.target_object_of(graph, pred, &mut ans)?,
            }
        }
        Ok(ans)
    }

    fn target_node(
        &self,
        node: Object,
        focus_nodes: &mut HashSet<RDFNode>,
    ) -> Result<(), ValidateError> {
        if let Object::BlankNode(_) = node {
            Err(TargetNodeBlankNode)
        } else {
            focus_nodes.insert(node);
            Ok(())
        }
    }

    fn target_class(
        &self,
        graph: &SRDFGraph,
        class: Object,
        focus_nodes: &mut HashSet<RDFNode>,
    ) -> Result<(), ValidateError> {
        match class {
            Object::Iri(iri) => {
                focus_nodes.extend(
                    graph
                        .subjects_with_predicate_object(
                            RDF_TYPE.as_named_node(),
                            &oxrdf::Term::NamedNode(iri.as_named_node().to_owned()),
                        )?
                        .into_iter()
                        .map(|subject| subject_to_node(subject))
                        .collect::<HashSet<_>>(),
                );
                Ok(())
            }
            Object::BlankNode(_) => return Err(TargetClassBlankNode),
            Object::Literal(_) => return Err(TargetClassLiteral),
        }
    }

    fn target_subject_of(
        &self,
        graph: &SRDFGraph,
        predicate: IriRef,
        focus_nodes: &mut HashSet<RDFNode>,
    ) -> Result<(), ValidateError> {
        let predicate = &match predicate.get_iri() {
            Ok(iri_s) => iri_s.as_named_node().to_owned(),
            Err(_) => todo!(),
        };
        focus_nodes.extend(
            graph
                .triples_with_predicate(&predicate)?
                .into_iter()
                .map(|triple| subject_to_node(triple.subj()))
                .collect::<HashSet<_>>(),
        );
        Ok(())
    }

    fn target_object_of(
        &self,
        graph: &SRDFGraph,
        predicate: IriRef,
        focus_nodes: &mut HashSet<RDFNode>,
    ) -> Result<(), ValidateError> {
        let predicate = &match predicate.get_iri() {
            Ok(iri_s) => iri_s.as_named_node().to_owned(),
            Err(_) => todo!(),
        };
        focus_nodes.extend(
            graph
                .triples_with_predicate(&predicate)?
                .into_iter()
                .map(|triple| term_to_node(triple.obj()))
                .collect::<HashSet<_>>(),
        );
        Ok(())
    }
}

impl Validate for NodeShape {
    fn validate(&self, data_graph: &SRDFGraph, report: &mut ValidationReport) {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return;
        }

        for component in self.components() {
            let constraint = ConstraintFactory::new_constraint(component);

            let value_nodes = match self.focus_nodes(data_graph, self.targets()) {
                Ok(focus_nodes) => focus_nodes,
                Err(_) => todo!(),
            };

            constraint.evaluate(data_graph, value_nodes, report);
        }
    }
}

impl Validate for PropertyShape {
    fn validate(&self, data_graph: &SRDFGraph, report: &mut ValidationReport) {
        if *self.is_deactivated() {
            return;
        }

        for component in self.components() {
            let constraint = ConstraintFactory::new_constraint(component);

            let focus_nodes = match self.focus_nodes(data_graph, self.targets()) {
                Ok(focus_nodes) => focus_nodes,
                Err(_) => todo!(),
            };

            let mut value_nodes = HashSet::new();

            for focus_node in focus_nodes {
                match self.path() {
                    // TODO: fix and improve this
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

            constraint.evaluate(data_graph, value_nodes, report);
        }
    }
}

pub fn validate(
    data_graph: &SRDFGraph,
    shapes_graph: Schema,
) -> Result<ValidationReport, ValidateError> {
    let mut ans = ValidationReport::default(); // conformant by default...
    for (_, shape) in shapes_graph.iter() {
        let result = match shape {
            Shape::NodeShape(node_shape) => node_shape.validate(data_graph, &mut ans),
            Shape::PropertyShape(property_shape) => property_shape.validate(data_graph, &mut ans),
        };
    }
    Ok(ans)
}

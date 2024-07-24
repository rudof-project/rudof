use std::collections::HashSet;

use prefixmap::IriRef;
use shacl_ast::node_kind::NodeKind;
use srdf::{RDFNode, SRDFGraph, RDFS_SUBCLASS_OF, RDF_TYPE, SRDF};

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    helper::oxrdf::{node_to_subject, node_to_term, term_to_subject},
    validation_report::report::ValidationReport,
};

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct ClassConstraintComponent {
    class_rule: RDFNode,
}

impl ClassConstraintComponent {
    pub fn new(class_rule: RDFNode) -> Self {
        ClassConstraintComponent { class_rule }
    }
}

impl Evaluate for ClassConstraintComponent {
    fn evaluate(
        &self,
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        let class_rule = node_to_term(self.class_rule.to_owned()); // TODO: check this clone?
        for node in value_nodes {
            let mut found = false;
            let subject = match node_to_subject(node) {
                Ok(subject) => subject,
                Err(_) => todo!(),
            };
            let predicate = RDF_TYPE.as_named_node();
            let objects = match graph.objects_for_subject_predicate(&subject, predicate) {
                Ok(objects) => objects,
                Err(_) => todo!(),
            };
            for object in objects {
                if object == class_rule {
                    found = true;
                    break;
                } else {
                    let subject = match term_to_subject(object) {
                        Ok(subject) => subject,
                        Err(_) => todo!(),
                    };
                    let predicate = RDFS_SUBCLASS_OF.as_named_node();
                    let objects = match graph.objects_for_subject_predicate(&subject, predicate) {
                        Ok(objects) => objects,
                        Err(_) => todo!(),
                    };
                    if objects.contains(&class_rule) {
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                report.add_result(self.make_validation_result(graph, node))
            }
        }
        Ok(())
    }
}

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct DatatypeConstraintComponent {
    iri_ref: IriRef,
}

impl DatatypeConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        DatatypeConstraintComponent { iri_ref }
    }
}

impl Evaluate for DatatypeConstraintComponent {
    fn evaluate(
        &self,
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

/// sh:nodeKind specifies a condition to be satisfied by the RDF node kind of
/// each value node.
///
/// https://www.w3.org/TR/shacl/#NodeKindConstraintComponent
pub(crate) struct NodeKindConstraintComponent {
    node_kind: NodeKind,
}

impl NodeKindConstraintComponent {
    pub fn new(node_kind: NodeKind) -> Self {
        NodeKindConstraintComponent { node_kind }
    }
}

impl Evaluate for NodeKindConstraintComponent {
    fn evaluate(
        &self,
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

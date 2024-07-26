use std::collections::HashSet;

use indoc::formatdoc;
use oxigraph::{
    model::{NamedNode, NamedNodeRef, Term},
    store::Store,
};
use prefixmap::IriRef;
use shacl_ast::node_kind::NodeKind;
use srdf::{Object, RDFNode};

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    helper::sparql::ask,
    validation_report::report::ValidationReport,
};

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct ClassConstraintComponent {
    class_rule: Option<NamedNode>,
}

impl ClassConstraintComponent {
    pub fn new(class_rule: RDFNode) -> Self {
        let class_rule = match class_rule {
            Object::Iri(i) => NamedNode::new_unchecked(i.to_string()).into(),
            Object::BlankNode(_) => None,
            Object::Literal(_) => None,
        };
        ClassConstraintComponent { class_rule }
    }
}

impl Evaluate for ClassConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            match &self.class_rule {
                Some(class_rule) => {
                    let query = formatdoc! {"
                            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                            ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
                        ", node, class_rule,
                    };
                    println!("{}", query);
                    if !ask(store, query)? {
                        self.make_validation_result(Some(node), report);
                    }
                }
                None => self.make_validation_result(Some(node), report),
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
    datatype: String,
}

impl DatatypeConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        let binding = iri_ref.to_string();
        let datatype = NamedNodeRef::new_unchecked(&binding);
        DatatypeConstraintComponent {
            datatype: datatype.to_string(),
        }
    }
}

impl Evaluate for DatatypeConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if let Term::Literal(literal) = node {
                if literal.datatype().to_string() != self.datatype {
                    self.make_validation_result(Some(node), report);
                }
            } else {
                self.make_validation_result(Some(node), report);
            }
        }
        Ok(())
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
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            let query = match node {
                Term::NamedNode(_) => Some(formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                    ", self.node_kind.to_string()
                }),
                Term::BlankNode(_) => Some(formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:Literal, sh:BlankNodeOrLiteral, sh:IRIOrLiteral ) ) }}
                    ", self.node_kind
                }),
                Term::Literal(_) => Some(formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:BlankNode, sh:BlankNodeOrIRI, sh:BlankNodeOrLiteral ) ) }}
                    ", self.node_kind
                }),
                Term::Triple(_) => None,
            };
            match query {
                Some(query) => {
                    if !ask(store, query)? {
                        self.make_validation_result(Some(node), report);
                    }
                }
                None => self.make_validation_result(Some(node), report),
            }
        }
        Ok(())
    }
}

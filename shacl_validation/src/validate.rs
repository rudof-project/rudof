use std::collections::HashSet;

use indoc::formatdoc;
use oxigraph::{model::Term, store::Store};
use prefixmap::IriRef;
use shacl_ast::{
    node_shape::NodeShape, property_shape::PropertyShape, shape::Shape, target::Target, Schema,
};

use crate::{
    constraints::ConstraintFactory,
    helper::sparql::select,
    validate_error::ValidateError::{
        self, TargetClassBlankNode, TargetClassLiteral, TargetNodeBlankNode,
    },
    validation_report::report::ValidationReport,
};

trait Validate {
    fn validate(&self, store: &Store, report: &mut ValidationReport);

    fn focus_nodes(
        &self,
        store: &Store,
        targets: &Vec<Target>,
    ) -> Result<HashSet<Term>, ValidateError> {
        let mut ans = HashSet::new();
        for target in targets.to_vec() {
            match target {
                Target::TargetNode(node) => self.target_node(store, node, &mut ans)?,
                Target::TargetClass(class) => self.target_class(store, class, &mut ans)?,
                Target::TargetSubjectsOf(pred) => self.target_subject_of(store, pred, &mut ans)?,
                Target::TargetObjectsOf(pred) => self.target_object_of(store, pred, &mut ans)?,
            }
        }
        Ok(ans)
    }

    fn target_node(
        &self,
        store: &Store,
        node: srdf::Object,
        focus_nodes: &mut HashSet<Term>,
    ) -> Result<(), ValidateError> {
        if let srdf::Object::BlankNode(_) = node {
            Err(TargetNodeBlankNode)
        } else {
            let query = formatdoc! {"
                SELECT DISTINCT ?this
                WHERE {{
                    BIND ({} AS ?this)
                }}
            ", node};
            focus_nodes.extend(select(store, query)?);
            Ok(())
        }
    }

    fn target_class(
        &self,
        store: &Store,
        class: srdf::Object,
        focus_nodes: &mut HashSet<Term>,
    ) -> Result<(), ValidateError> {
        match class {
            srdf::Object::Iri(iri) => {
                let query = formatdoc! {"
                    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

                    SELECT DISTINCT ?this
                    WHERE {{
                        ?this rdf:type/rdfs:subClassOf* {} .
                    }}
                ", iri.as_named_node()};
                focus_nodes.extend(select(store, query)?);
                Ok(())
            }
            srdf::Object::BlankNode(_) => Err(TargetClassBlankNode),
            srdf::Object::Literal(_) => Err(TargetClassLiteral),
        }
    }

    fn target_subject_of(
        &self,
        store: &Store,
        predicate: IriRef,
        focus_nodes: &mut HashSet<Term>,
    ) -> Result<(), ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};
        focus_nodes.extend(select(store, query)?);
        Ok(())
    }

    fn target_object_of(
        &self,
        store: &Store,
        predicate: IriRef,
        focus_nodes: &mut HashSet<Term>,
    ) -> Result<(), ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};
        focus_nodes.extend(select(store, query)?);
        Ok(())
    }
}

impl Validate for NodeShape {
    fn validate(&self, store: &Store, report: &mut ValidationReport) {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return;
        }

        for component in self.components() {
            let constraint = ConstraintFactory::new_constraint(component);

            let value_nodes = match self.focus_nodes(store, self.targets()) {
                Ok(focus_nodes) => focus_nodes,
                Err(_) => todo!(),
            };

            println!("{}", component);

            constraint.evaluate(store, value_nodes, report);
        }
    }
}

impl Validate for PropertyShape {
    fn validate(&self, store: &Store, report: &mut ValidationReport) {
        if *self.is_deactivated() {
            return;
        }

        for component in self.components() {
            let constraint = ConstraintFactory::new_constraint(component);

            let focus_nodes = match self.focus_nodes(store, self.targets()) {
                Ok(focus_nodes) => focus_nodes,
                Err(_) => todo!(),
            };

            let mut value_nodes = HashSet::new();

            for focus_node in focus_nodes {
                match self.path() {
                    srdf::SHACLPath::Predicate { pred: _ } => todo!(),
                    srdf::SHACLPath::Alternative { paths } => todo!(),
                    srdf::SHACLPath::Sequence { paths } => todo!(),
                    srdf::SHACLPath::Inverse { path } => todo!(),
                    srdf::SHACLPath::ZeroOrMore { path } => todo!(),
                    srdf::SHACLPath::OneOrMore { path } => todo!(),
                    srdf::SHACLPath::ZeroOrOne { path } => todo!(),
                }
            }

            constraint.evaluate(store, value_nodes, report);
        }
    }
}

pub fn validate(store: &Store, shapes_graph: Schema) -> Result<ValidationReport, ValidateError> {
    let mut ans = ValidationReport::default(); // conformant by default...
    for (_, shape) in shapes_graph.iter() {
        println!("{}", shape);
        match shape {
            Shape::NodeShape(node_shape) => node_shape.validate(store, &mut ans),
            Shape::PropertyShape(property_shape) => property_shape.validate(store, &mut ans),
        };
    }
    Ok(ans)
}

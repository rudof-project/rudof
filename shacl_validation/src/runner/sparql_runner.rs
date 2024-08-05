use std::collections::HashSet;

use indoc::formatdoc;
use shacl_ast::component::Component;
use shacl_ast::property_shape::PropertyShape;
use srdf::QuerySRDF;
use srdf::SHACLPath;

use crate::constraints::SparqlConstraintComponent;
use crate::helper::sparql::select;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

use super::FocusNode;
use super::ValidatorRunner;

type Result<T> = std::result::Result<T, ValidateError>;

pub struct SparqlValidatorRunner;

impl<S: QuerySRDF + 'static> ValidatorRunner<S> for SparqlValidatorRunner {
    fn evaluate(
        &self,
        store: &S,
        component: &Component,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<()> {
        let component: Box<dyn SparqlConstraintComponent<S>> = component.into();
        Ok(component.evaluate_sparql(store, value_nodes, report)?)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &S::Term, focus_nodes: &mut FocusNode<S>) -> Result<()> {
        if S::term_is_bnode(node) {
            return Err(ValidateError::TargetNodeBlankNode);
        }
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                BIND ({} AS ?this)
            }}
        ", node};
        focus_nodes.extend(select(store, query, "this")?);
        Ok(())
    }

    fn target_class(
        &self,
        store: &S,
        class: &S::Term,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        if S::term_is_iri(class) {
            let query = formatdoc! {"
                PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

                SELECT DISTINCT ?this
                WHERE {{
                    ?this rdf:type/rdfs:subClassOf* {} .
                }}
            ", class};
            focus_nodes.extend(select(store, query, "this")?);
            Ok(())
        } else {
            Err(ValidateError::TargetClassNotIri)
        }
    }

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &S::IRI,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};
        focus_nodes.extend(select(store, query, "this")?);
        Ok(())
    }

    fn target_object_of(
        &self,
        store: &S,
        predicate: &S::IRI,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};
        focus_nodes.extend(select(store, query, "this")?);
        Ok(())
    }

    fn predicate(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _predicate: &S::IRI,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn alternative(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn sequence(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn inverse(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn one_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_one(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }
}

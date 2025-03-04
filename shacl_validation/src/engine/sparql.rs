use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::SHACLPath;
use srdf::Sparql;
use srdf::Term;

use crate::constraints::SparqlDeref;
use crate::focus_nodes::FocusNodes;
use crate::helpers::sparql::select;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

use super::Engine;

pub struct SparqlEngine;

impl<S: Sparql + Query> Engine<S> for SparqlEngine {
    fn evaluate(
        store: &S,
        shape: &CompiledShape<S>,
        component: &CompiledComponent<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let validator = component.deref();
        validator.validate_sparql(component, shape, store, value_nodes)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(store: &S, node: &S::Term) -> Result<FocusNodes<S>, ValidateError> {
        if node.is_blank_node() {
            return Err(ValidateError::TargetNodeBlankNode);
        }

        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                BIND ({} AS ?this)
            }}
        ", node};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented("target_node"))
    }

    fn target_class(store: &S, class: &S::Term) -> Result<FocusNodes<S>, ValidateError> {
        if !class.is_iri() {
            return Err(ValidateError::TargetClassNotIri);
        }

        let query = formatdoc! {"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

            SELECT DISTINCT ?this
            WHERE {{
                ?this rdf:type/rdfs:subClassOf* {} .
            }}
        ", class};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented("target_class"))
    }

    fn target_subject_of(store: &S, predicate: &S::IRI) -> Result<FocusNodes<S>, ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented("target_subject_of"))
    }

    fn target_object_of(store: &S, predicate: &S::IRI) -> Result<FocusNodes<S>, ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented("target_object_of"))
    }

    fn implicit_target_class(_store: &S, _class: &S::Term) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented("implicit_target_class"))
    }

    fn predicate(
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _predicate: &S::IRI,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented("predicate path"))
    }

    fn alternative(
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented("alternative path"))
    }

    fn sequence(
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented("sequence path"))
    }

    fn inverse(
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented("inverse path"))
    }

    fn zero_or_more(
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented("zero or more path"))
    }

    fn one_or_more(
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented("one or more path"))
    }

    fn zero_or_one(
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented("zero or one path"))
    }
}

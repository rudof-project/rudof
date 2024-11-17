use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use shacl_ast::shacl_path::SHACLPath;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::TIri;
use srdf::model::rdf::TObject;
use srdf::model::rdf::TPredicate;
use srdf::model::sparql::Sparql;
use srdf::model::Term;

use super::Engine;
use crate::constraints::SparqlDeref;
use crate::focus_nodes::FocusNodes;
use crate::helpers::sparql::select;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

pub struct SparqlEngine;

impl<S: Rdf + Sparql + Clone + 'static> Engine<S> for SparqlEngine {
    fn evaluate(
        &self,
        store: &Store<S>,
        shape: &CompiledShape<S>,
        component: &CompiledComponent<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ValidateError> {
        let validator = component.deref();
        Ok(validator.validate_sparql(component, shape, store, value_nodes, subsetting)?)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(
        &self,
        store: &Store<S>,
        node: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        if node.is_blank_node() {
            return Err(ValidateError::TargetNodeBlankNode);
        }

        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                BIND ({} AS ?this)
            }}
        ", node};

        select(store.inner_store(), query, "this")?;

        Err(ValidateError::NotImplemented {
            msg: "target_node".to_string(),
        })
    }

    fn target_class(
        &self,
        store: &Store<S>,
        class: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
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

        select(store.inner_store(), query, "this")?;

        Err(ValidateError::NotImplemented {
            msg: "target_class".to_string(),
        })
    }

    fn target_subject_of(
        &self,
        store: &Store<S>,
        predicate: &TPredicate<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};

        select(store.inner_store(), query, "this")?;

        Err(ValidateError::NotImplemented {
            msg: "target_subject_of".to_string(),
        })
    }

    fn target_object_of(
        &self,
        store: &Store<S>,
        predicate: &TPredicate<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};

        select(store.inner_store(), query, "this")?;

        Err(ValidateError::NotImplemented {
            msg: "target_object_of".to_string(),
        })
    }

    fn implicit_target_class(
        &self,
        _store: &Store<S>,
        _shape: &CompiledShape<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "implicit_target_class".to_string(),
        })
    }

    fn predicate(
        &self,
        _store: &Store<S>,
        _shape: &CompiledPropertyShape<S>,
        _predicate: &TIri<S::Triple>,
        _focus_node: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "predicate".to_string(),
        })
    }

    fn alternative(
        &self,
        _store: &Store<S>,
        _shape: &CompiledPropertyShape<S>,
        _paths: &[SHACLPath<S::Triple>],
        _focus_node: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "alternative".to_string(),
        })
    }

    fn sequence(
        &self,
        _store: &Store<S>,
        _shape: &CompiledPropertyShape<S>,
        _paths: &[SHACLPath<S::Triple>],
        _focus_node: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "sequence".to_string(),
        })
    }

    fn inverse(
        &self,
        _store: &Store<S>,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath<S::Triple>,
        _focus_node: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "inverse".to_string(),
        })
    }

    fn zero_or_more(
        &self,
        _store: &Store<S>,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath<S::Triple>,
        _focus_node: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_more".to_string(),
        })
    }

    fn one_or_more(
        &self,
        _store: &Store<S>,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath<S::Triple>,
        _focus_node: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "one_or_more".to_string(),
        })
    }

    fn zero_or_one(
        &self,
        _store: &Store<S>,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath<S::Triple>,
        _focus_node: &TObject<S>,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_one".to_string(),
        })
    }
}

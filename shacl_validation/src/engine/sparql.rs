use super::Engine;
use crate::constraints::SparqlDeref;
use crate::focus_nodes::FocusNodes;
use crate::helpers::sparql::select;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use iri_s::IriS;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::property_shape::CompiledPropertyShape;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::QueryRDF;
use srdf::RDFNode;
use srdf::SHACLPath;
use srdf::Term;
use std::fmt::Debug;

pub struct SparqlEngine;

impl<S: QueryRDF + Debug + 'static> Engine<S> for SparqlEngine {
    fn evaluate(
        &self,
        store: &S,
        shape: &CompiledShape<S>,
        component: &CompiledComponent<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape<S>>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let validator = component.deref();
        Ok(validator.validate_sparql(
            component,
            shape,
            store,
            value_nodes,
            source_shape,
            maybe_path,
        )?)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &RDFNode) -> Result<FocusNodes<S>, ValidateError> {
        let node: S::Term = node.clone().into();
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

        Err(ValidateError::NotImplemented {
            msg: "target_node".to_string(),
        })
    }

    fn target_class(&self, store: &S, class: &RDFNode) -> Result<FocusNodes<S>, ValidateError> {
        let class: S::Term = class.clone().into();
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

        Err(ValidateError::NotImplemented {
            msg: "target_class".to_string(),
        })
    }

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &IriS,
    ) -> Result<FocusNodes<S>, ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented {
            msg: "target_subject_of".to_string(),
        })
    }

    fn target_object_of(
        &self,
        store: &S,
        predicate: &IriS,
    ) -> Result<FocusNodes<S>, ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented {
            msg: "target_object_of".to_string(),
        })
    }

    fn implicit_target_class(
        &self,
        _store: &S,
        _shape: &RDFNode,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "implicit_target_class".to_string(),
        })
    }

    fn predicate(
        &self,
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _predicate: &S::IRI,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "predicate".to_string(),
        })
    }

    fn alternative(
        &self,
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "alternative".to_string(),
        })
    }

    fn sequence(
        &self,
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "sequence".to_string(),
        })
    }

    fn inverse(
        &self,
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "inverse".to_string(),
        })
    }

    fn zero_or_more(
        &self,
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_more".to_string(),
        })
    }

    fn one_or_more(
        &self,
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "one_or_more".to_string(),
        })
    }

    fn zero_or_one(
        &self,
        _store: &S,
        _shape: &CompiledPropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_one".to_string(),
        })
    }
}

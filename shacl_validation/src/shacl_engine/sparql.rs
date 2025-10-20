use crate::constraints::ShaclComponent;
use crate::constraints::SparqlDeref;
use crate::focus_nodes::FocusNodes;
use crate::helpers::sparql::select;
use crate::shacl_engine::engine::Engine;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use iri_s::IriS;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::shape::ShapeIR;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::RDFNode;
use srdf::SHACLPath;
use srdf::Term;
use std::fmt::Debug;

pub struct SparqlEngine;

impl<S: QueryRDF + NeighsRDF + Debug + 'static> Engine<S> for SparqlEngine {
    fn evaluate(
        &self,
        store: &S,
        shape: &ShapeIR,
        component: &ComponentIR,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, Box<ValidateError>> {
        let shacl_component = ShaclComponent::new(component);
        let validator = shacl_component.deref();
        let result = validator
            .validate_sparql(
                component,
                shape,
                store,
                value_nodes,
                source_shape,
                maybe_path,
            )
            .map_err(|e| {
                Box::new(ValidateError::ConstraintError {
                    component: component.to_string(),
                    source: e,
                })
            })?;
        Ok(result)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &RDFNode) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let node: S::Term = node.clone().into();
        if node.is_blank_node() {
            return Err(Box::new(ValidateError::TargetNodeBlankNode));
        }

        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                BIND ({} AS ?this)
            }}
        ", node};

        select(store, query, "this").map_err(|e| {
            Box::new(ValidateError::SparqlError {
                msg: "target_node".to_string(),
                source: e,
            })
        })?;

        Err(Box::new(ValidateError::NotImplemented {
            msg: "target_node".to_string(),
        }))
    }

    fn target_class(
        &self,
        store: &S,
        class: &RDFNode,
    ) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let class: S::Term = class.clone().into();
        if !class.is_iri() {
            return Err(Box::new(ValidateError::TargetClassNotIri));
        }

        let query = formatdoc! {"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

            SELECT DISTINCT ?this
            WHERE {{
                ?this rdf:type/rdfs:subClassOf* {} .
            }}
        ", class};

        select(store, query, "this").map_err(|e| {
            Box::new(ValidateError::SparqlError {
                msg: "target_class".to_string(),
                source: e,
            })
        })?;

        Err(Box::new(ValidateError::NotImplemented {
            msg: "target_class".to_string(),
        }))
    }

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &IriS,
    ) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};

        select(store, query, "this").map_err(|e| {
            Box::new(ValidateError::SparqlError {
                msg: "target_subject_of".to_string(),
                source: e,
            })
        })?;

        Err(Box::new(ValidateError::NotImplemented {
            msg: "target_subject_of".to_string(),
        }))
    }

    fn target_object_of(
        &self,
        store: &S,
        predicate: &IriS,
    ) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};

        select(store, query, "this").map_err(|e| {
            Box::new(ValidateError::SparqlError {
                msg: "target_object_of".to_string(),
                source: e,
            })
        })?;

        Err(Box::new(ValidateError::NotImplemented {
            msg: "target_object_of".to_string(),
        }))
    }

    fn implicit_target_class(
        &self,
        _store: &S,
        _shape: &RDFNode,
    ) -> Result<FocusNodes<S>, Box<ValidateError>> {
        Err(Box::new(ValidateError::NotImplemented {
            msg: "implicit_target_class".to_string(),
        }))
    }

    /*fn predicate(
        &self,
        _store: &S,
        _shape: &PropertyShapeIR,
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
        _shape: &PropertyShapeIR,
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
        _shape: &PropertyShapeIR,
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
        _shape: &PropertyShapeIR,
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
        _shape: &PropertyShapeIR,
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
        _shape: &PropertyShapeIR,
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
        _shape: &PropertyShapeIR,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_one".to_string(),
        })
    }*/
}

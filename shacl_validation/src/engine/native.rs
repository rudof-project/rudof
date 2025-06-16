use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::property_shape::CompiledPropertyShape;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::matcher::Any;
use srdf::NeighsRDF;
use srdf::SHACLPath;
use srdf::Term;
use srdf::Triple;
use srdf::rdfs_subclass_of;
use srdf::rdf_type;

use super::Engine;
use crate::constraints::NativeDeref;
use crate::focus_nodes::FocusNodes;
use crate::helpers::srdf::get_objects_for;
use crate::helpers::srdf::get_subjects_for;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use std::fmt::Debug;

pub struct NativeEngine;

impl<S: NeighsRDF + Debug + 'static> Engine<S> for NativeEngine {
    fn evaluate(
        &self,
        store: &S,
        shape: &CompiledShape<S>,
        component: &CompiledComponent<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape<S>>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        tracing::debug!("NativeEngine, evaluate with shape {}", shape.id());
        let validator = component.deref();
        Ok(validator.validate_native(
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
    fn target_node(&self, _: &S, node: &S::Term) -> Result<FocusNodes<S>, ValidateError> {
        if node.is_blank_node() {
            Err(ValidateError::TargetNodeBlankNode)
        } else {
            Ok(FocusNodes::new(std::iter::once(node.clone())))
        }
    }

    fn target_class(&self, store: &S, class: &S::Term) -> Result<FocusNodes<S>, ValidateError> {
        if !class.is_iri() {
            return Err(ValidateError::TargetClassNotIri);
        }

        // TODO: this should not be necessary, check in others triples_matching calls
        let rdf_type: S::IRI = rdf_type().clone().into();

        let focus_nodes = store
            .triples_matching(Any, rdf_type, class.clone())
            .map_err(|_| ValidateError::SRDF)?
            .map(Triple::into_subject)
            .map(Into::into);

        Ok(FocusNodes::new(focus_nodes))
    }

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &S::IRI,
    ) -> Result<FocusNodes<S>, ValidateError> {
        let subjects = store
            .triples_with_predicate(predicate.clone())
            .map_err(|_| ValidateError::SRDF)?
            .map(Triple::into_subject)
            .map(Into::into);
        let focus_nodes = FocusNodes::new(subjects);
        Ok(focus_nodes)
    }

    fn target_object_of(
        &self,
        store: &S,
        predicate: &S::IRI,
    ) -> Result<FocusNodes<S>, ValidateError> {
        let objects = store
            .triples_with_predicate(predicate.clone())
            .map_err(|_| ValidateError::SRDF)?
            .map(Triple::into_object);
        Ok(FocusNodes::new(objects))
    }

    fn implicit_target_class(
        &self,
        store: &S,
        subject: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        let targets = get_subjects_for(store, &rdf_type().clone().into(), subject)?;

        let subclass_targets = get_subjects_for(store, &rdfs_subclass_of().clone().into(), subject)?
            .into_iter()
            .flat_map(move |subclass| {
                get_subjects_for(store, &rdf_type().clone().into(), &subclass)
                    .into_iter()
                    .flatten()
            });

        Ok(FocusNodes::new(targets.into_iter().chain(subclass_targets)))
    }

    fn predicate(
        &self,
        store: &S,
        _: &CompiledPropertyShape<S>,
        predicate: &S::IRI,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        Ok(FocusNodes::new(
            get_objects_for(store, focus_node, predicate)?.into_iter(),
        ))
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

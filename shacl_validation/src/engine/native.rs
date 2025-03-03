use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::matcher::Any;
use srdf::Query;
use srdf::SHACLPath;
use srdf::Term;
use srdf::Triple;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;

use crate::constraints::NativeDeref;
use crate::focus_nodes::FocusNodes;
use crate::helpers::srdf::get_objects_for;
use crate::helpers::srdf::get_subjects_for;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

use super::Engine;

pub struct NativeEngine;

impl<Q: Query> Engine<Q> for NativeEngine {
    fn evaluate(
        store: &Q,
        shape: &CompiledShape<Q>,
        component: &CompiledComponent<Q>,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let validator = component.deref();
        Ok(validator.validate(component, shape, store, value_nodes)?)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(_: &Q, node: &Q::Term) -> Result<FocusNodes<Q>, ValidateError> {
        if node.is_blank_node() {
            Err(ValidateError::TargetNodeBlankNode)
        } else {
            Ok(FocusNodes::new(std::iter::once(node.clone())))
        }
    }

    fn target_class(store: &Q, class: &Q::Term) -> Result<FocusNodes<Q>, ValidateError> {
        if !class.is_iri() {
            return Err(ValidateError::TargetClassNotIri);
        }

        // TODO: this should not be necessary, check in others triples_matching calls
        let rdf_type: Q::IRI = RDF_TYPE.clone().into();

        let focus_nodes = store
            .triples_matching(Any, rdf_type, class.clone())
            .map_err(|_| ValidateError::SRDF)?
            .map(Triple::into_subject)
            .map(Into::into);

        Ok(FocusNodes::new(focus_nodes))
    }

    fn target_subject_of(store: &Q, predicate: &Q::IRI) -> Result<FocusNodes<Q>, ValidateError> {
        let subjects = store
            .triples_with_predicate(predicate.clone())
            .map_err(|_| ValidateError::SRDF)?
            .map(Triple::into_subject)
            .map(Into::into);
        Ok(FocusNodes::new(subjects))
    }

    fn target_object_of(store: &Q, predicate: &Q::IRI) -> Result<FocusNodes<Q>, ValidateError> {
        let objects = store
            .triples_with_predicate(predicate.clone())
            .map_err(|_| ValidateError::SRDF)?
            .map(Triple::into_object);
        Ok(FocusNodes::new(objects))
    }

    fn implicit_target_class(store: &Q, subject: &Q::Term) -> Result<FocusNodes<Q>, ValidateError> {
        let targets = get_subjects_for(store, &RDF_TYPE.clone().into(), subject)?;

        let subclass_targets = get_subjects_for(store, &RDFS_SUBCLASS_OF.clone().into(), subject)?
            .into_iter()
            .flat_map(move |subclass| {
                get_subjects_for(store, &RDF_TYPE.clone().into(), &subclass)
                    .into_iter()
                    .flatten()
            });

        Ok(FocusNodes::new(targets.into_iter().chain(subclass_targets)))
    }

    fn predicate(
        store: &Q,
        _: &CompiledPropertyShape<Q>,
        predicate: &Q::IRI,
        focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        let values = get_objects_for(store, focus_node, predicate)?;
        Ok(FocusNodes::new(values.into_iter()))
    }

    fn alternative(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _paths: &[SHACLPath],
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "alternative".to_string(),
        })
    }

    fn sequence(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _paths: &[SHACLPath],
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "sequence".to_string(),
        })
    }

    fn inverse(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _path: &SHACLPath,
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "inverse".to_string(),
        })
    }

    fn zero_or_more(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _path: &SHACLPath,
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_more".to_string(),
        })
    }

    fn one_or_more(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _path: &SHACLPath,
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "one_or_more".to_string(),
        })
    }

    fn zero_or_one(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _path: &SHACLPath,
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_one".to_string(),
        })
    }
}

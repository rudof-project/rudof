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
        validator.validate(component, shape, store, value_nodes)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(_: &Q, node: &Q::Term) -> Result<FocusNodes<Q>, ValidateError> {
        if node.is_blank_node() {
            return Err(ValidateError::TargetNodeBlankNode);
        }
        Ok(FocusNodes::new(std::iter::once(node.clone())))
    }

    fn target_class(store: &Q, class: &Q::Term) -> Result<FocusNodes<Q>, ValidateError> {
        if !class.is_iri() {
            return Err(ValidateError::TargetClassNotIri);
        }

        let focus_nodes = store
            .triples_matching(Any, RDF_TYPE.clone(), class.clone())
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

    fn implicit_target_class(store: &Q, class: &Q::Term) -> Result<FocusNodes<Q>, ValidateError> {
        let classes = store
            .triples_matching(Any, RDF_TYPE.clone(), class.clone())
            .map_err(|_| ValidateError::SRDF)?
            .map(Triple::into_subject)
            .map(Into::into);

        let subclasses = store
            .triples_matching(Any, RDFS_SUBCLASS_OF.clone(), class.clone())
            .map_err(|_| ValidateError::SRDF)?
            .flat_map(|triple| store.triples_matching(Any, RDF_TYPE.clone(), triple.into_subject()))
            .flatten()
            .map(Triple::into_subject)
            .map(Into::into);

        Ok(FocusNodes::new(classes.into_iter().chain(subclasses)))
    }

    fn predicate(
        store: &Q,
        _: &CompiledPropertyShape<Q>,
        predicate: &Q::IRI,
        focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        let focus_node: Q::Subject = focus_node
            .clone()
            .try_into()
            .map_err(|_| ValidateError::ExpectedSubject(focus_node.to_string()))?;
        let values = store
            .triples_matching(focus_node, predicate.clone(), Any)
            .map_err(|_| ValidateError::SRDF)?
            .map(Triple::into_object);
        Ok(FocusNodes::new(values.into_iter()))
    }

    fn alternative(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _paths: &[SHACLPath],
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented("alternative path"))
    }

    fn sequence(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _paths: &[SHACLPath],
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented("sequence path"))
    }

    fn inverse(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _path: &SHACLPath,
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented("inverse path"))
    }

    fn zero_or_more(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _path: &SHACLPath,
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented("zero or more path"))
    }

    fn one_or_more(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _path: &SHACLPath,
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented("one or more path"))
    }

    fn zero_or_one(
        _store: &Q,
        _shape: &CompiledPropertyShape<Q>,
        _path: &SHACLPath,
        _focus_node: &Q::Term,
    ) -> Result<FocusNodes<Q>, ValidateError> {
        Err(ValidateError::NotImplemented("zero or one path"))
    }
}

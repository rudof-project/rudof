use oxrdf::Triple;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use shacl_ast::shacl_path::SHACLPath;
use srdf::model::matcher::Matcher;
use srdf::model::rdf::Iri;
use srdf::model::rdf::Object;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;
use srdf::model::Term as _;
use srdf::model::Triple;
use srdf::RDFS_CLASS;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;

use crate::constraints::NativeDeref;
use crate::focus_nodes::FocusNodes;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

use super::Engine;

pub struct NativeEngine;

impl<R: Rdf> Engine<R> for NativeEngine
where
    ValidateError: From<R::Error>,
{
    fn evaluate(
        &self,
        store: &Store<R>,
        shape: &CompiledShape<R>,
        component: &CompiledComponent<R>,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ValidateError> {
        let validator = component.deref();
        Ok(validator.validate_native(component, shape, store, Self, value_nodes, subsetting)?)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, _: &Store<R>, node: &Object<R>) -> Result<FocusNodes<R>, ValidateError> {
        if node.is_blank_node() {
            Err(ValidateError::TargetNodeBlankNode)
        } else {
            Ok(FocusNodes::new(std::iter::once(node.clone())))
        }
    }

    fn target_class(
        &self,
        store: &Store<R>,
        class: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        if !class.is_iri() {
            return Err(ValidateError::TargetClassNotIri);
        }

        let triples = store
            .inner_store()
            .triples_matching(Matcher::Any, RDF_TYPE.into(), class.clone())?
            .map(Triple::into_subject)
            .map(Into::into);

        Ok(FocusNodes::new(triples))
    }

    fn target_subject_of(
        &self,
        store: &Store<R>,
        predicate: &Predicate<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        let triples = store
            .inner_store()
            .triples_with_predicate(predicate.clone())?
            .map(Triple::into_subject)
            .map(Into::into);

        Ok(FocusNodes::new(triples))
    }

    fn target_object_of(
        &self,
        store: &Store<R>,
        predicate: &Predicate<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        let triples = store
            .inner_store()
            .triples_with_predicate(predicate.clone())?
            .map(Triple::into_object);

        Ok(FocusNodes::new(triples))
    }

    fn implicit_target_class(
        &self,
        store: &Store<R>,
        shape: &CompiledShape<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        let ctypes = store
            .inner_store()
            .triples_matching(shape.id().clone(), RDF_TYPE.into(), Matcher::Any)?
            .map(Triple::into_object);

        let subclasses = store
            .inner_store()
            .triples_matching(Matcher::Any, RDFS_SUBCLASS_OF.into(), RDFS_CLASS.into())?
            .map(Triple::into_subject)
            .collect::<Vec<_>>()
            .push(RDFS_SUBCLASS_OF.into());

        if ctypes.any(|t| subclasses.contains(t)) {
            let actual_class_nodes = store
                .inner_store()
                .triples_matching(Matcher::Any, RDF_TYPE.into(), shape.id())?
                .map(Triple::into_subject);

            let subclass_targets = store
                .inner_store()
                .triples_matching(Matcher::Any, RDFS_SUBCLASS_OF.into(), shape.id())?
                .map(Triple::into_subject)
                .filter_map(move |subclass| {
                    store
                        .inner_store()
                        .triples_matching(Matcher::Any, RDF_TYPE.into(), subclass)
                        .ok()
                })
                .flatten();

            let focus_nodes = actual_class_nodes.into_iter().chain(subclass_targets);

            Ok(FocusNodes::new(focus_nodes))
        } else {
            Ok(FocusNodes::default())
        }
    }

    fn predicate(
        &self,
        store: &Store<R>,
        _: &CompiledPropertyShape<R>,
        predicate: &Iri<R::Triple>,
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        let triples = store
            .inner_store()
            .triples_matching(focus_node.clone(), predicate.clone(), Matcher::Any)?
            .map(Triple::into_object);

        Ok(FocusNodes::new(triples))
    }

    fn alternative(
        &self,
        _store: &Store<R>,
        _shape: &CompiledPropertyShape<R>,
        _paths: &[SHACLPath<R::Triple>],
        _focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "alternative".to_string(),
        })
    }

    fn sequence(
        &self,
        _store: &Store<R>,
        _shape: &CompiledPropertyShape<R>,
        _paths: &[SHACLPath<R::Triple>],
        _focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "sequence".to_string(),
        })
    }

    fn inverse(
        &self,
        _store: &Store<R>,
        _shape: &CompiledPropertyShape<R>,
        _path: &SHACLPath<R::Triple>,
        _focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "inverse".to_string(),
        })
    }

    fn zero_or_more(
        &self,
        _store: &Store<R>,
        _shape: &CompiledPropertyShape<R>,
        _path: &SHACLPath<R::Triple>,
        _focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_more".to_string(),
        })
    }

    fn one_or_more(
        &self,
        _store: &Store<R>,
        _shape: &CompiledPropertyShape<R>,
        _path: &SHACLPath<R::Triple>,
        _focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "one_or_more".to_string(),
        })
    }

    fn zero_or_one(
        &self,
        _store: &Store<R>,
        _shape: &CompiledPropertyShape<R>,
        _path: &SHACLPath<R::Triple>,
        _focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        Err(ValidateError::NotImplemented {
            msg: "zero_or_one".to_string(),
        })
    }
}

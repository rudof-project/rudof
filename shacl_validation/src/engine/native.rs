use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use shacl_ast::shacl_path::SHACLPath;
use srdf::model::rdf::Iri;
use srdf::model::rdf::Object;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;
use srdf::model::Iri as _;
use srdf::model::Term as _;
use srdf::model::Triple;
use srdf::RDFS_CLASS;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;

use crate::constraints::NativeDeref;
use crate::focus_nodes::FocusNodes;
use crate::helpers::srdf::get_objects_for;
use crate::helpers::srdf::get_subjects_for;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

use super::Engine;

pub struct NativeEngine;

impl<R: Rdf + Clone + 'static> Engine<R> for NativeEngine {
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

        let rdf_type = Predicate::<R>::new(RDF_TYPE.as_str());

        let triples = match store
            .inner_store()
            .triples_matching(None, Some(&rdf_type), Some(class))
        {
            Ok(subjects) => subjects.map(Triple::subj).map(Clone::clone).map(Into::into),
            Err(_) => return Err(ValidateError::SRDF),
        };

        Ok(FocusNodes::new(triples))
    }

    fn target_subject_of(
        &self,
        store: &Store<R>,
        predicate: &Predicate<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        let triples = match store.inner_store().triples_with_predicate(predicate) {
            Ok(triples) => triples.map(Triple::subj).map(Clone::clone).map(Into::into),
            Err(_) => return Err(ValidateError::SRDF),
        };

        Ok(FocusNodes::new(triples))
    }

    fn target_object_of(
        &self,
        store: &Store<R>,
        predicate: &Predicate<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        let triples = match store.inner_store().triples_with_predicate(predicate) {
            Ok(triples) => triples.map(Triple::obj).map(Clone::clone).map(Into::into),
            Err(_) => return Err(ValidateError::SRDF),
        };

        Ok(FocusNodes::new(triples))
    }

    fn implicit_target_class(
        &self,
        store: &Store<R>,
        shape: &CompiledShape<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        let ctypes = get_objects_for(
            store.inner_store(),
            shape.id(),
            &Predicate::<R>::new(RDF_TYPE.as_str()),
        )?;

        let mut subclasses = get_subjects_for(
            store.inner_store(),
            &Predicate::<R>::new(RDFS_SUBCLASS_OF.as_str()),
            &Predicate::<R>::new(RDFS_CLASS.as_str()).into(),
        )?;

        subclasses.insert(Predicate::<R>::new(RDFS_SUBCLASS_OF.as_str()).into());

        if ctypes.iter().any(|t| subclasses.contains(t)) {
            let actual_class_nodes = get_subjects_for(
                store.inner_store(),
                &Predicate::<R>::new(RDF_TYPE.as_str()),
                shape.id(),
            )?;

            let subclass_targets = get_subjects_for(
                store.inner_store(),
                &Predicate::<R>::new(RDFS_SUBCLASS_OF.as_str()),
                shape.id(),
            )?
            .into_iter()
            .flat_map(move |subclass| {
                get_subjects_for(
                    store.inner_store(),
                    &Predicate::<R>::new(RDF_TYPE.as_str()),
                    &subclass,
                )
                .into_iter()
                .flatten()
            });

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
        let predicate = Predicate::<R>::new(&predicate.to_string());
        Ok(FocusNodes::new(
            get_objects_for(store.inner_store(), focus_node, &predicate)?.into_iter(),
        ))
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

use shacl_ast::property_shape::PropertyShape;
use srdf::SHACLPath;
use srdf::RDFS_CLASS;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;
use srdf::SRDF;

use crate::constraints::DefaultConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::helper::srdf::get_objects_for;
use crate::helper::srdf::get_subjects_for;
use crate::validate_error::ValidateError;
use crate::validation_report::result::LazyValidationIterator;
use crate::Targets;
use crate::ValueNodes;

use super::ValidatorRunner;

pub struct DefaultValidatorRunner;

impl<S: SRDF + 'static> ValidatorRunner<S> for DefaultValidatorRunner {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ValidateError> {
        let component: Box<dyn DefaultConstraintComponent<S>> =
            evaluation_context.component().into();
        Ok(component.evaluate_default(validation_context, evaluation_context, value_nodes)?)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, _store: &S, node: &S::Term) -> Result<Targets<S>, ValidateError> {
        if S::term_is_bnode(node) {
            Err(ValidateError::TargetNodeBlankNode)
        } else {
            Ok(Targets::new(std::iter::once(node.clone())))
        }
    }

    fn target_class(&self, store: &S, class: &S::Term) -> Result<Targets<S>, ValidateError> {
        if !S::term_is_iri(class) {
            return Err(ValidateError::TargetClassNotIri);
        }

        let subjects = match store.subjects_with_predicate_object(&S::iri_s2iri(&RDF_TYPE), class) {
            Ok(subjects) => subjects,
            Err(_) => return Err(ValidateError::SRDF),
        };

        let targets = subjects.iter().map(|subject| S::subject_as_term(subject));

        Ok(Targets::new(targets))
    }

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &S::IRI,
    ) -> Result<Targets<S>, ValidateError> {
        let triples = match store.triples_with_predicate(predicate) {
            Ok(triples) => triples,
            Err(_) => return Err(ValidateError::SRDF),
        };

        let targets = triples
            .iter()
            .map(|triple| S::subject_as_term(&triple.subj()));

        Ok(Targets::new(targets))
    }

    fn target_object_of(&self, store: &S, predicate: &S::IRI) -> Result<Targets<S>, ValidateError> {
        let triples = match store.triples_with_predicate(predicate) {
            Ok(triples) => triples,
            Err(_) => return Err(ValidateError::SRDF),
        };

        let targets = triples.into_iter().map(|triple| triple.obj());

        Ok(Targets::new(targets))
    }

    fn implicit_target_class(
        &self,
        store: &S,
        shape: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        let ctypes = get_objects_for(store, shape, &S::iri_s2iri(&RDF_TYPE))?;

        let mut subclasses = get_subjects_for(
            store,
            &S::iri_s2iri(&RDFS_SUBCLASS_OF),
            &S::iri_s2term(&RDFS_CLASS),
        )?;

        subclasses.insert(S::iri_s2term(&RDFS_CLASS));

        if ctypes.iter().any(|t| subclasses.contains(t)) {
            let actual_class_nodes = get_subjects_for(store, &S::iri_s2iri(&RDF_TYPE), shape)?;

            let subclass_targets =
                get_subjects_for(store, &S::iri_s2iri(&RDFS_SUBCLASS_OF), shape)?
                    .into_iter()
                    .flat_map(move |subclass| {
                        get_subjects_for(store, &S::iri_s2iri(&RDF_TYPE), &subclass)
                            .into_iter()
                            .flatten()
                    });

            let targets = actual_class_nodes.into_iter().chain(subclass_targets);
            Ok(Targets::new(targets))
        } else {
            Ok(Targets::default())
        }
    }

    fn predicate(
        &self,
        store: &S,
        _shape: &PropertyShape,
        predicate: &S::IRI,
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Ok(Targets::new(
            get_objects_for(store, focus_node, predicate)?.into_iter(),
        ))
    }

    fn alternative(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn sequence(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn inverse(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn zero_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn one_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn zero_or_one(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }
}

use std::collections::HashSet;

use iri_s::IriS;
use prefixmap::IriRef;
use shacl_ast::component::Component;
use shacl_ast::property_shape::PropertyShape;
use srdf::SHACLPath;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::DefaultConstraintComponent;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

use super::ValidatorRunner;

type Result<T> = std::result::Result<T, ValidateError>;

pub struct DefaultValidatorRunner;

impl<S: SRDF + 'static> ValidatorRunner<S> for DefaultValidatorRunner {
    fn evaluate(
        &self,
        store: &S,
        component: &Component,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<()> {
        let component: Box<dyn DefaultConstraintComponent<S>> = component.into();
        Ok(component.evaluate_default(store, value_nodes, report)?)
    }

    fn target_node(&self, _: &S, node: &S::Term, focus_nodes: &mut HashSet<S::Term>) -> Result<()> {
        if S::term_is_bnode(node) {
            Err(ValidateError::TargetNodeBlankNode)
        } else {
            focus_nodes.insert(node.to_owned());
            Ok(())
        }
    }

    fn target_class(
        &self,
        store: &S,
        class: &S::Term,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        if S::term_as_iri(class).is_some() {
            let subjects =
                match store.subjects_with_predicate_object(&S::iri_s2iri(&srdf::RDF_TYPE), class) {
                    Ok(subjects) => subjects,
                    Err(_) => return Err(ValidateError::SRDF),
                };
            let ans = subjects
                .into_iter()
                .map(|subject| S::subject_as_term(&subject))
                .collect::<HashSet<_>>();
            focus_nodes.extend(ans);
            Ok(())
        } else {
            Err(ValidateError::TargetClassNotIri)
        }
    }

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        let triples = match store.triples_with_predicate(&S::iri_s2iri(&predicate.get_iri()?)) {
            Ok(triples) => triples,
            Err(_) => return Err(ValidateError::SRDF),
        };
        let ans = triples
            .into_iter()
            .map(|triple| S::subject_as_term(&triple.subj()))
            .collect::<HashSet<_>>();
        focus_nodes.extend(ans);
        Ok(())
    }

    fn target_object_of(
        &self,
        store: &S,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        let triples = match store.triples_with_predicate(&S::iri_s2iri(&predicate.get_iri()?)) {
            Ok(triples) => triples,
            Err(_) => return Err(ValidateError::SRDF),
        };
        let ans: HashSet<<S as SRDFBasic>::Term> = triples
            .into_iter()
            .map(|triple| triple.obj())
            .collect::<HashSet<_>>();
        focus_nodes.extend(ans);
        Ok(())
    }

    fn predicate(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _predicate: &IriS,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn alternative(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn sequence(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn inverse(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn one_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_one(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: S::Term,
        _value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }
}

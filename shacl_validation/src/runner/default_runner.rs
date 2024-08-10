use std::collections::HashSet;

use shacl_ast::property_shape::PropertyShape;
use srdf::SHACLPath;
use srdf::SRDFBasic;
use srdf::RDFS_CLASS;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;
use srdf::SRDF;

use crate::helper::srdf::get_objects_for;
use crate::helper::srdf::get_subjects_for;
use crate::shape::ValueNode;
use crate::validate_error::ValidateError;

use super::FocusNode;
use super::Result;
use super::ValidatorRunner;

pub struct DefaultValidatorRunner;

impl<S: SRDF + 'static> ValidatorRunner<S> for DefaultValidatorRunner {
    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, _: &S, node: &S::Term, focus_nodes: &mut FocusNode<S>) -> Result<()> {
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
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        if S::term_as_iri(class).is_some() {
            let subjects =
                match store.subjects_with_predicate_object(&S::iri_s2iri(&RDF_TYPE), class) {
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
        predicate: &S::IRI,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        let triples = match store.triples_with_predicate(predicate) {
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
        predicate: &S::IRI,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        let triples = match store.triples_with_predicate(predicate) {
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

    fn implicit_target_class(
        &self,
        store: &S,
        shape: &S::Term,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        let ctypes = get_objects_for(store, shape, &S::iri_s2iri(&RDF_TYPE))?;
        let mut subclasses = get_subjects_for(
            store,
            &S::iri_s2iri(&RDFS_SUBCLASS_OF),
            &S::iri_s2term(&RDFS_CLASS),
        )?;
        subclasses.insert(S::iri_s2term(&RDFS_CLASS));

        if ctypes.iter().any(|t| subclasses.contains(t)) {
            focus_nodes.extend(get_subjects_for(store, &S::iri_s2iri(&RDF_TYPE), shape)?); // the actual class

            let subclass_targets = // transitive classes (i.e subClassOf)
                get_subjects_for(store, &S::iri_s2iri(&RDFS_SUBCLASS_OF), shape)?
                    .into_iter()
                    .flat_map(|subclass| {
                        get_subjects_for(store, &S::iri_s2iri(&RDF_TYPE), &subclass)
                    })
                    .flatten()
                    .collect::<HashSet<_>>();

            focus_nodes.extend(subclass_targets);
        }

        Ok(())
    }

    fn predicate(
        &self,
        store: &S,
        _shape: &PropertyShape,
        predicate: &S::IRI,
        focus_node: &S::Term,
        value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        value_nodes.insert(
            focus_node.to_owned(),
            get_objects_for(store, focus_node, predicate)?,
        );
        Ok(())
    }

    fn alternative(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn sequence(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn inverse(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn one_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_one(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }
}

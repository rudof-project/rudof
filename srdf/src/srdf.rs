use std::collections::HashSet;
//use std::hash::Hash;

use crate::SRDFComparisons;

pub trait SRDF: SRDFComparisons {
    fn get_predicates_for_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashSet<Self::IRI>, Self::Err>;

    fn get_objects_for_subject_predicate(
        &self,
        subject: &Self::Subject,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err>;

    fn get_subjects_for_object_predicate(
        &self,
        object: &Self::Term,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Subject>, Self::Err>;

    /// Get the neighbours of a term
    /// This code creates an intermediate vector and is not very efficient
    fn get_neighs(
        &self,
        node: &Self::Term,
    ) -> Result<Vec<(Self::IRI, HashSet<Self::Term>)>, Self::Err> {
        match self.term_as_subject(node) {
            None => Ok(Vec::new()),
            Some(subject) => {
                let mut result = Vec::new();
                let preds = self.get_predicates_for_subject(&subject)?;
                for pred in preds {
                    let objs = self.get_objects_for_subject_predicate(&subject, &pred)?;
                    result.push((pred.clone(), objs));
                }
                Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {}

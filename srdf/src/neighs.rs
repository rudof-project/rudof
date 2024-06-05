use std::{collections::HashSet, vec::IntoIter};

use crate::SRDF;

pub enum Neigh<S>
where
    S: SRDF,
{
    Direct { p: S::IRI, o: S::Term },
    Inverse { s: S::Subject, p: S::IRI },
}

impl<S> Neigh<S>
where
    S: SRDF,
{
    pub fn direct(pred: S::IRI, object: S::Term) -> Neigh<S> {
        Neigh::Direct { p: pred, o: object }
    }

    pub fn inverse(pred: S::IRI, subject: S::Subject) -> Neigh<S> {
        Neigh::Inverse {
            p: pred,
            s: subject,
        }
    }
}

// TODO...the following code is just a draft...
// I would like to generate the neighs as an iterator...
pub struct NeighsIterator<S>
where
    S: SRDF,
{
    _term: S::Term,
    _neigh_iter: IntoIter<Neigh<S>>,
}

impl<S> NeighsIterator<S>
where
    S: SRDF,
{
    pub fn new(term: S::Term, rdf: S) -> Result<NeighsIterator<S>, S::Err> {
        match S::term_as_subject(&term) {
            None => {
                todo!()
            }
            Some(subject) => {
                let preds: HashSet<S::IRI> = rdf.predicates_for_subject(&subject)?;
                let _qs = preds.into_iter();
                /*let vv = qs.flat_map(|p| {
                    let objs = rdf.get_objects_for_subject_predicate(&subject, &p)?;
                    objs.into_iter().map(|o| Neigh::Direct { p, o })
                });*/
                todo!(); // Ok(vv)
            }
        }
        // NeighsIterator { term, objectsIter }
    }
}

impl<S> FromIterator<Neigh<S>> for NeighsIterator<S>
where
    S: SRDF,
{
    fn from_iter<T>(_t: T) -> Self
    where
        T: IntoIterator,
    {
        todo!()
    }
}

impl<S> Iterator for NeighsIterator<S>
where
    S: SRDF,
{
    type Item = Neigh<S>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

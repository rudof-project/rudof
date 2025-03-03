use std::{collections::HashSet, vec::IntoIter};

use crate::{Query, Triple};

pub enum Neigh<Q: Query> {
    Direct { p: Q::IRI, o: Q::Term },
    Inverse { s: Q::Subject, p: Q::IRI },
}

impl<Q: Query> Neigh<Q> {
    pub fn direct(pred: Q::IRI, object: Q::Term) -> Neigh<Q> {
        Neigh::Direct { p: pred, o: object }
    }

    pub fn inverse(pred: Q::IRI, subject: Q::Subject) -> Neigh<Q> {
        Neigh::Inverse {
            p: pred,
            s: subject,
        }
    }
}

// TODO...the following code is just a draft...
// I would like to generate the neighs as an iterator...
pub struct NeighsIterator<Q: Query> {
    _term: Q::Term,
    _neigh_iter: IntoIter<Neigh<Q>>,
}

impl<Q: Query> NeighsIterator<Q> {
    pub fn new(term: Q::Term, rdf: Q) -> Result<NeighsIterator<Q>, Q::Err> {
        match term.try_into() {
            Ok(subject) => {
                let subject: Q::Subject = subject;
                let preds: HashSet<Q::IRI> = rdf
                    .triples_with_subject(subject)?
                    .map(Triple::into_predicate)
                    .collect();
                let _qs = preds.into_iter();
                /*let vv = qs.flat_map(|p| {
                    let objs = rdf.get_objects_for_subject_predicate(&subject, &p)?;
                    objs.into_iter().map(|o| Neigh::Direct { p, o })
                });*/
                todo!(); // Ok(vv)
            }
            Err(_) => {
                todo!()
            }
        }
        // NeighsIterator { term, objectsIter }
    }
}

impl<Q: Query> FromIterator<Neigh<Q>> for NeighsIterator<Q> {
    fn from_iter<T>(_t: T) -> Self
    where
        T: IntoIterator,
    {
        todo!()
    }
}

impl<Q: Query> Iterator for NeighsIterator<Q> {
    type Item = Neigh<Q>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

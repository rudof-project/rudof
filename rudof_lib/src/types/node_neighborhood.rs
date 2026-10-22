use crate::{Result, errors::DataError};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::{ArcDirection, NeighArc, NeighsIterator, Rdf, term::Object};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sparql_service::RdfData;
use std::{iter::FusedIterator, vec::IntoIter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct NeighborArc {
    /// The node the neighborhood expansion started from.
    pub root: Object,
    /// Whether the arc leaves or enters [`Self::node`].
    pub direction: ArcDirection,
    /// How many arcs away from [`Self::root`] this arc is, starting at 1.
    pub depth: usize,
    /// The node this arc starts from.
    pub node: Object,
    /// The predicate IRI of the arc.
    #[schemars(with = "String")]
    pub predicate: IriS,
    /// The node at the other end of the arc.
    pub neighbor: Object,
    /// `true` for the last arc of [`Self::node`] in this direction.
    pub is_last: bool,
}

pub struct NodeNeighborhood<'a> {
    rdf: &'a RdfData,
    roots: IntoIter<<RdfData as Rdf>::Term>,
    predicates: Vec<<RdfData as Rdf>::IRI>,
    directions: &'static [ArcDirection],
    max_depth: usize,
    current: Option<(Object, NeighsIterator<'a, RdfData>)>,
    finished: bool,
}

impl<'a> NodeNeighborhood<'a> {
    pub(crate) fn new(
        rdf: &'a RdfData,
        roots: Vec<<RdfData as Rdf>::Term>,
        predicates: Vec<<RdfData as Rdf>::IRI>,
        directions: &'static [ArcDirection],
        max_depth: usize,
    ) -> Self {
        Self {
            rdf,
            roots: roots.into_iter(),
            predicates,
            directions,
            max_depth,
            current: None,
            finished: false,
        }
    }

    fn fail(&mut self, error: DataError) -> Option<Result<NeighborArc>> {
        self.finished = true;
        self.current = None;
        Some(Err(Box::new(error).into()))
    }
}

impl Iterator for NodeNeighborhood<'_> {
    type Item = Result<NeighborArc>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished {
            if let Some((root, neighs_iterator)) = self.current.as_mut() {
                match neighs_iterator.next() {
                    Some(Ok(arc)) => {
                        let root = root.clone();
                        return match to_neighbor_arc(root, arc) {
                            Ok(arc) => Some(Ok(arc)),
                            Err(e) => self.fail(e),
                        };
                    },
                    Some(Err(e)) => return self.fail(DataError::FailedArcRetrieval { error: e.to_string() }),
                    None => self.current = None,
                }
                continue;
            }

            let Some(root) = self.roots.next() else {
                self.finished = true;
                break;
            };
            let root_object = match RdfData::term_as_object(&root) {
                Ok(object) => object,
                Err(e) => return self.fail(DataError::FailedQualification { error: e.to_string() }),
            };
            let neighs = NeighsIterator::new(self.rdf, root)
                .with_directions(self.directions)
                .with_predicates(self.predicates.clone())
                .with_max_depth(self.max_depth);
            self.current = Some((root_object, neighs));
        }
        None
    }
}

impl FusedIterator for NodeNeighborhood<'_> {}

fn to_neighbor_arc(root: Object, arc: NeighArc<RdfData>) -> std::result::Result<NeighborArc, DataError> {
    let (direction, predicate, neighbor) = arc.neigh.into_parts();
    Ok(NeighborArc {
        root,
        direction,
        depth: arc.depth,
        node: RdfData::term_as_object(&arc.node)
            .map_err(|e| DataError::FailedQualification { error: e.to_string() })?,
        predicate: predicate.into(),
        neighbor: RdfData::term_as_object(&neighbor)
            .map_err(|e| DataError::FailedQualification { error: e.to_string() })?,
        is_last: arc.is_last,
    })
}

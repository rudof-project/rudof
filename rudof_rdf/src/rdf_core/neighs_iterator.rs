use crate::rdf_core::NeighsRDF;
use std::{collections::VecDeque, fmt, iter::FusedIterator, vec::IntoIter};

/// Direction in wich an arc is followed from the node being expanded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArcDirection {
    Incoming,
    Outgoing
}

/// Represents a single neighborhood relationship in an RDF graph.
///
/// A neighborhood relationship can be either a direct connection (outgoing arc)
/// or an inverse connection (incoming arc) relative to a focus node.
///
/// # Type Parameters
///
/// * `S` - The RDF graph type implementing [`NeighsRDF`]
///
/// # Variants
///
/// - [`Direct`](Self::Direct): An outgoing relationship where the focus node is the subject
/// - [`Inverse`](Self::Inverse): An incoming relationship where the focus node is the object
pub enum Neigh<S>
where
    S: NeighsRDF,
{
    /// A direct (outgoing) relationship from the focus node.
    ///
    /// Represents a triple pattern: `(focusNode, p, o)` where the focus node
    /// is the subject, `p` is the predicate, and `o` is the object.
    Direct { p: S::IRI, o: S::Term },
    /// An inverse (incoming) relationship to the focus node.
    ///
    /// Represents a triple pattern: `(s, p, focusNode)` where `s` is the subject,
    /// `p` is the predicate, and the focus node is the object.
    Inverse { s: S::Subject, p: S::IRI },
}

impl<S> Neigh<S>
where
    S: NeighsRDF,
{
    /// Creates a direct (outgoing) neighborhood relationship.
    pub fn direct(pred: S::IRI, object: S::Term) -> Neigh<S> {
        Neigh::Direct { p: pred, o: object }
    }

    /// Creates an inverse (incoming) neighborhood relationship.
    pub fn inverse(pred: S::IRI, subject: S::Subject) -> Neigh<S> {
        Neigh::Inverse { p: pred, s: subject }
    }

    /// Returns the direction of the neighborhood relationship.
    pub fn direction(&self) -> ArcDirection {
        match self {
            Neigh::Direct { .. } => ArcDirection::Outgoing,
            Neigh::Inverse { .. } => ArcDirection::Incoming
        }
    }

    /// Returns the predicate of the relationship.
    pub fn predicate(&self) -> &S::IRI {
        match self {
            Neigh::Direct { p, .. } | Neigh::Inverse { p, .. } => p
        }
    }

    /// Returns the node at the other end of the relationship:
    /// the object for direct relationships and the subject for inverse relationships.
    pub fn neighbor(&self) -> S::Term {
        match self {
            Neigh::Direct{o, ..} => o.clone(),
            Neigh::Inverse{s, ..} => S::subject_as_term(s)
        }
    }

    /// Consumes the relationship returning its direction, predicate, and neighbor
    pub fn into_parts(self) -> (ArcDirection, S::IRI, S::Term) {
        match self {
            Neigh::Direct {o, p} => (ArcDirection::Outgoing, p, o),
            Neigh::Inverse { s, p } => (ArcDirection::Incoming, p, s.into())
        }
    }
}

impl<S: NeighsRDF> Clone for Neigh<S> {
    fn clone(&self) -> Self {
        match self {
            Neigh::Direct { p, o } => Neigh::Direct {
                p: p.clone(),
                o: o.clone(),
            },
            Neigh::Inverse { s, p } => Neigh::Inverse {
                s: s.clone(),
                p: p.clone(),
            }
        }
    }
}

impl<S: NeighsRDF> fmt::Debug for Neigh<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Neigh::Direct {p, o} => f.debug_struct("Direct").field("p", p).field("o", o).finish(),
            Neigh::Inverse { s, p } => f.debug_struct("Inverse").field("s", s).field("p", p).finish()
        }
    }
}

impl<S: NeighsRDF> PartialEq for Neigh<S> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Neigh::Direct {p: p1, o: o1}, Neigh::Direct {p: p2, o: o2}) => p1 == p2 && o1 == o2,
            (Neigh::Inverse { s: s1, p: p1 }, Neigh::Inverse { s: s2, p: p2 }) => s1 == s2 && p1 == p2,
            _ => false,
        }
    }
}

impl<S: NeighsRDF> Eq for Neigh<S> {}

/// One arc yielded by [`NeighsIterator`].
pub struct NeighArc<S: NeighsRDF> {
    pub depth: usize,
    pub node: S::Term,
    pub neigh: Neigh<S>
}

impl<S: NeighsRDF> Clone for NeighArc<S> {
    fn clone(&self) -> Self {
        NeighArc {
            depth: self.depth,
            node: self.node.clone(),
            neigh: self.neigh.clone()
        }
    }
}

impl<S: NeighsRDF> fmt::Debug for NeighArc<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NeighArc")
            .field("depth", &self.depth)
            .field("node", &self.node)
            .field("neigh", &self.neigh)
            .finish()
    }
}

impl<S:NeighsRDF> PartialEq for NeighArc<S> {
    fn eq(&self, other: &Self) -> bool {
        &self.depth == &other.depth && &self.node == &other.node && &self.neigh == &other.neigh
    }
}

impl<S:NeighsRDF> Eq for NeighArc<S> {}

/// Lazy, depth-first iterator over the neighborhood of a node in an RDF graph.
pub struct NeighsIterator<'a, S: NeighsRDF> {
    rdf: &'a S,
    root: S::Term,
    /// Filtering predicates for the arcs to be yielded. If empty, all predicates are allowed.
    predicates: Vec<S::IRI>,
    max_depth: usize,
    /// Directions whose tree has not been started yet
    directions: VecDeque<ArcDirection>,
    /// Nodes whose arcs were fetched but not all yielded yet
    stack: Vec<Frame<S>>,
    /// Neighbor of the last yielded arc, expanded on the next call to `next`
    pending: Option<Pending<S>>,
    finished: bool
}

struct Frame<S: NeighsRDF> {
    node: S::Term,
    depth: usize,
    arcs: IntoIter<Neigh<S>>
}

struct Pending<S: NeighsRDF> {
    node: S::Term,
    depth: usize,
    direction: ArcDirection,
}

impl<'a, S: NeighsRDF> NeighsIterator<'a, S> {
    pub fn new(rdf: &'a S, root: S::Term) -> Self {
        NeighsIterator {
            rdf,
            root,
            predicates: Vec::new(),
            max_depth: 1,
            directions: VecDeque::from(vec![ArcDirection::Outgoing, ArcDirection::Incoming]),
            stack: Vec::new(),
            pending: None,
            finished: false
        }
    }

    /// Sets the directions to traverse, in order.
    pub fn with_directions(mut self, directions: &[ArcDirection]) -> Self {
        self.directions = directions.iter().copied().collect();
        self
    }

    /// Sets the filter of predicates to traverse.
    pub fn with_predicates(mut self, predicates: Vec<S::IRI>) -> Self {
        self.predicates = predicates;
        self
    }

    /// Sets the maximum depth to traverse.
    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn root(&self) -> &S::Term {
        &self.root
    }

    /// Fetches the arcs of `node` in `direction`, sorted by predicate and neighbor
    fn fetch_arcs(&self, node: &S::Term, direction: ArcDirection) -> Result<Vec<Neigh<S>>, S::Err> {
        let mut arcs: Vec<Neigh<S>> = match direction {
            ArcDirection::Outgoing => {
                let Ok(subject) = S::term_as_subject(node) else {
                    return Ok(Vec::new());
                };
                let arcs = if self.predicates.is_empty() {
                    self.rdf.outgoing_arcs(&subject)?
                } else {
                    self.rdf.outgoing_arcs_from_list(&subject, &self.predicates)?.0
                };
                arcs.into_iter()
                    .flat_map(|(p, objects)| objects.into_iter().map(move |o| Neigh::direct(p.clone(), o)))
                    .collect()
            },
            ArcDirection::Incoming => {
                let arcs = if self.predicates.is_empty() {
                    self.rdf.incoming_arcs(node)?
                } else {
                    self.rdf.incoming_arcs_from_list(node, &self.predicates)?
                };
                arcs.into_iter()
                    .flat_map(|(p, subjects)| subjects.into_iter().map(move |s| Neigh::inverse(p.clone(), s)))
                    .collect()
            }
        };
        arcs.sort_by_cached_key(|neigh| (neigh.predicate().clone(), neigh.neighbor().to_string()));
        Ok(arcs) 
    }

    fn finish(&mut self) {
        self.finished = true;
        self.stack.clear();
        self.pending = None;
    }
}

impl <S: NeighsRDF> Iterator for NeighsIterator<'_, S> {
    type Item = Result<NeighArc<S>, S::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished {
            // 1. Expand the neighbor of the previously yielded arc
            if let Some(Pending { node, depth, direction}) = self.pending.take() {
                if self.stack.iter().any(|f| f.node == node) {
                    continue;
                }
                match self.fetch_arcs(&node, direction) {
                    Ok(arcs) => self.stack.push(Frame {
                        node,
                        depth,
                        arcs: arcs.into_iter()
                    }),
                    Err(e) => {
                        self.finish();
                        return Some(Err(e));
                    }
                }
                continue;
            }

            // 2. Yield the next arc of the deepest node that still has arcs
            if let Some(frame) = self.stack.last_mut() {
                let Some(neigh) = frame.arcs.next() else {
                    self.stack.pop();
                    continue;
                };
                let depth = frame.depth;
                let node = frame.node.clone();
                if depth < self.max_depth {
                    self.pending = Some(Pending {
                        node: neigh.neighbor(),
                        depth: depth + 1,
                        direction: neigh.direction()
                    });
                }
                return Some(Ok(NeighArc {depth, node, neigh}));
            }

            // 3. The current tree is exhausted: start the new direction from the root
            match self.directions.pop_front() {
                Some(direction) if self.max_depth > 0 => {
                    self.pending = Some(Pending {
                        node: self.root.clone(),
                        depth: 1,
                        direction
                    })
                },
                _ => self.finish(),
            }
        }
        None
    }
}

impl<S: NeighsRDF>  FusedIterator for NeighsIterator<'_, S> {}
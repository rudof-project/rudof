//! The valid and invalid subgraphs of a SHACL validation.
//!
//! While a [`SubgraphTracker`] is installed, the store's [`ReadRecorder`] hands
//! it every triple the native engine reads, and the engine tells it which
//! `(focus node, shape)` pair it is validating. Each triple read is then
//! attributed to the innermost pair being validated when it was read, and each
//! pair remembers the pairs it relied on (its property shapes, `sh:node`,
//! `sh:and`, `sh:or`... references) and whether it conforms.
//!
//! Once the validation ends:
//!
//! - the **valid subgraph** has the triples read while validating the
//!   conformant pairs of the shapes with targets, following the conformant
//!   pairs they rely on, transitively. A pair that turned out not to conform
//!   contributes nothing, and neither do the pairs only it relies on. A triple
//!   read while selecting the focus nodes of a shape (e.g. the `rdf:type`
//!   triple behind `sh:targetClass`) is valid when it mentions a focus node
//!   that conforms to that shape.
//! - the **invalid subgraph** has the triples read that are not in the valid
//!   one. The two split the triples read: a triple read while validating a
//!   non-conformant pair but also read by a conformant one is valid.
//!
//! To attribute every read to one pair, a validation with a tracker validates
//! the focus nodes one by one, and the shapes of a level one after the other
//! instead of in parallel.
//!
//! [`ReadRecorder`]: crate::validator::store::ReadRecorder

use crate::error::ValidationError;
use crate::ir::ShapeLabelIdx;
use indexmap::{IndexMap, IndexSet};
use oxrdf::Triple as OxTriple;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf_sink::TriplesSink;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard};

/// A `(focus node, shape)` pair.
pub type Pair = (Object, ShapeLabelIdx);

/// See the [module documentation](self). Cloning it shares the same state.
#[derive(Clone, Debug, Default)]
pub struct SubgraphTracker(Arc<Mutex<TrackerState>>);

#[derive(Debug, Default)]
struct TrackerState {
    /// Every distinct triple read, in reading order.
    read: IndexSet<OxTriple>,
    /// The pairs being validated, innermost last.
    stack: Vec<Frame>,
    /// The shape whose focus nodes are being selected, if any.
    selecting: Option<ShapeLabelIdx>,
    /// The triples read while selecting the focus nodes of each shape.
    target_reads: IndexMap<ShapeLabelIdx, IndexSet<OxTriple>>,
    /// The pairs validated at the top level, for the shapes with targets.
    top: IndexSet<Pair>,
    /// What is known about each pair validated.
    pairs: IndexMap<Pair, PairInfo>,
}

#[derive(Debug)]
struct Frame {
    pair: Pair,
    reads: IndexSet<OxTriple>,
    children: IndexSet<Pair>,
}

#[derive(Debug, Default)]
struct PairInfo {
    conforms: bool,
    reads: IndexSet<OxTriple>,
    children: IndexSet<Pair>,
}

/// The valid and invalid subgraphs, see the [module documentation](self).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Subgraphs {
    pub valid: Vec<OxTriple>,
    pub invalid: Vec<OxTriple>,
}

impl SubgraphTracker {
    pub fn new() -> Self {
        Self::default()
    }

    fn state(&self) -> MutexGuard<'_, TrackerState> {
        // A poisoned lock only means another thread panicked: the state is still usable
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Records that the validator read `triples`.
    pub fn record_read<'a>(&self, triples: impl IntoIterator<Item = &'a OxTriple>) {
        let mut state = self.state();
        let state = &mut *state;
        for triple in triples {
            state.read.insert(triple.clone());
            if let Some(frame) = state.stack.last_mut() {
                frame.reads.insert(triple.clone());
            } else if let Some(shape) = state.selecting {
                state.target_reads.entry(shape).or_default().insert(triple.clone());
            }
        }
    }

    /// Whether no pair is being validated: a pair entered now is a top-level one.
    pub fn at_top_level(&self) -> bool {
        self.state().stack.is_empty()
    }

    /// Starts selecting the focus nodes of `shape`: the triples read until
    /// [`Self::end_targets`] are the ones behind its targets.
    pub fn begin_targets(&self, shape: ShapeLabelIdx) {
        self.state().selecting = Some(shape);
    }

    pub fn end_targets(&self) {
        self.state().selecting = None;
    }

    /// Starts validating `pair`, which the pair being validated, if any, relies on.
    pub fn enter(&self, pair: Pair) {
        let mut state = self.state();
        match state.stack.last_mut() {
            Some(parent) => {
                parent.children.insert(pair.clone());
            },
            None => {
                state.top.insert(pair.clone());
            },
        }
        state.stack.push(Frame {
            pair,
            reads: IndexSet::new(),
            children: IndexSet::new(),
        });
    }

    /// Ends validating the innermost pair being validated, which `conforms` or not.
    pub fn exit(&self, conforms: bool) {
        let mut state = self.state();
        let Some(frame) = state.stack.pop() else {
            return;
        };
        // A pair may be entered again: from the cache, or as a recursive
        // reference cut further down its own validation. The outermost exit,
        // the last one, is the one that says whether it conforms.
        let info = state.pairs.entry(frame.pair).or_default();
        info.conforms = conforms;
        info.reads.extend(frame.reads);
        info.children.extend(frame.children);
    }

    /// Leaves every pair still being validated, after an error.
    pub fn reset_stack(&self) {
        let mut state = self.state();
        state.stack.clear();
        state.selecting = None;
    }

    /// Whether `pair` was validated and conforms.
    fn conforms(state: &TrackerState, pair: &Pair) -> bool {
        state.pairs.get(pair).is_some_and(|info| info.conforms)
    }

    /// The valid and invalid subgraphs of the validation so far.
    pub fn subgraphs(&self) -> Subgraphs {
        let state = self.state();
        let mut valid: IndexSet<OxTriple> = IndexSet::new();

        // The triples behind the targets of a conformant focus node
        for (shape, triples) in &state.target_reads {
            let conformant: HashSet<oxrdf::Term> = state
                .top
                .iter()
                .filter(|pair| pair.1 == *shape && Self::conforms(&state, pair))
                .map(|(node, _)| node.clone().into())
                .collect();
            for triple in triples {
                let subject: oxrdf::Term = triple.subject.clone().into();
                if conformant.contains(&subject) || conformant.contains(&triple.object) {
                    valid.insert(triple.clone());
                }
            }
        }

        // The triples read by the conformant pairs, and by the conformant pairs they rely on
        let mut queue: Vec<&Pair> = state
            .top
            .iter()
            .filter(|pair| Self::conforms(&state, pair))
            .rev()
            .collect();
        let mut visited: HashSet<&Pair> = HashSet::new();
        while let Some(pair) = queue.pop() {
            if !visited.insert(pair) {
                continue;
            }
            let Some(info) = state.pairs.get(pair) else {
                continue;
            };
            valid.extend(info.reads.iter().cloned());
            queue.extend(
                info.children
                    .iter()
                    .rev()
                    .filter(|child| !visited.contains(child) && Self::conforms(&state, child)),
            );
        }

        // In reading order, so the subgraphs come out the same on every run
        let mut subgraphs = Subgraphs::default();
        for triple in &state.read {
            if valid.contains(triple) {
                subgraphs.valid.push(triple.clone());
            } else {
                subgraphs.invalid.push(triple.clone());
            }
        }
        subgraphs
    }

    /// Pushes the valid subgraph into `valid_sink` and the invalid one into
    /// `invalid_sink`, for those given.
    pub fn push_subgraphs(
        &self,
        valid_sink: Option<&TriplesSink>,
        invalid_sink: Option<&TriplesSink>,
    ) -> Result<(), ValidationError> {
        let subgraphs = self.subgraphs();
        for (sink, triples) in [(valid_sink, subgraphs.valid), (invalid_sink, subgraphs.invalid)] {
            let Some(sink) = sink else {
                continue;
            };
            let mut sink = sink
                .0
                .lock()
                .map_err(|e| ValidationError::SubgraphSinkFailed(e.to_string()))?;
            for triple in triples {
                sink.push(triple)
                    .map_err(|e| ValidationError::SubgraphSinkFailed(e.to_string()))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{Literal, NamedNode};

    fn ex(local: &str) -> NamedNode {
        NamedNode::new_unchecked(format!("http://example.org/{local}"))
    }

    fn t(s: &str, p: &str, o: &str) -> OxTriple {
        OxTriple::new(ex(s), ex(p), Literal::new_simple_literal(o))
    }

    fn pair(node: &str, shape: usize) -> Pair {
        (
            Object::iri(rudof_iri::IriS::new_unchecked(ex(node).as_str())),
            ShapeLabelIdx::from(shape),
        )
    }

    #[test]
    fn splits_the_reads_by_the_conformance_of_their_pairs() {
        let tracker = SubgraphTracker::new();
        tracker.enter(pair("alice", 0));
        tracker.record_read([&t("alice", "name", "Alice")]);
        tracker.exit(true);
        tracker.enter(pair("bob", 0));
        tracker.record_read([&t("bob", "name", "Bob"), &t("alice", "name", "Alice")]);
        tracker.exit(false);

        let subgraphs = tracker.subgraphs();
        assert_eq!(subgraphs.valid, vec![t("alice", "name", "Alice")]);
        assert_eq!(subgraphs.invalid, vec![t("bob", "name", "Bob")]);
    }

    #[test]
    fn follows_only_the_conformant_pairs_relied_on() {
        let tracker = SubgraphTracker::new();
        tracker.enter(pair("alice", 0));
        tracker.record_read([&t("alice", "knows", "x")]);
        tracker.enter(pair("bob", 1));
        tracker.record_read([&t("bob", "age", "1")]);
        tracker.exit(true);
        tracker.enter(pair("carol", 2));
        tracker.record_read([&t("carol", "age", "x")]);
        tracker.exit(false);
        tracker.exit(true);

        let subgraphs = tracker.subgraphs();
        assert_eq!(subgraphs.valid, vec![t("alice", "knows", "x"), t("bob", "age", "1")]);
        assert_eq!(subgraphs.invalid, vec![t("carol", "age", "x")]);
    }

    #[test]
    fn target_reads_are_valid_for_conformant_focus_nodes() {
        let tracker = SubgraphTracker::new();
        let shape = ShapeLabelIdx::from(0);
        tracker.begin_targets(shape);
        tracker.record_read([&t("alice", "type", "Person"), &t("bob", "type", "Person")]);
        tracker.end_targets();
        tracker.enter(pair("alice", 0));
        tracker.exit(true);
        tracker.enter(pair("bob", 0));
        tracker.exit(false);

        let subgraphs = tracker.subgraphs();
        assert_eq!(subgraphs.valid, vec![t("alice", "type", "Person")]);
        assert_eq!(subgraphs.invalid, vec![t("bob", "type", "Person")]);
    }
}

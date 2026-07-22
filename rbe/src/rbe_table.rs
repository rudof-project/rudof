use crate::Bag;
use crate::Cardinality;
use crate::Component;
use crate::Context;
use crate::EmptyIter;
use crate::Key;
use crate::MatchCond;
use crate::Pending;
use crate::RbeError;
use crate::Ref;
use crate::Value;
use crate::deriv_error::DerivError;
use crate::match_cond::MatchKind;
use crate::rbe_error;
use crate::rbe_struct::RbeStruct;
use crate::values::Values;
use core::hash::Hash;
use indexmap::IndexMap;
use indexmap::IndexSet;
use itertools::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::vec::IntoIter;
// use tracing::trace;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RbeTable<K, V, R, Ctx, P = ()>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    // A regular bag expression of components
    rbe: RbeStruct<Component>,

    // Each key is associated with a set of components
    key_components: IndexMap<K, IndexSet<Component>>,

    // TODO: Unify in a single table component_cond and component_key
    component_cond: IndexMap<Component, MatchCond<K, V, R, Ctx, P>>,
    component_key: HashMap<Component, K>,

    // Indicates if the RBE is open or closed
    open: bool,

    // Counter for the number of components
    component_counter: usize,
}

impl<K, V, R, Ctx, P> Default for RbeTable<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn default() -> Self {
        Self {
            rbe: RbeStruct::default(),
            key_components: IndexMap::new(),
            component_cond: IndexMap::new(),
            component_key: HashMap::new(),
            open: false,
            component_counter: 0,
        }
    }
}

impl<K, V, R, Ctx, P> RbeTable<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    pub fn new() -> RbeTable<K, V, R, Ctx, P> {
        RbeTable::default()
    }

    pub fn get_condition(&self, c: &Component) -> Option<&MatchCond<K, V, R, Ctx, P>> {
        self.component_cond.get(c)
    }

    pub fn get_key(&self, c: &Component) -> Option<&K> {
        self.component_key.get(c)
    }

    /// Translates a cardinality-matching error against `Component` ids (as
    /// produced internally by `RbeStruct<Component>`) into `(key, expected
    /// cardinality, actual count)` triples naming the real key (e.g.
    /// predicate) instead of the opaque id, so a caller with more context
    /// (e.g. a prefix map to qualify the key for display) can render it.
    /// `Err` is returned for errors that can't be attributed to a single
    /// key's cardinality (e.g. `Or`-branch interactions); it carries the
    /// original error's `Display` text as a best-effort fallback.
    /// Only called once matching has already failed, so the extra lookups
    /// here don't cost anything on the success path.
    pub fn cardinality_violations(&self, err: &DerivError<Component>) -> Result<Vec<(K, Cardinality, usize)>, String> {
        match err {
            DerivError::CardinalityFail {
                symbol,
                expected_cardinality,
                current_number,
            } => match self.get_key(symbol) {
                Some(key) => Ok(vec![(key.clone(), expected_cardinality.clone(), *current_number)]),
                None => Err(err.to_string()),
            },
            DerivError::CardinalityFailMulti { failures } => {
                let violations: Vec<_> = failures
                    .iter()
                    .filter_map(|(_, e)| self.cardinality_violations(e).ok())
                    .flatten()
                    .collect();
                if violations.is_empty() {
                    Err(err.to_string())
                } else {
                    Ok(violations)
                }
            },
            other => Err(other.to_string()),
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.key_components.keys()
    }

    pub fn add_component(&mut self, k: K, cond: &MatchCond<K, V, R, Ctx, P>) -> Component {
        let c = Component::from(self.component_counter);
        let key = k.clone();
        self.key_components
            .entry(k)
            .and_modify(|vs| {
                (*vs).insert(c);
            })
            .or_insert_with(|| {
                let mut hs = IndexSet::new();
                hs.insert(c);
                hs
            });
        self.component_cond.insert(c, cond.clone());
        self.component_key.insert(c, key);
        self.component_counter += 1;
        c
    }

    pub fn with_rbe(&mut self, rbe: RbeStruct<Component>) {
        self.rbe = rbe;
    }

    /// Sound refutation test: returns `false` only if no assignment of the given values to
    /// this table's components can satisfy the regular bag expression, judged from the
    /// per-component candidate counts (values whose key and condition match). Values may be
    /// a superset of what the expression will actually receive (the counts over-approximate),
    /// which keeps refutation sound. `true` is not a guarantee; use [`Self::matches`] to decide.
    /// Linear in the number of values times components per key, plus the expression size.
    pub fn feasible_neighs(&self, values: &[(K, V, Ctx)]) -> bool {
        let mut hi: HashMap<Component, usize> = HashMap::new();
        for (key, value, ctx) in values {
            if let Some(components) = self.key_components.get(key) {
                for component in components {
                    if let Some(cond) = self.component_cond.get(component)
                        && cond.matches(value, ctx).is_ok()
                    {
                        *hi.entry(*component).or_insert(0) += 1;
                    }
                }
            }
        }
        let lo: HashMap<Component, usize> = HashMap::new();
        self.rbe.feasible(&lo, &hi)
    }

    pub fn matches(
        &self,
        values: Vec<(K, V, Ctx)>,
    ) -> Result<MatchTableIter<K, V, R, Ctx, P>, RbeError<K, V, R, Ctx, P>> {
        /*tracing::trace!(
            "Checking if RbeTable {} matches [{}]",
            &self,
            values.iter().map(|(k, v, _ctx)| format!("({k} {v})")).join(", ")
        );*/
        let mut pairs_found = 0;
        let mut candidates = Vec::new();
        let cs_empty = IndexSet::new();
        for (key, value, ctx) in &values {
            let components = self.key_components.get(key).unwrap_or(&cs_empty);
            let mut pairs = Vec::new();
            if components.len() > 1 {
                // Multiple components for this key: pre-filter by checking
                // which components can actually accept this value, to avoid
                // generating invalid cartesian product combinations.
                let mut last_err = None;
                for component in components {
                    let cond = self.component_cond.get(component).unwrap();
                    match cond.matches(value, ctx) {
                        Ok(_) => {
                            pairs_found += 1;
                            pairs.push((key.clone(), value.clone(), ctx.clone(), *component, cond.clone()));
                        },
                        Err(err) => {
                            // trace!("Pre-filter: condition {cond} rejected value {value} for component {component}");
                            last_err = Some(err);
                        },
                    }
                }
                if pairs.is_empty()
                    && let Some(err) = last_err
                {
                    return Err(err);
                }
            } else {
                for component in components {
                    let cond = self.component_cond.get(component).unwrap();
                    pairs_found += 1;
                    pairs.push((key.clone(), value.clone(), ctx.clone(), *component, cond.clone()));
                }
            }
            candidates.push(pairs);
        }

        if candidates.is_empty() || pairs_found == 0 {
            /*tracing::trace!(
                "No candidates for rbe: {}, candidates: {:?}, pairs_found: {pairs_found}",
                self.rbe,
                candidates,
            );*/
            if self.rbe.nullable() {
                //trace!("Rbe is nullable and no candidates...should be sucessful");

                Ok(MatchTableIter::Empty(EmptyIter::new(
                    &self.rbe,
                    self,
                    &Values::from(&values),
                )))
            } else {
                let result = Ok(MatchTableIter::Empty(EmptyIter::new(
                    &self.rbe,
                    self,
                    &Values::from(&values),
                )));
                //tracing::trace!("Result of matches: {:?}", result);
                result
            }
        } else {
            /*tracing::trace!(
                "Some candidates found for rbe: {}\nCandidates:\n{}",
                self.rbe,
                candidates
                    .iter()
                    .enumerate()
                    .map(|(i, c)| format!("[Candidate {i}: {}]", show_candidate(c)))
                    .join("\n")
            );*/
            let mp = candidates.into_iter().multi_cartesian_product();
            Ok(MatchTableIter::NonEmpty(IterCartesianProduct {
                is_first: true,
                state: mp,
                rbe: self.rbe.clone(),
                open: self.open,
                failed: Vec::new(),
                failed_cardinality: Vec::new(),
            }))
        }
    }

    pub fn components(&self) -> ComponentsIter<'_, K, V, R, Ctx, P> {
        ComponentsIter {
            current: 0,
            table: self,
        }
    }

    pub fn find_cond(&self, key: &K) -> Option<&MatchCond<K, V, R, Ctx, P>> {
        self.key_components.get(key).and_then(|cs| {
            if let Some(c) = cs.iter().next() {
                self.component_cond.get(c)
            } else {
                None
            }
        })
    }

    pub fn show_rbe_table<SK, SV>(&self, show_key: SK, _show_value: SV, width: usize) -> String
    where
        SK: Fn(&K) -> String,
        SV: Fn(&V) -> String,
    {
        let rbe_str = self.rbe.inner_rbe().map(&|c| {
            let key = self.component_key.get(c).unwrap();
            let cond = self.component_cond.get(c).unwrap();
            format!("{} {}", show_key(key), cond.show())
        });
        rbe_str.pretty(width)
    }

    pub fn show_rbe_simplified(&self) -> String {
        self.key_components
            .keys()
            .map(|k| {
                let cond = self.find_cond(k).unwrap();
                format!("{} {}", k, cond)
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub struct ComponentsIter<'a, K, V, R, Ctx, P = ()>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    current: usize,
    table: &'a RbeTable<K, V, R, Ctx, P>,
}

impl<K, V, R, Ctx, P> Iterator for ComponentsIter<'_, K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    type Item = (Component, K, MatchCond<K, V, R, Ctx, P>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.table.component_counter {
            let c = Component::from(self.current);
            let cond = self.table.component_cond.get(&c).unwrap().clone();
            let key = self.table.component_key.get(&c).unwrap().clone();
            self.current += 1;
            Some((c, key, cond))
        } else {
            None
        }
    }
}

impl<K, V, R, Ctx, P> Debug for ComponentsIter<'_, K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentsIter")
            .field("current", &self.current)
            .field("table", &self.table)
            .finish()
    }
}

impl<K, V, R, Ctx, P> Debug for RbeTable<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RbeTable")
            .field("rbe", &self.rbe)
            .field("key_components", &self.key_components)
            .field("component_cond", &self.component_cond)
            .field("component_key", &self.component_key)
            .field("open", &self.open)
            .field("component_counter", &self.component_counter)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum MatchTableIter<K, V, R, Ctx, P = ()>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    Empty(EmptyIter<K, V, R, Ctx, P>),
    NonEmpty(IterCartesianProduct<K, V, R, Ctx, P>),
}

impl<K, V, R, Ctx, P> Iterator for MatchTableIter<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    type Item = Result<Pending<K, V, R>, rbe_error::RbeError<K, V, R, Ctx, P>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MatchTableIter::Empty(e) => e.next(),
            MatchTableIter::NonEmpty(cp) => cp.next(),
        }
    }
}

impl<K, V, R, Ctx, P> MatchTableIter<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    /// Candidates rejected because a value failed a key's own condition
    /// (e.g. a node constraint or shape reference), kept as structured
    /// `(candidate, key, value, error)` tuples — using the real `K`/`V`
    /// types, not opaque `Component` ids — so a caller with more display
    /// context (e.g. a prefix map) can render them. Only `IterCartesianProduct`
    /// accumulates these; once the iterator is exhausted (`next()` returned
    /// `None`), this holds an entry for every candidate that was tried.
    #[allow(clippy::type_complexity)]
    pub fn failed_candidates(&self) -> &[(Vec<(K, V)>, K, V, RbeError<K, V, R, Ctx, P>)] {
        match self {
            MatchTableIter::Empty(_) => &[],
            MatchTableIter::NonEmpty(cp) => cp.failed_candidates(),
        }
    }

    /// Candidates rejected specifically because their cardinality didn't
    /// satisfy the expression, kept as structured `(candidate, error)` pairs.
    /// The error still refers to opaque `Component` ids: the caller, which
    /// knows how to map a `Component` back to the real key via
    /// [`RbeTable::cardinality_violations`], must do the final translation.
    pub fn failed_cardinality(&self) -> &[(Vec<(K, V)>, DerivError<Component>)] {
        match self {
            MatchTableIter::Empty(_) => &[],
            MatchTableIter::NonEmpty(cp) => cp.failed_cardinality(),
        }
    }
}

type IterState<K, V, R, Ctx, P> = MultiProduct<IntoIter<(K, V, Ctx, Component, MatchCond<K, V, R, Ctx, P>)>>;

#[derive(Debug, Clone)]
pub struct IterCartesianProduct<K, V, R, Ctx, P = ()>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    is_first: bool,
    state: IterState<K, V, R, Ctx, P>,
    rbe: RbeStruct<Component>,
    open: bool,
    // Candidates previously rejected by a per-value condition failure.
    // Grows as the iterator is driven; complete once it returns `None`.
    #[allow(clippy::type_complexity)]
    failed: Vec<(Vec<(K, V)>, K, V, RbeError<K, V, R, Ctx, P>)>,
    // Candidates rejected because their cardinality didn't satisfy the
    // expression, kept structured (candidate, raw error over `Component`
    // ids) since translating `Component` back to the real key requires the
    // `RbeTable`, which this iterator doesn't hold onto.
    failed_cardinality: Vec<(Vec<(K, V)>, DerivError<Component>)>,
}

impl<K, V, R, Ctx, P> IterCartesianProduct<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    #[allow(clippy::type_complexity)]
    pub fn failed_candidates(&self) -> &[(Vec<(K, V)>, K, V, RbeError<K, V, R, Ctx, P>)] {
        &self.failed
    }

    pub fn failed_cardinality(&self) -> &[(Vec<(K, V)>, DerivError<Component>)] {
        &self.failed_cardinality
    }
}

impl<K, V, R, Ctx, P> Iterator for IterCartesianProduct<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    type Item = Result<Pending<K, V, R>, rbe_error::RbeError<K, V, R, Ctx, P>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_state = self.state.next();
        match next_state {
            None => {
                if self.is_first {
                    //trace!("Should be internal error? No more candidates");
                    //trace!("RBE: {}", self.rbe);
                    None
                } else {
                    //trace!("No more candidates");
                    None
                }
            },
            Some(vs) => {
                let candidate: Vec<(K, V)> = vs.iter().map(|(k, v, _, _, _)| (k.clone(), v.clone())).collect();
                let mut pending: Pending<K, V, R> = Pending::empty();
                for (k, v, ctx, _, cond) in &vs {
                    match cond.matches(v, ctx) {
                        Ok(mut new_pending) => {
                            /*tracing::trace!(
                                "Condition {} matches value {}, pending: {} for key {}",
                                cond,
                                v,
                                pending,
                                k
                            );*/
                            new_pending.annotate_key(k);
                            pending.merge(new_pending);
                        },
                        Err(err) => {
                            //tracing::trace!("Failed condition: {cond} with value: {v} and key {k}, error: {err}");
                            self.failed.push((candidate, k.clone(), v.clone(), err.clone()));
                            return Some(Err(err));
                        },
                    }
                }
                let bag = Bag::from_iter(vs.into_iter().map(|(_, _, _, c, _)| c));
                match self.rbe.match_bag_interval(&bag, self.open) {
                    Ok(()) => {
                        //tracing::trace!("### Rbe {} matches bag {}", self.rbe, bag);
                        self.is_first = false;
                        Some(Ok(pending))
                    },
                    Err(err) => {
                        //tracing::trace!("### Rbe {} does not match bag {}, error: {err}", self.rbe, bag);
                        //trace!("### Skipped error: {err}!\n");
                        self.failed_cardinality.push((candidate, err));
                        self.next()
                    },
                }
            },
        }
    }
}

impl<K, V, R, Ctx, P> Display for RbeTable<K, V, R, Ctx, P>
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
    Ctx: Context + Display,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RBE [{}]", self.rbe)?;
        write!(
            f,
            ", Keys: [{}]",
            self.key_components
                .iter()
                .map(|(k, c)| format!("{k} -> {{{}}}", c.iter().map(|c| c.to_string()).join(" ")))
                .join(", ")
        )?;
        write!(
            f,
            ", conds: [{}]",
            self.component_cond
                .iter()
                .map(|(c, cond)| format!("{c} -> {cond}"))
                .join(", ")
        )?;
        Ok(())
    }
}

#[allow(clippy::type_complexity)]
pub fn show_candidate<K, V, R, Ctx, P>(candidate: &[(K, V, Ctx, Component, MatchCond<K, V, R, Ctx, P>)]) -> String
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
    Ctx: Context + Display,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    candidate
        .iter()
        .enumerate()
        .map(|(i, (k, v, _ctx, c, cond))| format!("[{i}: ({k} {v})@{c} {cond}]"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use crate::{Max, SingleCond};
    use serde::Deserialize;

    use super::*;

    impl Key for char {}
    impl Value for char {}
    impl Ref for char {}

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    enum TestKind {
        Equals(char),
        Ref(char),
    }

    impl MatchKind<char, char, char, char> for TestKind {
        fn eval(
            &self,
            v: &char,
            _ctx: &char,
        ) -> Result<Pending<char, char, char>, RbeError<char, char, char, char, Self>> {
            match self {
                TestKind::Equals(expected) => {
                    if *v == *expected {
                        Ok(Pending::empty())
                    } else {
                        Err(RbeError::MsgError {
                            msg: format!("Value {v}!='{expected}'"),
                        })
                    }
                },
                TestKind::Ref(token) => {
                    let mut pending = Pending::empty();
                    pending.insert(*v, *token);
                    Ok(pending)
                },
            }
        }
    }

    type Cond = MatchCond<char, char, char, char, TestKind>;
    type Table = RbeTable<char, char, char, char, TestKind>;

    fn is_a() -> Cond {
        MatchCond::single(SingleCond::new().with_name("is_a").with_kind(TestKind::Equals('a')))
    }

    fn is_x() -> Cond {
        MatchCond::single(SingleCond::new().with_name("is_x").with_kind(TestKind::Equals('x')))
    }

    fn is_y() -> Cond {
        MatchCond::single(SingleCond::new().with_name("is_y").with_kind(TestKind::Equals('y')))
    }

    fn ref_(name: &str, token: char) -> Cond {
        MatchCond::single(SingleCond::new().with_name(name).with_kind(TestKind::Ref(token)))
    }

    #[test]
    fn test_rbe_table_1() {
        // { p a; q y; q z } == { p is_a; q @t ; q @u }
        //     Pending y/@t, z/@u | y@u, z@t
        let vs = vec![('p', 'a', ' '), ('q', 'y', ' '), ('q', 'z', ' ')];

        // rbe_table = { p is_a ; q @t ; q @u+ }
        let mut rbe_table: Table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_a());
        let c2 = rbe_table.add_component('q', &ref_("ref_t", 't'));
        let c3 = rbe_table.add_component('q', &ref_("ref_u", 'u'));
        rbe_table.with_rbe(RbeStruct::and(vec![
            RbeStruct::symbol(c1, 1, Max::IntMax(1)),
            RbeStruct::symbol(c2, 1, Max::IntMax(1)),
            RbeStruct::symbol(c3, 1, Max::Unbounded),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        let mut expected1 = Pending::new();
        expected1.insert_with_key('y', 't', 'q');
        expected1.insert_with_key('z', 'u', 'q');
        assert_eq!(iter.next(), Some(Ok(expected1)));

        let mut expected2 = Pending::new();
        expected2.insert_with_key('y', 'u', 'q');
        expected2.insert_with_key('z', 't', 'q');
        assert_eq!(iter.next(), Some(Ok(expected2)));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rbe_table_2_fail() {
        // { p a; q y } != { p is_a; q @t ; q @u }
        let vs = vec![('p', 'a', ' '), ('q', 'y', ' ')];

        // rbe_table = { p is_a ; q @t ; q @u+ }
        let mut rbe_table: Table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_a());
        let c2 = rbe_table.add_component('q', &ref_("ref_t", 't'));
        let c3 = rbe_table.add_component('q', &ref_("ref_u", 'u'));
        rbe_table.with_rbe(RbeStruct::and(vec![
            RbeStruct::symbol(c1, 1, Max::IntMax(1)),
            RbeStruct::symbol(c2, 1, Max::IntMax(1)),
            RbeStruct::symbol(c3, 1, Max::Unbounded),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rbe_table_3_basic() {
        // { p a; q a } == { p is_a; q is_a }
        //     Ok
        let vs = vec![('p', 'a', ' '), ('q', 'a', ' ')];

        // rbe_table = { p is_a ; q is_a }
        let mut rbe_table: Table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_a());
        let c2 = rbe_table.add_component('q', &is_a());
        rbe_table.with_rbe(RbeStruct::and(vec![
            RbeStruct::symbol(c1, 1, Max::IntMax(1)),
            RbeStruct::symbol(c2, 1, Max::IntMax(1)),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(iter.next(), Some(Ok(Pending::empty())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rbe_table_4_basic_fail() {
        // { p a; q b } == { p is_a; q is_a }
        let vs = vec![('p', 'a', ' '), ('q', 'b', ' ')];

        // rbe_table = { p is_a ; q is_a }
        let mut rbe_table: Table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_a());
        let c2 = rbe_table.add_component('q', &is_a());
        rbe_table.with_rbe(RbeStruct::and(vec![
            RbeStruct::symbol(c1, 1, Max::IntMax(1)),
            RbeStruct::symbol(c2, 1, Max::IntMax(1)),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(
            iter.next(),
            Some(Err(rbe_error::RbeError::MsgError {
                msg: "Value b!='a'".to_string()
            }))
        );
    }

    /// Reproduces the bug where two strict conditions on the same key
    /// (e.g. `a [ex:Person] ; a [ex:Employee]`) failed because the
    /// cartesian product tried invalid pairings before valid ones.
    #[test]
    fn test_rbe_table_5_same_key_strict_conditions() {
        // { p x; p y } == { p is_x; p is_y }
        // Each value should match exactly one condition.
        let vs = vec![('p', 'x', ' '), ('p', 'y', ' ')];

        // rbe_table = { p is_x ; p is_y }
        let mut rbe_table: Table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_x());
        let c2 = rbe_table.add_component('p', &is_y());
        rbe_table.with_rbe(RbeStruct::and(vec![
            RbeStruct::symbol(c1, 1, Max::IntMax(1)),
            RbeStruct::symbol(c2, 1, Max::IntMax(1)),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(iter.next(), Some(Ok(Pending::empty())));
        assert_eq!(iter.next(), None);
    }

    /// Same key, two strict conditions, but one value doesn't match any.
    #[test]
    fn test_rbe_table_6_same_key_strict_no_match() {
        // Value 'z' doesn't match is_x or is_y
        let vs = vec![('p', 'x', ' '), ('p', 'z', ' ')];

        let mut rbe_table: Table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_x());
        let c2 = rbe_table.add_component('p', &is_y());
        rbe_table.with_rbe(RbeStruct::and(vec![
            RbeStruct::symbol(c1, 1, Max::IntMax(1)),
            RbeStruct::symbol(c2, 1, Max::IntMax(1)),
        ]));

        // matches() itself returns Err because 'z' matches no component
        let result = rbe_table.matches(vs);
        assert!(result.is_err());
    }
}

use indexmap::IndexMap;
use indexmap::IndexSet;
use itertools::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::vec::IntoIter;
use tracing::debug;

use crate::Bag;
use crate::Key;
use crate::MatchCond;
use crate::Pending;
use crate::RbeError;
use crate::Ref;
use crate::Value;
// use crate::RbeError;
use crate::rbe::Rbe;
use crate::rbe1::Rbe as Rbe1;
use crate::rbe_error;
use crate::values::Values;
use crate::Component;

#[derive(Default, PartialEq, Eq, Clone)]
pub struct RbeTable<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    rbe: Rbe<Component>,
    key_components: IndexMap<K, IndexSet<Component>>,
    component_cond: IndexMap<Component, MatchCond<K, V, R>>,
    component_key: HashMap<Component, K>,
    open: bool,
    component_counter: usize,
}

impl<K, V, R> RbeTable<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub fn new() -> RbeTable<K, V, R> {
        RbeTable::default()
    }

    pub fn add_component(&mut self, k: K, cond: &MatchCond<K, V, R>) -> Component {
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

    pub fn with_rbe(&mut self, rbe: Rbe<Component>) {
        self.rbe = rbe;
    }

    pub fn matches(
        &self,
        values: Vec<(K, V)>,
    ) -> Result<MatchTableIter<K, V, R>, RbeError<K, V, R>> {
        let mut pairs_found = 0;
        let mut candidates = Vec::new();
        let cs_empty = IndexSet::new();
        for (key, value) in &values {
            let components = self.key_components.get(key).unwrap_or(&cs_empty);
            let mut pairs = Vec::new();
            for component in components {
                // TODO: Add some better error control to replace unwrap()?
                //  This should mark an internal error anyway
                let cond = self.component_cond.get(component).unwrap();
                pairs_found += 1;
                pairs.push((key.clone(), value.clone(), *component, cond.clone()));
            }
            candidates.push(pairs);
        }

        if candidates.is_empty() || pairs_found == 0 {
            debug!(
                "No candidates for rbe: {:?}, candidates: {:?}, pairs_found: {pairs_found}",
                self.rbe, candidates,
            );
            Ok(MatchTableIter::Empty(EmptyIter {
                is_first: true,
                rbe: cnv_rbe(&self.rbe, self),
                values: Values::from(&values),
            }))
        } else {
            debug!("Candidates not empty rbe: {:?}", self.rbe);
            let _: Vec<_> = candidates
                .iter()
                .zip(0..)
                .map(|(candidate, n)| {
                    debug!("Candidate {n}: {candidate:?}");
                })
                .collect();
            let mp = candidates.into_iter().multi_cartesian_product();
            Ok(MatchTableIter::NonEmpty(IterCartesianProduct {
                is_first: true,
                state: mp,
                rbe: self.rbe.clone(),
                open: self.open,
                // controlled: self.controlled.clone()
            }))
        }
    }
}

impl<K, V, R> Debug for RbeTable<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
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

#[derive(Debug)]
pub enum MatchTableIter<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    Empty(EmptyIter<K, V, R>),
    NonEmpty(IterCartesianProduct<K, V, R>),
}

impl<K, V, R> Iterator for MatchTableIter<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    type Item = Result<Pending<V, R>, rbe_error::RbeError<K, V, R>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MatchTableIter::Empty(ref mut e) => {
                debug!("MatchTableIter::Empty");
                e.next()
            }
            MatchTableIter::NonEmpty(ref mut cp) => {
                debug!("MatchTableIter::NonEmpty");
                cp.next()
            }
        }
    }
}

type State<K, V, R> = MultiProduct<IntoIter<(K, V, Component, MatchCond<K, V, R>)>>;

#[derive(Debug)]
pub struct IterCartesianProduct<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    is_first: bool,
    state: State<K, V, R>,
    rbe: Rbe<Component>,
    open: bool,
    // controlled: HashSet<K>
}

impl<K, V, R> Iterator for IterCartesianProduct<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    type Item = Result<Pending<V, R>, rbe_error::RbeError<K, V, R>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_state = self.state.next();
        debug!("state in IterCartesianProduct {:?}", self.state);
        match next_state {
            None => {
                if self.is_first {
                    debug!("Should be internal error? No more candidates");
                    debug!("RBE: {}", self.rbe);
                    None
                } else {
                    debug!("No more candidates");
                    None
                }
            }
            Some(vs) => {
                for (k, v, c, cond) in &vs {
                    debug!("Next state: ({k} {v}) should match component {c} with cond: {cond})");
                }
                let mut pending: Pending<V, R> = Pending::new();
                for (_k, v, _, cond) in &vs {
                    match cond.matches(v) {
                        Ok(new_pending) => {
                            debug!("Condition passed: {cond} with value: {v}");
                            pending.merge(new_pending);
                        }
                        Err(err) => {
                            debug!("Failed condition: {cond} with value: {v}");
                            return Some(Err(err));
                        }
                    }
                }
                debug!("Pending after checking conditions: {pending:?}");
                let bag = Bag::from_iter(vs.into_iter().map(|(_, _, c, _)| c));
                match self.rbe.match_bag(&bag, self.open) {
                    Ok(()) => {
                        debug!("Rbe {} matches bag {}", self.rbe, bag);
                        self.is_first = false;
                        Some(Ok(pending))
                    }
                    Err(err) => {
                        debug!("### Skipped error: {err}!!!!\n");
                        self.next()
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct EmptyIter<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    is_first: bool,
    rbe: Rbe1<K, V, R>,
    values: Values<K, V>,
}

impl<K, V, R> Iterator for EmptyIter<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    type Item = Result<Pending<V, R>, rbe_error::RbeError<K, V, R>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            Some(Err(RbeError::EmptyCandidates {
                rbe: Box::new(self.rbe.clone()),
                values: self.values.clone(),
            }))
        } else {
            None
        }
    }
}

fn cnv_rbe<K, V, R>(rbe: &Rbe<Component>, table: &RbeTable<K, V, R>) -> Rbe1<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    match rbe {
        Rbe::Empty => Rbe1::Empty,
        Rbe::And { values } => {
            let values1 = values.iter().map(|c| cnv_rbe(c, table)).collect();
            Rbe1::And { exprs: values1 }
        }
        Rbe::Or { values } => {
            let values1 = values.iter().map(|c| cnv_rbe(c, table)).collect();
            Rbe1::Or { exprs: values1 }
        }
        Rbe::Symbol { value, card } => {
            let key = cnv_key(value, table);
            let cond = cnv_cond(value, table);
            Rbe1::Symbol {
                key,
                cond,
                card: (*card).clone(),
            }
        }
        _ => todo!(),
    }
}

fn cnv_cond<K, V, R>(c: &Component, table: &RbeTable<K, V, R>) -> MatchCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    table.component_cond.get(c).unwrap().clone()
}

fn cnv_key<K, V, R>(c: &Component, table: &RbeTable<K, V, R>) -> K
where
    K: Key,
    V: Value,
    R: Ref,
{
    table.component_key.get(c).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::{Max, SingleCond};

    use super::*;

    impl Key for char {}
    impl Value for char {}
    impl Ref for char {}

    #[test]
    fn test_rbe_table_1() {
        // { p a; q y; q z } == { p is_a; q @t ; q @u }
        //     Pending y/@t, z/@u | y@u, z@t
        let is_a: MatchCond<char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_a").with_cond(move |v| {
                if *v == 'a' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='a'"),
                    })
                }
            }));

        let ref_t: MatchCond<char, char, char> =
            MatchCond::single(SingleCond::new().with_name("ref_t").with_cond(move |v| {
                let mut pending = Pending::new();
                pending.insert(*v, 't');
                Ok(pending)
            }));

        let ref_u: MatchCond<char, char, char> =
            MatchCond::single(SingleCond::new().with_name("ref_u").with_cond(move |v| {
                let mut pending = Pending::new();
                pending.insert(*v, 'u');
                Ok(pending)
            }));

        let vs = vec![('p', 'a'), ('q', 'y'), ('q', 'z')];

        // rbe_table = { p is_a ; q @t ; q @u+ }
        let mut rbe_table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_a);
        let c2 = rbe_table.add_component('q', &ref_t);
        let c3 = rbe_table.add_component('q', &ref_u);
        rbe_table.with_rbe(Rbe::and(vec![
            Rbe::symbol(c1, 1, Max::IntMax(1)),
            Rbe::symbol(c2, 1, Max::IntMax(1)),
            Rbe::symbol(c3, 1, Max::Unbounded),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(
            iter.next(),
            Some(Ok(Pending::from(
                vec![('y', vec!['t']), ('z', vec!['u'])].into_iter()
            )))
        );
        assert_eq!(
            iter.next(),
            Some(Ok(Pending::from(
                vec![('y', vec!['u']), ('z', vec!['t'])].into_iter()
            )))
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rbe_table_2_fail() {
        // { p a; q y } != { p is_a; q @t ; q @u }
        //     Pending y/@t, z/@u | y@u, z@t
        let is_a: MatchCond<char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_a").with_cond(move |v| {
                if *v == 'a' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='a'"),
                    })
                }
            }));

        let ref_t: MatchCond<char, char, char> =
            MatchCond::single(SingleCond::new().with_name("ref_t").with_cond(move |v| {
                let mut pending = Pending::new();
                pending.insert(*v, 't');
                Ok(pending)
            }));

        let ref_u: MatchCond<char, char, char> =
            MatchCond::single(SingleCond::new().with_name("ref_u").with_cond(move |v| {
                let mut pending = Pending::new();
                pending.insert(*v, 'u');
                Ok(pending)
            }));

        let vs = vec![('p', 'a'), ('q', 'y')];

        // rbe_table = { p is_a ; q @t ; q @u+ }
        let mut rbe_table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_a);
        let c2 = rbe_table.add_component('q', &ref_t);
        let c3 = rbe_table.add_component('q', &ref_u);
        rbe_table.with_rbe(Rbe::and(vec![
            Rbe::symbol(c1, 1, Max::IntMax(1)),
            Rbe::symbol(c2, 1, Max::IntMax(1)),
            Rbe::symbol(c3, 1, Max::Unbounded),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rbe_table_3_basic() {
        // { p a; q a } == { p is_a; q is_a }
        //     Ok
        let is_a: MatchCond<char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_a").with_cond(move |v| {
                if *v == 'a' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='a'"),
                    })
                }
            }));

        let vs = vec![('p', 'a'), ('q', 'a')];

        // rbe_table = { p is_a ; q is_a }
        let mut rbe_table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_a);
        let c2 = rbe_table.add_component('q', &is_a);
        rbe_table.with_rbe(Rbe::and(vec![
            Rbe::symbol(c1, 1, Max::IntMax(1)),
            Rbe::symbol(c2, 1, Max::IntMax(1)),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(iter.next(), Some(Ok(Pending::new())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rbe_table_4_basic_fail() {
        // { p a; q b } == { p is_a; q is_a }
        //     Ok
        let is_a: MatchCond<char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_a").with_cond(move |v| {
                if *v == 'a' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='a'"),
                    })
                }
            }));

        let vs = vec![('p', 'a'), ('q', 'b')];

        // rbe_table = { p is_a ; q is_a }
        let mut rbe_table = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_a);
        let c2 = rbe_table.add_component('q', &is_a);
        rbe_table.with_rbe(Rbe::and(vec![
            Rbe::symbol(c1, 1, Max::IntMax(1)),
            Rbe::symbol(c2, 1, Max::IntMax(1)),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(
            iter.next(),
            Some(Err(rbe_error::RbeError::MsgError {
                msg: "Value b!='a'".to_string()
            }))
        );
    }
}

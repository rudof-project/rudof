use crate::Bag;
use crate::Component;
use crate::Context;
use crate::EmptyIter;
use crate::Key;
use crate::MatchCond;
use crate::Pending;
use crate::RbeError;
use crate::Ref;
use crate::Value;
use crate::rbe::Rbe;
use crate::rbe_error;
use crate::values::Values;
use indexmap::IndexMap;
use indexmap::IndexSet;
use itertools::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::vec::IntoIter;
use tracing::trace;

#[derive(Default, PartialEq, Eq, Clone)]
pub struct RbeTable<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    // A regular bag expression of components
    rbe: Rbe<Component>,

    // Each key is associated with a set of components
    key_components: IndexMap<K, IndexSet<Component>>,

    // TODO: Unify in a single table component_cond and component_key
    component_cond: IndexMap<Component, MatchCond<K, V, R, Ctx>>,
    component_key: HashMap<Component, K>,

    // Indicates if the RBE is open or closed
    open: bool,

    // Counter for the number of components
    component_counter: usize,
}

impl<K, V, R, Ctx> RbeTable<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    pub fn new() -> RbeTable<K, V, R, Ctx> {
        RbeTable::default()
    }

    pub fn get_condition(&self, c: &Component) -> Option<&MatchCond<K, V, R, Ctx>> {
        self.component_cond.get(c)
    }

    pub fn get_key(&self, c: &Component) -> Option<&K> {
        self.component_key.get(c)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.key_components.keys()
    }

    pub fn add_component(&mut self, k: K, cond: &MatchCond<K, V, R, Ctx>) -> Component {
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

    pub fn matches(&self, values: Vec<(K, V, Ctx)>) -> Result<MatchTableIter<K, V, R, Ctx>, RbeError<K, V, R, Ctx>> {
        /*trace!(
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
                            trace!("Pre-filter: condition {cond} rejected value {value} for component {component}");
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
            trace!(
                "No candidates for rbe: {}, candidates: {:?}, pairs_found: {pairs_found}",
                self.rbe, candidates,
            );
            if self.rbe.nullable() {
                trace!("Rbe is nullable and no candidates...should be sucessful");

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
                trace!("Result of matches: {:?}", result);
                result
            }
        } else {
            trace!("Some candidates found for rbe: {:?}", self.rbe);
            let mp = candidates.into_iter().multi_cartesian_product();
            Ok(MatchTableIter::NonEmpty(IterCartesianProduct {
                is_first: true,
                state: mp,
                rbe: self.rbe.clone(),
                open: self.open,
            }))
        }
    }

    pub fn components(&self) -> ComponentsIter<'_, K, V, R, Ctx> {
        ComponentsIter {
            current: 0,
            table: self,
        }
    }

    pub fn find_cond(&self, key: &K) -> Option<&MatchCond<K, V, R, Ctx>> {
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
        let rbe_str = self.rbe.map(&|c| {
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

pub struct ComponentsIter<'a, K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    current: usize,
    table: &'a RbeTable<K, V, R, Ctx>,
}

impl<K, V, R, Ctx> Iterator for ComponentsIter<'_, K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    type Item = (Component, K, MatchCond<K, V, R, Ctx>);

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

impl<K, V, R, Ctx> Debug for ComponentsIter<'_, K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentsIter")
            .field("current", &self.current)
            .field("table", &self.table)
            .finish()
    }
}

impl<K, V, R, Ctx> Debug for RbeTable<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
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
pub enum MatchTableIter<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    Empty(EmptyIter<K, V, R, Ctx>),
    NonEmpty(IterCartesianProduct<K, V, R, Ctx>),
}

impl<K, V, R, Ctx> Iterator for MatchTableIter<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    type Item = Result<Pending<V, R>, rbe_error::RbeError<K, V, R, Ctx>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MatchTableIter::Empty(e) => e.next(),
            MatchTableIter::NonEmpty(cp) => cp.next(),
        }
    }
}

type IterState<K, V, R, Ctx> = MultiProduct<IntoIter<(K, V, Ctx, Component, MatchCond<K, V, R, Ctx>)>>;

#[derive(Debug, Clone)]
pub struct IterCartesianProduct<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    is_first: bool,
    state: IterState<K, V, R, Ctx>,
    rbe: Rbe<Component>,
    open: bool,
}

impl<K, V, R, Ctx> Iterator for IterCartesianProduct<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    type Item = Result<Pending<V, R>, rbe_error::RbeError<K, V, R, Ctx>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_state = self.state.next();
        match next_state {
            None => {
                if self.is_first {
                    trace!("Should be internal error? No more candidates");
                    trace!("RBE: {}", self.rbe);
                    None
                } else {
                    trace!("No more candidates");
                    None
                }
            },
            Some(vs) => {
                let mut pending: Pending<V, R> = Pending::new();
                for (_k, v, ctx, _, cond) in &vs {
                    match cond.matches(v, ctx) {
                        Ok(new_pending) => {
                            pending.merge(new_pending);
                        },
                        Err(err) => {
                            trace!("Failed condition: {cond} with value: {v}");
                            return Some(Err(err));
                        },
                    }
                }
                let bag = Bag::from_iter(vs.into_iter().map(|(_, _, _, c, _)| c));
                match self.rbe.match_bag(&bag, self.open) {
                    Ok(()) => {
                        trace!("Rbe {} matches bag {}", self.rbe, bag);
                        self.is_first = false;
                        Some(Ok(pending))
                    },
                    Err(err) => {
                        trace!("### Rbe {} does not match bag {}, error: {err}", self.rbe, bag);
                        trace!("### Skipped error: {err}!\n");
                        self.next()
                    },
                }
            },
        }
    }
}

impl<K, V, R, Ctx> Display for RbeTable<K, V, R, Ctx>
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
    Ctx: Context + Display,
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
pub fn show_candidate<K, V, R, Ctx>(candidate: &[(K, V, Ctx, Component, MatchCond<K, V, R, Ctx>)]) -> String
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
    Ctx: Context + Display,
{
    candidate
        .iter()
        .map(|(k, v, ctx, c, cond)| format!("({k} {v} {ctx})@{c} {cond}"))
        .collect::<Vec<_>>()
        .join(", ")
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
        let is_a: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_a").with_cond(move |v, _ctx| {
                if *v == 'a' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='a'"),
                    })
                }
            }));

        let ref_t: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("ref_t").with_cond(move |v, _ctx| {
                let mut pending = Pending::new();
                pending.insert(*v, 't');
                Ok(pending)
            }));

        let ref_u: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("ref_u").with_cond(move |v, _ctx| {
                let mut pending = Pending::new();
                pending.insert(*v, 'u');
                Ok(pending)
            }));

        let vs = vec![('p', 'a', ' '), ('q', 'y', ' '), ('q', 'z', ' ')];

        // rbe_table = { p is_a ; q @t ; q @u+ }
        let mut rbe_table: RbeTable<char, char, char, char> = RbeTable::new();
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
            Some(Ok(Pending::from(vec![('y', vec!['t']), ('z', vec!['u'])].into_iter())))
        );
        assert_eq!(
            iter.next(),
            Some(Ok(Pending::from(vec![('y', vec!['u']), ('z', vec!['t'])].into_iter())))
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rbe_table_2_fail() {
        // { p a; q y } != { p is_a; q @t ; q @u }
        //     Pending y/@t, z/@u | y@u, z@t
        let is_a: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_a").with_cond(move |v, _ctx| {
                if *v == 'a' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='a'"),
                    })
                }
            }));

        let ref_t: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("ref_t").with_cond(move |v, _ctx| {
                let mut pending = Pending::new();
                pending.insert(*v, 't');
                Ok(pending)
            }));

        let ref_u: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("ref_u").with_cond(move |v, _ctx| {
                let mut pending = Pending::new();
                pending.insert(*v, 'u');
                Ok(pending)
            }));

        let vs = vec![('p', 'a', ' '), ('q', 'y', ' ')];

        // rbe_table = { p is_a ; q @t ; q @u+ }
        let mut rbe_table: RbeTable<char, char, char, char> = RbeTable::new();
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
        let is_a: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_a").with_cond(move |v, _ctx| {
                if *v == 'a' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='a'"),
                    })
                }
            }));

        let vs = vec![('p', 'a', ' '), ('q', 'a', ' ')];

        // rbe_table = { p is_a ; q is_a }
        let mut rbe_table: RbeTable<char, char, char, char> = RbeTable::new();
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
        let is_a: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_a").with_cond(move |v, _ctx| {
                if *v == 'a' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='a'"),
                    })
                }
            }));

        let vs = vec![('p', 'a', ' '), ('q', 'b', ' ')];

        // rbe_table = { p is_a ; q is_a }
        let mut rbe_table: RbeTable<char, char, char, char> = RbeTable::new();
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

    /// Reproduces the bug where two strict conditions on the same key
    /// (e.g. `a [ex:Person] ; a [ex:Employee]`) failed because the
    /// cartesian product tried invalid pairings before valid ones.
    #[test]
    fn test_rbe_table_5_same_key_strict_conditions() {
        // { p x; p y } == { p is_x; p is_y }
        // Each value should match exactly one condition.
        let is_x: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_x").with_cond(move |v, _ctx| {
                if *v == 'x' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='x'"),
                    })
                }
            }));

        let is_y: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_y").with_cond(move |v, _ctx| {
                if *v == 'y' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='y'"),
                    })
                }
            }));

        let vs = vec![('p', 'x', ' '), ('p', 'y', ' ')];

        // rbe_table = { p is_x ; p is_y }
        let mut rbe_table: RbeTable<char, char, char, char> = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_x);
        let c2 = rbe_table.add_component('p', &is_y);
        rbe_table.with_rbe(Rbe::and(vec![
            Rbe::symbol(c1, 1, Max::IntMax(1)),
            Rbe::symbol(c2, 1, Max::IntMax(1)),
        ]));

        let mut iter = rbe_table.matches(vs).unwrap();

        assert_eq!(iter.next(), Some(Ok(Pending::new())));
        assert_eq!(iter.next(), None);
    }

    /// Same key, two strict conditions, but one value doesn't match any.
    #[test]
    fn test_rbe_table_6_same_key_strict_no_match() {
        let is_x: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_x").with_cond(move |v, _ctx| {
                if *v == 'x' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='x'"),
                    })
                }
            }));

        let is_y: MatchCond<char, char, char, char> =
            MatchCond::single(SingleCond::new().with_name("is_y").with_cond(move |v, _ctx| {
                if *v == 'y' {
                    Ok(Pending::new())
                } else {
                    Err(rbe_error::RbeError::MsgError {
                        msg: format!("Value {v}!='y'"),
                    })
                }
            }));

        // Value 'z' doesn't match is_x or is_y
        let vs = vec![('p', 'x', ' '), ('p', 'z', ' ')];

        let mut rbe_table: RbeTable<char, char, char, char> = RbeTable::new();
        let c1 = rbe_table.add_component('p', &is_x);
        let c2 = rbe_table.add_component('p', &is_y);
        rbe_table.with_rbe(Rbe::and(vec![
            Rbe::symbol(c1, 1, Max::IntMax(1)),
            Rbe::symbol(c2, 1, Max::IntMax(1)),
        ]));

        // matches() itself returns Err because 'z' matches no component
        let result = rbe_table.matches(vs);
        assert!(result.is_err());
    }
}

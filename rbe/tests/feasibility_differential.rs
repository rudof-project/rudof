//! Differential soundness tests for the feasibility refutation layer.
//!
//! Property under test: `RbeTable::feasible_neighs(values) == false` implies that
//! `RbeTable::matches(values)` produces no successful candidate — i.e. the guard never
//! refutes a satisfiable neighbourhood. Verified exhaustively over every sub-bag of a
//! value pool, for the expression patterns that exercise each rule of the predicate
//! (Or exclusivity, And co-occurrence, cardinality bounds, repetition weakenings).
//!
//! The converse is deliberately not asserted: the predicate is a necessary condition
//! only (count coupling and divisibility are ignored), so `feasible == true` with no
//! match is expected on some instances. See docs/src/internals/feasibility-model.md.

use rbe::rbe_error::RbeError;
use rbe::{Context, Key, MatchCond, MatchKind, Max, Pending, RbeStruct, RbeTable, Ref, SingleCond, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct C(char);
impl std::fmt::Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Key for C {}
impl Value for C {}
impl Ref for C {}
impl Context for C {}

/// Test-only `MatchKind` payload
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum TestKind {
    Any,
    Is(char),
}

impl MatchKind<C, C, C, C> for TestKind {
    fn eval(&self, v: &C, _ctx: &C) -> Result<Pending<C, C, C>, RbeError<C, C, C, C, Self>> {
        match self {
            TestKind::Any => Ok(Pending::empty()),
            TestKind::Is(expected) => {
                if v.0 == *expected {
                    Ok(Pending::empty())
                } else {
                    Err(RbeError::MsgError {
                        msg: format!("Value {v} != {expected}"),
                    })
                }
            },
        }
    }
}

type Cond = MatchCond<C, C, C, C, TestKind>;
type Table = RbeTable<C, C, C, C, TestKind>;

/// A condition accepting only the given value.
fn is(name: &str, expected: char) -> Cond {
    MatchCond::single(SingleCond::new().with_name(name).with_kind(TestKind::Is(expected)))
}

/// A condition accepting any value.
fn any(name: &str) -> Cond {
    MatchCond::single(SingleCond::new().with_name(name).with_kind(TestKind::Any))
}

/// `{ p [a]{1,2} | p . + ; q . }` — shared key across Or branches (the blowup pattern).
fn table_shared_key_or() -> Table {
    let mut t = Table::new();
    let c1 = t.add_component(C('p'), &is("is_a", 'a'));
    let c2 = t.add_component(C('p'), &any("any_p"));
    let c3 = t.add_component(C('q'), &any("any_q"));
    t.with_rbe(RbeStruct::or(vec![
        RbeStruct::symbol(c1, 1, Max::IntMax(2)),
        RbeStruct::and(vec![
            RbeStruct::symbol(c2, 1, Max::Unbounded),
            RbeStruct::symbol(c3, 1, Max::IntMax(1)),
        ]),
    ]));
    t
}

/// `{ p . ; q . }` — mandatory co-occurrence.
fn table_mandatory_pair() -> Table {
    let mut t = Table::new();
    let c1 = t.add_component(C('p'), &any("any_p"));
    let c2 = t.add_component(C('q'), &any("any_q"));
    t.with_rbe(RbeStruct::and(vec![
        RbeStruct::symbol(c1, 1, Max::IntMax(1)),
        RbeStruct::symbol(c2, 1, Max::IntMax(1)),
    ]));
    t
}

/// `{ (p . ; q .)+ }` — count coupling under repetition (feasibility ignores it).
fn table_coupled_plus() -> Table {
    let mut t = Table::new();
    let c1 = t.add_component(C('p'), &any("any_p"));
    let c2 = t.add_component(C('q'), &any("any_q"));
    t.with_rbe(RbeStruct::plus(RbeStruct::and(vec![
        RbeStruct::symbol(c1, 1, Max::IntMax(1)),
        RbeStruct::symbol(c2, 1, Max::IntMax(1)),
    ])));
    t
}

/// `{ (p [a] | q .)* ; p [b]? }` — mixing under star plus an optional restricted symbol.
fn table_star_mix() -> Table {
    let mut t = Table::new();
    let c1 = t.add_component(C('p'), &is("is_a", 'a'));
    let c2 = t.add_component(C('q'), &any("any_q"));
    let c3 = t.add_component(C('p'), &is("is_b", 'b'));
    t.with_rbe(RbeStruct::and(vec![
        RbeStruct::star(RbeStruct::or(vec![
            RbeStruct::symbol(c1, 1, Max::IntMax(1)),
            RbeStruct::symbol(c2, 1, Max::IntMax(1)),
        ])),
        RbeStruct::repeat(RbeStruct::symbol(c3, 1, Max::IntMax(1)), 0, Max::IntMax(1)),
    ]));
    t
}

/// Whether the exact matcher accepts some assignment of the values.
fn matches_some(table: &Table, values: &[(C, C, C)]) -> bool {
    match table.matches(values.to_vec()) {
        Ok(iter) => iter.into_iter().any(|r| r.is_ok()),
        Err(_) => false,
    }
}

/// Exhaustively checks refutation soundness over every sub-bag of the pool.
fn check_sound(table: &Table, pool: &[(char, char)]) {
    let n = pool.len();
    for mask in 0..(1u32 << n) {
        let values: Vec<(C, C, C)> = (0..n)
            .filter(|i| mask & (1 << i) != 0)
            .map(|i| (C(pool[i].0), C(pool[i].1), C(' ')))
            .collect();
        let feasible = table.feasible_neighs(&values);
        let matched = matches_some(table, &values);
        assert!(
            feasible || !matched,
            "refutation unsound for sub-bag {values:?}: feasible_neighs = false but the exact matcher accepts"
        );
    }
}

const POOL: &[(char, char)] = &[('p', 'a'), ('p', 'a'), ('p', 'b'), ('p', 'b'), ('q', 'x'), ('q', 'y')];

#[test]
fn refutation_sound_shared_key_or() {
    check_sound(&table_shared_key_or(), POOL);
}

#[test]
fn refutation_sound_mandatory_pair() {
    check_sound(&table_mandatory_pair(), POOL);
}

#[test]
fn refutation_sound_coupled_plus() {
    check_sound(&table_coupled_plus(), POOL);
}

#[test]
fn refutation_sound_star_mix() {
    check_sound(&table_star_mix(), POOL);
}

/// The guard must actually refute something: the blowup pattern with `p`-values only
/// (no `q`) and more than two of them is infeasible and detected as such.
#[test]
fn refutation_fires_on_blowup_pattern() {
    let table = table_shared_key_or();
    let values: Vec<(C, C, C)> = vec![
        (C('p'), C('b'), C(' ')),
        (C('p'), C('b'), C(' ')),
        (C('p'), C('b'), C(' ')),
    ];
    assert!(!table.feasible_neighs(&values));
    assert!(!matches_some(&table, &values));
}

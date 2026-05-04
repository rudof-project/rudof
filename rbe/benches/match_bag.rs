//! Benchmarks comparing the Derivatives and Interval algorithms for `match_bag`.
//!
//! Four groups each isolate one dimension of the input space:
//!
//! 1. `shape_breadth`  – fixed bag density (2 per symbol), shape grows in number of symbols.
//!    Both algorithms are O(symbols); interval has lower constant overhead.
//!
//! 2. `bag_density`    – fixed 4-symbol shape, bag count per symbol grows.
//!    Derivatives are O(total_occurrences); interval is O(distinct_symbols), so it stays flat.
//!
//! 3. `open_extras`    – fixed 4-symbol shape, open matching, growing number of extra symbols.
//!    Derivatives must process every extra occurrence; interval skips them all.
//!
//! 4. `star_density`   – `(p{1,1})*` shape, bag count grows.
//!    Same O(total) vs O(1) split as `bag_density`, but through a Star node.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use rbe::{Bag, Max, RbeStruct};

// ---------------------------------------------------------------------------
// Shape / bag builders
// ---------------------------------------------------------------------------

/// `And(p_0{1,100}, p_1{1,100}, ..., p_{n-1}{1,100})`
fn flat_and(num_syms: usize) -> RbeStruct<u32> {
    let items: Vec<RbeStruct<u32>> = (0..num_syms as u32)
        .map(|sym| RbeStruct::symbol(sym, 1, Max::IntMax(100)))
        .collect();
    RbeStruct::and(items)
}

/// Bag with `count_per_sym` occurrences of each of the first `num_syms` symbols.
fn bag_dense(num_syms: usize, count_per_sym: usize) -> Bag<u32> {
    let mut bag = Bag::new();
    for sym in 0..num_syms as u32 {
        bag.insert_many(sym, count_per_sym);
    }
    bag
}

/// Bag matching the first `num_syms` symbols (1 each) plus `num_extras` unknown symbols.
fn bag_with_extras(num_syms: usize, num_extras: usize) -> Bag<u32> {
    let mut bag = Bag::new();
    for sym in 0..num_syms as u32 {
        bag.insert(sym);
    }
    for extra in num_syms as u32..(num_syms + num_extras) as u32 {
        bag.insert(extra);
    }
    bag
}

// ---------------------------------------------------------------------------
// Benchmark groups
// ---------------------------------------------------------------------------

/// Shape breadth: how both algorithms scale with the number of distinct symbol types.
fn bench_shape_breadth(c: &mut Criterion) {
    let mut group = c.benchmark_group("shape_breadth");
    for num_syms in [4_usize, 8, 16, 32] {
        let rbe = flat_and(num_syms);
        let bag = bag_dense(num_syms, 2);
        group.throughput(Throughput::Elements(num_syms as u64));
        group.bench_with_input(BenchmarkId::new("deriv", num_syms), &(&rbe, &bag), |b, (rbe, bag)| {
            b.iter(|| rbe.match_bag_deriv(bag, false))
        });
        group.bench_with_input(
            BenchmarkId::new("interval", num_syms),
            &(&rbe, &bag),
            |b, (rbe, bag)| b.iter(|| rbe.match_bag_interval(bag, false)),
        );
    }
    group.finish();
}

/// Bag density: fixed 4-symbol shape, count per symbol grows.
/// Derivatives must re-derive for every occurrence; interval only needs one lookup per type.
fn bench_bag_density(c: &mut Criterion) {
    let mut group = c.benchmark_group("bag_density");
    let rbe = flat_and(4);
    for count_per_sym in [1_usize, 10, 50, 100] {
        let bag = bag_dense(4, count_per_sym);
        group.throughput(Throughput::Elements((4 * count_per_sym) as u64));
        group.bench_with_input(
            BenchmarkId::new("deriv", count_per_sym),
            &(&rbe, &bag),
            |b, (rbe, bag)| b.iter(|| rbe.match_bag_deriv(bag, false)),
        );
        group.bench_with_input(
            BenchmarkId::new("interval", count_per_sym),
            &(&rbe, &bag),
            |b, (rbe, bag)| b.iter(|| rbe.match_bag_interval(bag, false)),
        );
    }
    group.finish();
}

/// Open matching: fixed 4-symbol shape, bag has growing number of extra symbols.
/// Derivatives process each extra symbol in the derivative chain; interval ignores them.
fn bench_open_extras(c: &mut Criterion) {
    let mut group = c.benchmark_group("open_extras");
    let rbe = flat_and(4);
    for num_extras in [0_usize, 8, 32, 128] {
        let bag = bag_with_extras(4, num_extras);
        group.throughput(Throughput::Elements((4 + num_extras) as u64));
        group.bench_with_input(BenchmarkId::new("deriv", num_extras), &(&rbe, &bag), |b, (rbe, bag)| {
            b.iter(|| rbe.match_bag_deriv(bag, true))
        });
        group.bench_with_input(
            BenchmarkId::new("interval", num_extras),
            &(&rbe, &bag),
            |b, (rbe, bag)| b.iter(|| rbe.match_bag_interval(bag, true)),
        );
    }
    group.finish();
}

/// Star density: `(p{1,1})*` shape, bag count grows.
/// Interval uses a short-circuit via `no_symbols_in_bag` and then a single interval check;
/// derivatives must iterate through every occurrence.
fn bench_star_density(c: &mut Criterion) {
    let mut group = c.benchmark_group("star_density");
    let rbe = RbeStruct::star(RbeStruct::symbol(0_u32, 1, Max::IntMax(1)));
    for count_per_sym in [1_usize, 10, 50, 100] {
        let mut bag = Bag::new();
        bag.insert_many(0_u32, count_per_sym);
        group.throughput(Throughput::Elements(count_per_sym as u64));
        group.bench_with_input(
            BenchmarkId::new("deriv", count_per_sym),
            &(&rbe, &bag),
            |b, (rbe, bag)| b.iter(|| rbe.match_bag_deriv(bag, false)),
        );
        group.bench_with_input(
            BenchmarkId::new("interval", count_per_sym),
            &(&rbe, &bag),
            |b, (rbe, bag)| b.iter(|| rbe.match_bag_interval(bag, false)),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_shape_breadth,
    bench_bag_density,
    bench_open_extras,
    bench_star_density,
);
criterion_main!(benches);

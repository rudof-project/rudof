//! A path: junctions of sequences of steps.
//!
//! ```text
//! shapePath        ::= sequenceStep ("," sequenceStep)*
//! sequenceStep     ::= unionStep ("union" unionStep)*
//! unionStep        ::= intersectionStep ("intersection" intersectionStep)*
//! intersectionStep ::= startStep nextStep*
//! startStep        ::= ("/" | "//")? step | shortcut
//! nextStep         ::= ("/" | "//") step | shortcut
//! ```

use super::lexical::{keyword_no_case, token};
use super::shortcuts::shortcut;
use super::steps::step;
use crate::compact::{IRes, Span};
use crate::shexpath::{ShExPath, Step};
use nom::{
    Parser,
    branch::alt,
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::preceded,
};

/// `shapePath ::= sequenceStep ("," sequenceStep)*`
pub(super) fn shape_path(i: Span) -> IRes<ShExPath> {
    map(separated_list1(token(","), sequence_step), |paths| {
        junction(paths, ShExPath::Sequence)
    })
    .parse(i)
}

/// `sequenceStep ::= unionStep ("union" unionStep)*`
fn sequence_step(i: Span) -> IRes<ShExPath> {
    map(separated_list1(keyword_no_case("union"), union_step), |paths| {
        junction(paths, ShExPath::Union)
    })
    .parse(i)
}

/// `unionStep ::= intersectionStep ("intersection" intersectionStep)*`
fn union_step(i: Span) -> IRes<ShExPath> {
    map(
        separated_list1(keyword_no_case("intersection"), intersection_step),
        |paths| junction(paths, ShExPath::Intersection),
    )
    .parse(i)
}

/// A junction of `paths`, or the path itself if there is only one.
fn junction(mut paths: Vec<ShExPath>, make: fn(Vec<ShExPath>) -> ShExPath) -> ShExPath {
    if paths.len() == 1 { paths.remove(0) } else { make(paths) }
}

/// `intersectionStep ::= startStep nextStep*`
fn intersection_step(i: Span) -> IRes<ShExPath> {
    map((start_step, many0(next_step)), |(first, rest)| {
        ShExPath::path(first.into_iter().chain(rest.into_iter().flatten()))
    })
    .parse(i)
}

/// `startStep ::= ("/" | "//")? step | shortcut`
///
/// The separator has no effect on where the path begins.
fn start_step(i: Span) -> IRes<Vec<Step>> {
    alt((map(preceded(opt(separator), step), |step| vec![step]), shortcut)).parse(i)
}

/// `nextStep ::= ("/" | "//") step | shortcut`
fn next_step(i: Span) -> IRes<Vec<Step>> {
    alt((map(preceded(separator, step), |step| vec![step]), shortcut)).parse(i)
}

/// `"/" | "//"`, which currently mean the same thing.
fn separator(i: Span) -> IRes<Span> {
    alt((token("//"), token("/"))).parse(i)
}

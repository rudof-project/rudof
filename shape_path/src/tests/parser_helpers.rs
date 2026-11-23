//! Helpers shared by the ShExPath parser tests.

use crate::{Attribute, AttributeOrAny, Comparator, Filter, Operand, RValue, ShExPath, ShExPathParser, Step, TermType};
use prefixmap::{IriRef, PrefixMap};
use rudof_iri::IriS;

pub(super) fn child(attribute: Attribute) -> Step {
    Step::child(AttributeOrAny::Attribute(attribute))
}

pub(super) fn any() -> Step {
    Step::child(AttributeOrAny::Any)
}

pub(super) fn parse(src: &str) -> ShExPath {
    match ShExPathParser::parse_syntax(src) {
        Ok(path) => path,
        Err(e) => panic!("{src} is not parsed: {e}"),
    }
}

pub(super) fn of_type(step: Step, term_type: TermType) -> Step {
    step.with_filter(Filter::TermType(term_type))
}

pub(super) fn path_of(attribute: Attribute) -> Operand {
    Operand::Path(ShExPath::path([child(attribute)]))
}

pub(super) fn iri(s: &str) -> RValue {
    RValue::Iri(IriRef::iri(IriS::new_unchecked(s)))
}

pub(super) fn pname(prefix: &str, local: &str) -> RValue {
    RValue::Iri(IriRef::prefixed(prefix, local))
}

pub(super) fn example_prefixmap() -> PrefixMap {
    let mut prefixmap = PrefixMap::new();
    prefixmap.add_prefix("", IriS::new_unchecked("http://ex.example/ns#"));
    prefixmap.add_prefix("foaf", IriS::new_unchecked("http://xmlns.com/foaf/0.1/"));
    prefixmap
}

pub(super) fn example_base() -> IriS {
    IriS::new_unchecked("http://ex.example/schema")
}

pub(super) fn resolve(src: &str) -> ShExPath {
    let prefixmap = example_prefixmap();
    match ShExPathParser::parse(src, Some(&example_base()), Some(&prefixmap)) {
        Ok(path) => path,
        Err(e) => panic!("{src} is not parsed: {e}"),
    }
}

pub(super) fn compares(attribute: Attribute, rvalue: RValue) -> ShExPath {
    ShExPath::path([any().with_filter(Filter::compare(path_of(attribute), Comparator::Equal, rvalue))])
}

pub(super) fn steps_of(path: ShExPath) -> Vec<Step> {
    match path {
        ShExPath::Path(steps) => steps,
        other => panic!("{other:?} is not a sequence of steps"),
    }
}

pub(super) fn a() -> ShExPath {
    ShExPath::path([child(Attribute::Shapes)])
}

pub(super) fn b() -> ShExPath {
    ShExPath::path([child(Attribute::Start)])
}

pub(super) fn c() -> ShExPath {
    ShExPath::path([child(Attribute::Imports)])
}

pub(super) fn d() -> ShExPath {
    ShExPath::path([child(Attribute::StartActs)])
}

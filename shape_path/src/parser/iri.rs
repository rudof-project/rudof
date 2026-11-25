//! The IRI terminals of ShExPath, which are those of ShExC (and of SPARQL and Turtle).
//!
//! They are a copy of the ones in the ShExC grammar of `shex_ast`, which depends on this crate.
//! Issue #970 is about keeping one copy of them for both parsers.

use super::span::{IRes, Span, map_error};
use crate::error::ShExPathParseError;
use nom::{
    Err, Input, Parser,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, one_of, satisfy},
    combinator::{opt, recognize},
    error::{ErrorKind, ParseError},
    multi::{count, fold_many0, many0},
    sequence::{delimited, preceded},
};
use prefixmap::IriRef;
use rudof_iri::IriS;

/// `[136s] iri ::= IRIREF | prefixedName`
pub(super) fn iri(i: Span<'_>) -> IRes<'_, IriRef> {
    alt((iri_ref_s, prefixed_name())).parse(i)
}

fn iri_ref_s(i: Span<'_>) -> IRes<'_, IriRef> {
    let (i, iri) = iri_ref(i)?;
    Ok((i, iri.into()))
}

/// `[137s] prefixedName ::= PNAME_LN | PNAME_NS`
fn prefixed_name<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, IriRef> {
    map_error(alt((pname_ln, pname_ns_iri_ref)), || {
        ShExPathParseError::ExpectedPrefixedName
    })
}

fn pname_ns_iri_ref(i: Span<'_>) -> IRes<'_, IriRef> {
    let (i, pname_ns) = pname_ns(i)?;
    Ok((i, IriRef::prefixed(*pname_ns.fragment(), "")))
}

/// `[141s] <PNAME_LN> ::= PNAME_NS PN_LOCAL`
fn pname_ln(i: Span<'_>) -> IRes<'_, IriRef> {
    let (i, (prefix, local)) = (pname_ns, pn_local).parse(i)?;
    Ok((i, IriRef::prefixed(*prefix.fragment(), local)))
}

/// `[77] <PN_LOCAL> ::= (PN_CHARS_U | ":" | [0-9] | PLX) (PN_CHARS | "." | ":" | PLX)`
fn pn_local(i: Span<'_>) -> IRes<'_, &str> {
    let (i, cs) = recognize((alt((one_if(is_pn_local_start), plx)), pn_local_rest)).parse(i)?;
    Ok((i, cs.fragment()))
}

fn is_pn_local_start(c: char) -> bool {
    c == ':' || c.is_ascii_digit() || is_pn_chars_u(c)
}

fn pn_local_rest(i: Span<'_>) -> IRes<'_, Span<'_>> {
    recognize(many0(alt((take_while1(is_pn_chars_colon), plx, recognize(char('.')))))).parse(i)
}

fn is_pn_chars_colon(c: char) -> bool {
    c == ':' || is_pn_chars(c)
}

fn plx(i: Span<'_>) -> IRes<'_, Span<'_>> {
    alt((percent, pn_local_esc)).parse(i)
}

/// `[173s] <PN_LOCAL_ESC> ::= "\\" ( "_" | "~" | "." | "-" | "!" | "$" | "&" | "'" |
///             "(" | ")" | "*" | "+" | "," | ";" | "=" | "/" | "?" | "#" | "@" | "%" )`
fn pn_local_esc(i: Span<'_>) -> IRes<'_, Span<'_>> {
    recognize((char('\\'), one_if(|c| "_~.-!$&'()*+,;=/?#@%".contains(c)))).parse(i)
}

fn percent(i: Span<'_>) -> IRes<'_, Span<'_>> {
    recognize((char('%'), one_if(is_hex), one_if(is_hex))).parse(i)
}

fn is_hex(c: char) -> bool {
    c.is_ascii_hexdigit()
}

/// `[18t] <IRIREF> ::= "<" ([^#0000- <>\"{}|^`\\] | UCHAR)* ">"`
fn iri_ref(i: Span<'_>) -> IRes<'_, IriS> {
    let (i, chars) = delimited(char('<'), many0(alt((satisfy(is_iri_char), uchar))), char('>')).parse(i)?;
    let iri: String = chars.into_iter().collect();
    Ok((i, IriS::new_unchecked(iri.as_str())))
}

fn is_iri_char(c: char) -> bool {
    c > ' ' && !"<>\"{}|^`\\".contains(c)
}

/// `[26t] <UCHAR> ::= "\\u" HEX HEX HEX HEX | "\\U" HEX HEX HEX HEX HEX HEX HEX HEX`
fn uchar(i: Span<'_>) -> IRes<'_, char> {
    let hex = || one_of("0123456789ABCDEFabcdef");
    let (rest, digits) = alt((
        preceded(tag(r"\u"), recognize(count(hex(), 4))),
        preceded(tag(r"\U"), recognize(count(hex(), 8))),
    ))
    .parse(i)?;
    match u32::from_str_radix(digits.fragment(), 16).ok().and_then(char::from_u32) {
        Some(c) => Ok((rest, c)),
        None => Err(Err::Error(ParseError::from_error_kind(i, ErrorKind::Char))),
    }
}

/// `[140s] <PNAME_NS> ::= PN_PREFIX? ":"`
fn pname_ns(i: Span<'_>) -> IRes<'_, Span<'_>> {
    let (i, (prefix, _)) = (opt(pn_prefix), char(':')).parse(i)?;
    Ok((i, prefix.unwrap_or(Span::from(""))))
}

/// `[168s] <PN_PREFIX> ::= PN_CHARS_BASE ( (PN_CHARS | ".")* PN_CHARS )?`
fn pn_prefix(i: Span<'_>) -> IRes<'_, Span<'_>> {
    recognize((
        satisfy(is_pn_chars_base),
        take_while(is_pn_chars),
        fold_many0((char('.'), take_while1(is_pn_chars)), || (), |(), _| ()),
    ))
    .parse(i)
}

/// `[164s] <PN_CHARS_BASE> ::= [A-Z] | [a-z]`
///        `                | [#00C0-#00D6] | [#00D8-#00F6] | [#00F8-#02FF]`
///        `                | [#0370-#037D] | [#037F-#1FFF]`
///        `                | [#200C-#200D] | [#2070-#218F] | [#2C00-#2FEF]`
///        `                | [#3001-#D7FF] | [#F900-#FDCF] | [#FDF0-#FFFD]`
///        `                | [#10000-#EFFFF]`
fn is_pn_chars_base(c: char) -> bool {
    c.is_ascii_alphabetic()
        || matches!(c as u32,
            0x00C0..=0x00D6
            | 0x00D8..=0x00F6
            | 0x00F8..=0x02FF
            | 0x0370..=0x037D
            | 0x037F..=0x1FFF
            | 0x200C..=0x200D
            | 0x2070..=0x218F
            | 0x2C00..=0x2FEF
            | 0x3001..=0xD7FF
            | 0xF900..=0xFDCF
            | 0xFDF0..=0xFFFD
            | 0x10000..=0xEFFFF)
}

/// `[165s] <PN_CHARS_U> ::= PN_CHARS_BASE | "_"`
fn is_pn_chars_u(c: char) -> bool {
    c == '_' || is_pn_chars_base(c)
}

/// `[167s] <PN_CHARS> ::= PN_CHARS_U | "-" | [0-9] | [#00B7] | [#0300-#036F] | [#203F-#2040]`
fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c) || c == '-' || c.is_ascii_digit() || matches!(c as u32, 0x00B7 | 0x0300..=0x036F | 0x203F..=0x2040)
}

/// One character, if it satisfies `f`.
fn one_if<'a, F: Fn(char) -> bool>(f: F) -> impl Fn(Span<'a>) -> IRes<'a, Span<'a>> {
    move |i: Span<'a>| match i.chars().next() {
        Some(c) if f(c) => Ok(i.take_split(c.len_utf8())),
        _ => Err(Err::Error(ParseError::from_error_kind(i, ErrorKind::OneOf))),
    }
}

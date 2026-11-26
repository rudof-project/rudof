//! RDF terms and triples written in Turtle syntax, with IRIs written as
//! prefixed names where a prefix of a [`PrefixMap`] matches.
//!
//! The prefix map should have no colours, hyperlinks or base IRI (see
//! [`PrefixMap::without_colors`], [`PrefixMap::with_hyperlink`] and
//! [`PrefixMap::with_base`]), or they show up in the text.

use oxrdf::{Literal, LiteralRef, NamedNodeRef, NamedOrBlankNodeRef, TermRef, Triple, TripleRef, vocab::xsd};
use prefixmap::PrefixMap;
use rudof_iri::IriS;

/// `term` in Turtle syntax: IRIs as prefixed names where `pm` allows it, and
/// integer, decimal and boolean literals in their short form.
pub fn show_term(term: TermRef<'_>, pm: &PrefixMap) -> String {
    match term {
        TermRef::NamedNode(n) => show_iri(n, pm),
        TermRef::BlankNode(b) => b.to_string(),
        TermRef::Literal(l) => show_literal(l, pm),
        TermRef::Triple(t) => format!("<<( {} )>>", show_triple_ref(t.as_ref(), pm)),
    }
}

/// `triple` in Turtle syntax, without the final `.`; see [`show_term`].
pub fn show_triple(triple: &Triple, pm: &PrefixMap) -> String {
    show_triple_ref(triple.as_ref(), pm)
}

/// The prefixes of `pm` that [`show_term`] uses to write `terms`. A triple
/// can be passed as a triple term ([`TermRef::Triple`]).
pub fn used_prefixes<'t>(terms: impl IntoIterator<Item = TermRef<'t>>, pm: &PrefixMap) -> PrefixMap {
    let mut used = PrefixMap::new();
    for t in terms {
        add_used_in_term(t, pm, &mut used);
    }
    used
}

fn show_triple_ref(t: TripleRef<'_>, pm: &PrefixMap) -> String {
    let subject = match t.subject {
        NamedOrBlankNodeRef::NamedNode(n) => show_iri(n, pm),
        NamedOrBlankNodeRef::BlankNode(b) => b.to_string(),
    };
    format!("{subject} {} {}", show_iri(t.predicate, pm), show_term(t.object, pm))
}

fn show_iri(iri: NamedNodeRef<'_>, pm: &PrefixMap) -> String {
    pm.qualify(&IriS::new_unchecked(iri.as_str()))
}

fn show_literal(l: LiteralRef<'_>, pm: &PrefixMap) -> String {
    let datatype = l.datatype();
    // Simple strings and language-tagged strings have no datatype to write
    if l.language().is_some() || datatype == xsd::STRING {
        return l.to_string();
    }
    if has_short_form(l.value(), datatype) {
        return l.value().to_string();
    }
    format!("{}^^{}", Literal::new_simple_literal(l.value()), show_iri(datatype, pm))
}

/// Whether Turtle can write the literal without quotes and datatype.
fn has_short_form(value: &str, datatype: NamedNodeRef<'_>) -> bool {
    let unsigned = value.strip_prefix(['+', '-']).unwrap_or(value);
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if datatype == xsd::INTEGER {
        digits(unsigned)
    } else if datatype == xsd::DECIMAL {
        // [+-]? [0-9]* '.' [0-9]+
        unsigned
            .split_once('.')
            .is_some_and(|(int, frac)| (int.is_empty() || digits(int)) && digits(frac))
    } else if datatype == xsd::BOOLEAN {
        value == "true" || value == "false"
    } else {
        false
    }
}

fn add_used_in_triple(t: TripleRef<'_>, pm: &PrefixMap, used: &mut PrefixMap) {
    if let NamedOrBlankNodeRef::NamedNode(n) = t.subject {
        add_used(n, pm, used);
    }
    add_used(t.predicate, pm, used);
    add_used_in_term(t.object, pm, used);
}

fn add_used_in_term(term: TermRef<'_>, pm: &PrefixMap, used: &mut PrefixMap) {
    match term {
        TermRef::NamedNode(n) => add_used(n, pm, used),
        TermRef::Literal(l) => {
            let datatype = l.datatype();
            if l.language().is_none() && datatype != xsd::STRING && !has_short_form(l.value(), datatype) {
                add_used(datatype, pm, used);
            }
        },
        TermRef::Triple(inner) => add_used_in_triple(inner.as_ref(), pm, used),
        TermRef::BlankNode(_) => {},
    }
}

/// Adds to `used` the prefix of `pm` that [`show_iri`] writes `iri` with, if any.
fn add_used(iri: NamedNodeRef<'_>, pm: &PrefixMap, used: &mut PrefixMap) {
    let shown = show_iri(iri, pm);
    if shown.starts_with('<') {
        return;
    }
    if let Some((alias, _)) = shown.split_once(':')
        && let Some(prefix_iri) = pm.find(alias)
    {
        used.add_prefix(alias, prefix_iri.clone());
    }
}

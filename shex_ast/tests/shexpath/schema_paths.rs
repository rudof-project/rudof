//! `SchemaPath`: what it shows is a ShExPath that the parser reads back as `to_shexpath`, and that
//! selects the shape expression or triple constraint it names.

use crate::common::*;
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use shex_ast::shexpath::{SchemaPath, SchemaStep, ShExPathParser};

fn iri(local: &str) -> IriS {
    IriS::new_unchecked(&format!("http://a.example/{local}"))
}

fn prefixmap() -> PrefixMap {
    let mut pm = PrefixMap::new();
    pm.add_prefix("", iri(""));
    pm
}

fn tc(local: &str, nth: Option<usize>) -> SchemaStep {
    SchemaStep::TripleConstraint {
        predicate: iri(local),
        nth,
    }
}

fn paths() -> Vec<SchemaPath> {
    vec![
        SchemaPath::shape(iri("S")),
        SchemaPath::shape(iri("S")).with_step(tc("p", None)),
        SchemaPath::shape(iri("S")).with_step(tc("p", Some(1))),
        SchemaPath::shape(iri("S"))
            .with_step(tc("p", Some(0)))
            .with_step(SchemaStep::ValueExpr)
            .with_step(tc("q", None)),
        SchemaPath::shape(iri("A")).with_step(SchemaStep::Operand(1)),
        SchemaPath::shape(iri("N")).with_step(SchemaStep::Negated),
        SchemaPath::start().with_step(tc("p", None)),
    ]
}

#[test]
fn a_schema_path_is_shown_with_the_shortcuts() {
    let shown: Vec<String> = paths().iter().map(|p| p.show_qualified(&prefixmap())).collect();
    assert_eq!(
        shown,
        [
            "@:S",
            "@:S~:p",
            "@:S~:p/self::[1]",
            "@:S~:p/self::[0]/valueExpr~:q",
            "@:A/shapeExprs/*[1]",
            "@:N/shapeExpr",
            "/start~:p",
        ]
    );
    assert_eq!(
        SchemaPath::shape(iri("S")).with_step(tc("p", None)).to_string(),
        "@<http://a.example/S>~<http://a.example/p>"
    );
}

#[test]
fn what_a_schema_path_shows_is_read_back_as_its_shexpath() {
    let pm = prefixmap();
    for path in paths() {
        for shown in [path.to_string(), path.show_qualified(&pm)] {
            let parsed = ShExPathParser::parse(&shown, None, Some(&pm)).unwrap_or_else(|e| panic!("{shown}: {e}"));
            assert_eq!(parsed, path.to_shexpath(), "{shown}");
        }
    }
}

#[test]
fn a_schema_path_selects_what_it_names() {
    let doc = document(
        "PREFIX : <http://a.example/>
         :S { :p . ; ( :q . | :p { :r . } ) }
         :A IRI AND { :p . }
         :N NOT { :p . }
         start = { :p . }",
    );
    let pm = prefixmap();
    let one = |path: SchemaPath| {
        let items = select(&doc, &path.show_qualified(&pm));
        assert_eq!(items.len(), 1, "{}", path.show_qualified(&pm));
        labelled(&items).remove(0)
    };
    assert_eq!(one(SchemaPath::shape(iri("S"))), "Shape");
    // The constraints on :p are counted in the order they are written, also inside a group.
    let second_p = SchemaPath::shape(iri("S")).with_step(tc("p", Some(1)));
    assert_eq!(one(second_p.clone()), "TripleConstraint(:p)");
    assert_eq!(
        selects(&doc, &second_p.show_qualified(&pm)),
        selects(&doc, "@:S/expression/expressions/*[1]/expressions/*[1]")
    );
    assert_eq!(one(second_p.clone().with_step(SchemaStep::ValueExpr)), "Shape");
    assert_eq!(
        one(second_p.with_step(SchemaStep::ValueExpr).with_step(tc("r", None))),
        "TripleConstraint(:r)"
    );
    assert_eq!(
        one(SchemaPath::shape(iri("A")).with_step(SchemaStep::Operand(1))),
        "Shape"
    );
    assert_eq!(one(SchemaPath::shape(iri("N")).with_step(SchemaStep::Negated)), "Shape");
    assert_eq!(
        one(SchemaPath::start().with_step(tc("p", None))),
        "TripleConstraint(:p)"
    );
}

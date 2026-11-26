//! An inclusion `&label` between parentheses is the inclusion itself. It can't
//! have a cardinality or a label, as ShExJ has no place for them.

use rudof_iri::IriS;
use shex_ast::ShExParser;

fn parse(triple_expr: &str) -> Result<String, String> {
    let src = format!("prefix : <http://example.org/>\n:S1 {{ {triple_expr} }}\n:S2 {{ $:e :p . }}");
    let base = IriS::new_unchecked("http://example.org/");
    ShExParser::parse(&src, None, &base)
        .map(|schema| format!("{schema:?}"))
        .map_err(|e| e.to_string())
}

#[test]
fn inclusion_between_parentheses() {
    // This used to panic
    assert!(parse("( &:e ) ; :q .").is_ok());
}

#[test]
fn inclusion_with_cardinality_or_label_is_an_error() {
    for invalid in ["( &:e )+", "( &:e ) // :a 1", "$:x ( &:e )"] {
        let error = parse(invalid).expect_err(invalid);
        assert!(error.contains("An inclusion (&label) can't have"), "{invalid}: {error}");
    }
}

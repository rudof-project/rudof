//! Helpers and fixtures shared by the ShExPath tests.

use rudof_iri::IriS;
use shex_ast::compact::ShExParser;
use shex_ast::shexpath::{ShExJDocument, ShExPathItem, shexj_document};

pub fn document(src: &str) -> ShExJDocument {
    let schema = match ShExParser::parse(src, None, &IriS::new_unchecked("http://default/")) {
        Ok(schema) => schema,
        Err(e) => panic!("the schema is not parsed: {e}"),
    };
    match shexj_document(&schema) {
        Ok(doc) => doc,
        Err(e) => panic!("the schema is not written as ShExJ: {e}"),
    }
}

/// The example schema of the spec.
pub fn issue() -> ShExJDocument {
    document(include_str!("../fixtures/shexpath/issue.shex"))
}

pub fn locations(items: &[ShExPathItem<'_>]) -> Vec<String> {
    items.iter().map(|item| item.location().to_string()).collect()
}

pub fn select<'a>(doc: &'a ShExJDocument, path: &str) -> Vec<ShExPathItem<'a>> {
    match doc.select(path) {
        Ok(items) => items,
        Err(e) => panic!("{path} is not evaluated: {e}"),
    }
}

pub fn selects(doc: &ShExJDocument, path: &str) -> Vec<String> {
    locations(&select(doc, path))
}

/// `<S1> { <p1> . ; <p2> . ; <p3> . }` and `<S2> { <p4> . ; <p5> . }`, the schema of the
/// reference implementation's `filters-test.ts`.
pub fn two_shapes() -> ShExJDocument {
    document(
        "BASE <http://a.example/>
         <S1> { <p1> . ; <p2> . ; <p3> . }
         <S2> { <p4> . ; <p5> . }",
    )
}

/// The local names of the predicates of the triple constraints a path selects.
pub fn predicates(doc: &ShExJDocument, path: &str) -> Vec<String> {
    select(doc, path)
        .iter()
        .map(|item| match item.value()["predicate"].as_str() {
            Some(predicate) => predicate.rsplit(['/', '#']).next().unwrap_or(predicate).to_owned(),
            None => panic!("{} has no predicate", item.value()),
        })
        .collect()
}

/// What each item is, as `type` or `type(predicate or id)`, with `http://a.example/` written `:`.
pub fn described(items: &[ShExPathItem<'_>]) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            let value = item.value();
            let short = |v: &serde_json::Value| v.as_str().map(|s| s.replace("http://a.example/", ":"));
            if let Some(s) = value.as_str() {
                format!("{:?}", s.replace("http://a.example/", ":"))
            } else {
                let ty = value["type"].as_str().unwrap_or("?").to_owned();
                match short(&value["predicate"]).or_else(|| short(&value["id"])) {
                    Some(name) => format!("{ty}({name})"),
                    None => ty,
                }
            }
        })
        .collect()
}

/// The schema of the reference implementation's `inclusions-test.ts`.
pub fn inclusions_schema() -> ShExJDocument {
    document(
        "PREFIX : <http://a.example/>
         BASE <http://a.example/>
         <S1> { &<grp> ; :p9 . }
         <Holder> { $<grp> ( :p1 . ; :p2 . ) }
         <Cycle> { $<loop> ( &<loop> | :p3 . ) }",
    )
}

/// The schema of the reference implementation's `shortcuts-test.ts`.
pub fn shortcuts_schema() -> ShExJDocument {
    document(
        "PREFIX : <http://a.example/>
         BASE <http://a.example/>
         <S1> { $<grp> ( $<t1> :p1 . ; :p2 @<S2> ; :p3 { $<nested> :p4 . } ) }
         <S2> IRI",
    )
}

/// What each item is, as `type` or `type(id or predicate)`, with `http://a.example/` written `:`.
pub fn labelled(items: &[ShExPathItem<'_>]) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            let value = item.value();
            let ty = value["type"].as_str().unwrap_or("?");
            match value["id"].as_str().or_else(|| value["predicate"].as_str()) {
                Some(name) => format!("{ty}({})", name.replace("http://a.example/", ":")),
                None => ty.to_owned(),
            }
        })
        .collect()
}

/// `<S1> { <p1> . ; <p2> . ; <p3> . }` and `<S2> { <p4> . }`, the schema of the reference
/// implementation's `junctions-test.ts`.
pub fn junctions_schema() -> ShExJDocument {
    document(
        "BASE <http://a.example/>
         <S1> { <p1> . ; <p2> . ; <p3> . }
         <S2> { <p4> . }",
    )
}

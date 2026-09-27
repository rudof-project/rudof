use crate::{
    Rudof, RudofConfig,
    formats::{DataFormat, InputSpec, ShExFormat},
};
use std::collections::HashMap;

const INPUT_SCHEMA: &str = r#"PREFIX : <http://in.example/>
PREFIX v: <http://vars.example/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX Map: <http://shex.io/extensions/Map/#>
start = @<Person>
<Person> {
  :name xsd:string %Map:{ v:name %} ;
  :phone { :number xsd:string %Map:{ v:number %} ; :use xsd:string %Map:{ v:use %} }*
}
"#;

const OUTPUT_SCHEMA: &str = r#"PREFIX : <http://out.example/>
PREFIX v: <http://vars.example/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX Map: <http://shex.io/extensions/Map/#>
start = @<Card>
<Card> {
  :fullName xsd:string %Map:{ v:name %} ;
  :tel { :val xsd:string %Map:{ v:number %} ; :kind xsd:string %Map:{ v:use %} }*
}
"#;

const DATA: &str = r#"PREFIX : <http://in.example/>
<ann> :name "Ann" ;
  :phone [ :number "+1 555 0100" ; :use "work" ] , [ :number "+1 555 0199" ; :use "home" ] .
"#;

fn rudof_with(schema: &str, data: Option<&str>) -> Rudof {
    let mut rudof = Rudof::new(RudofConfig::default());
    if let Some(data) = data {
        rudof
            .load_data()
            .with_data(&[InputSpec::Str(data.to_string())])
            .with_data_format(&DataFormat::Turtle)
            .with_base("http://in.example/")
            .execute()
            .expect("data loads");
    }
    rudof
        .load_shex_schema(&InputSpec::Str(schema.to_string()))
        .with_shex_schema_format(&ShExFormat::ShExC)
        .execute()
        .expect("schema loads");
    rudof
}

#[test]
fn bind_then_materialize_maps_nested_repetitions() {
    let mut rudof = rudof_with(INPUT_SCHEMA, Some(DATA));
    rudof.shexmap_bind("<http://in.example/ann>").execute().expect("binds");
    let bindings = rudof.shexmap_bindings().expect("bindings kept");
    assert_eq!(bindings.tree.lists.len(), 1);
    assert_eq!(bindings.tree.lists[0].len(), 2);
    assert!(!bindings.ambiguous());

    let mut json = Vec::new();
    rudof.shexmap_serialize_bindings(&mut json, true).unwrap();
    let text = String::from_utf8(json).unwrap();
    assert!(text.contains("http://vars.example/number"));

    // swap to the output schema and materialize
    rudof
        .load_shex_schema(&InputSpec::Str(OUTPUT_SCHEMA.to_string()))
        .with_shex_schema_format(&ShExFormat::ShExC)
        .execute()
        .unwrap();
    let mut out = Vec::new();
    let summary = rudof
        .shexmap_materialize(&mut out)
        .with_root("<http://out.example/card1>")
        .execute()
        .expect("materializes");
    let turtle = String::from_utf8(out).unwrap();
    assert!(turtle.contains("fullName"), "{turtle}");
    assert_eq!(turtle.matches("+1 555").count(), 2, "{turtle}");
    assert_eq!(summary.alternatives, 1);
    assert_eq!(summary.chosen.triples.len(), 7); // name, two links, two numbers, two kinds

    // bindings read back from JSON materialize the same
    let mut rudof2 = rudof_with(OUTPUT_SCHEMA, None);
    rudof2.shexmap_load_bindings(text.as_bytes()).unwrap();
    let mut out2 = Vec::new();
    rudof2
        .shexmap_materialize(&mut out2)
        .with_root("<http://out.example/card1>")
        .execute()
        .unwrap();
    assert_eq!(String::from_utf8(out2).unwrap().matches("+1 555").count(), 2);
}

#[test]
fn a_non_conformant_focus_is_reported_by_the_validator() {
    let mut rudof = rudof_with(INPUT_SCHEMA, Some(DATA));
    let err = rudof.shexmap_bind("<http://in.example/nobody>").execute().unwrap_err();
    assert!(err.to_string().contains("does not conform"), "{err}");
}

#[test]
fn check_reports_unbound_reads() {
    let input = shex_ast::ShExParser::parse(
        INPUT_SCHEMA,
        None,
        &rudof_iri::IriS::new_unchecked("http://in.example/schema"),
    )
    .unwrap();
    let bad_output = OUTPUT_SCHEMA.replace("v:use", "v:nowhere");
    let rudof = rudof_with(&bad_output, None);
    let report = rudof.shexmap_check(&input).execute().unwrap();
    assert!(!report.ok());
    assert!(report.errors.iter().any(|e| e.contains("v:nowhere")), "{report}");

    let rudof = rudof_with(OUTPUT_SCHEMA, None);
    let report = rudof.shexmap_check(&input).execute().unwrap();
    assert!(report.ok(), "{report}");
}

#[test]
fn static_variables_are_parsed_as_terms() {
    let output = r#"PREFIX : <http://out.example/>
PREFIX v: <http://vars.example/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX Map: <http://shex.io/extensions/Map/#>
start = @<Card>
<Card> { :fullName xsd:string %Map:{ v:name %} ; :source IRI %Map:{ v:source %} }
"#;
    let mut rudof = rudof_with(INPUT_SCHEMA, Some(DATA));
    rudof.shexmap_bind("<http://in.example/ann>").execute().unwrap();
    rudof
        .load_shex_schema(&InputSpec::Str(output.to_string()))
        .with_shex_schema_format(&ShExFormat::ShExC)
        .execute()
        .unwrap();
    let mut statics = HashMap::new();
    statics.insert(
        "http://vars.example/source".to_string(),
        "<http://src.example/crm>".to_string(),
    );
    let mut out = Vec::new();
    rudof
        .shexmap_materialize(&mut out)
        .with_root("<http://out.example/card1>")
        .with_static_vars(statics)
        .execute()
        .unwrap();
    assert!(String::from_utf8(out).unwrap().contains("src.example/crm"));
}

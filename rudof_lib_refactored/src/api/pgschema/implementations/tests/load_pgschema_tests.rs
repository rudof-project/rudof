use crate::{
    Rudof, RudofConfig,
    api::pgschema::implementations::load_pgschema::load_pgschema,
    api::pgschema::implementations::serialize_pgschema::serialize_pgschema,
    formats::{InputSpec, PgSchemaFormat},
};
use std::str::FromStr;

/// Helper: serialize current PG schema to string
fn serialize_to_string(rudof: &Rudof, format: Option<PgSchemaFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_pgschema(
        rudof,
        format.as_ref(),
        &mut buffer,
    )
    .unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_load_and_serialize_pgschema() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let pgschema_input = InputSpec::from_str(
        r#"
CREATE NODE TYPE ( AdultStudentType: Student {
    name: STRING ,
    age: INTEGER CHECK > 18
})
        "#,
    )
    .unwrap();

    load_pgschema(&mut rudof, &pgschema_input, None).unwrap();

    let serialized = serialize_to_string(&rudof, None);

    println!(
        "\n===== test_load_and_serialize_pgschema =====\n{}\n============================================",
        serialized
    );
}
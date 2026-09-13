use crate::{
    Rudof, RudofConfig,
    api::shex::implementations::compile_shex_schema_to_file::compile_shex_schema_to_file,
    api::shex::implementations::load_shex_schema::load_shex_schema,
    api::shex::implementations::load_shex_schema_precompiled::load_shex_schema_precompiled,
    api::shex::implementations::serialize_shex_schema::serialize_shex_schema,
    formats::{InputSpec, ShExFormat},
};
use std::io::Write;

/// Compiles a sample schema to a precompiled `SchemaIR` cache file and
/// returns an `InputSpec` pointing at it. The returned `NamedTempFile` must
/// be kept alive for as long as the `InputSpec` is used -- dropping it
/// deletes the file.
fn compile_sample_schema() -> (tempfile::NamedTempFile, InputSpec) {
    let mut rudof = Rudof::new(RudofConfig::default());
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer ?
           }"#,
    );
    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let mut cache = tempfile::NamedTempFile::new().unwrap();
    compile_shex_schema_to_file(&rudof, &mut cache).unwrap();
    cache.flush().unwrap();
    let input_spec = InputSpec::Path(cache.path().to_path_buf());
    (cache, input_spec)
}

fn serialize_to_string(rudof: &Rudof, format: Option<ShExFormat>) -> crate::Result<String> {
    let mut buffer = Vec::new();
    serialize_shex_schema(
        rudof,
        None,
        Some(true),
        Some(false),
        Some(false),
        Some(false),
        None,
        format.as_ref(),
        None,
        &mut buffer,
    )?;
    Ok(String::from_utf8(buffer).unwrap())
}

#[test]
fn test_load_precompiled_then_show_internal_succeeds() {
    let (_cache_file, input_spec) = compile_sample_schema();

    let mut rudof = Rudof::new(RudofConfig::default());
    load_shex_schema_precompiled(&mut rudof, &input_spec, None).unwrap();

    let internal = serialize_to_string(&rudof, Some(ShExFormat::Internal)).unwrap();
    assert!(internal.contains("PersonShape"));
}

#[test]
fn test_load_precompiled_then_convert_to_shexc_fails_with_actionable_error() {
    let (_cache_file, input_spec) = compile_sample_schema();

    let mut rudof = Rudof::new(RudofConfig::default());
    load_shex_schema_precompiled(&mut rudof, &input_spec, None).unwrap();

    let result = serialize_to_string(&rudof, Some(ShExFormat::ShExC));
    let err = result.expect_err("converting a precompiled schema to ShExC should fail: the AST isn't cached");
    let message = err.to_string();
    assert!(
        message.contains("precompiled cache"),
        "unexpected error message: {message}"
    );
    assert!(message.contains("shexc"), "unexpected error message: {message}");
}

#[test]
fn test_no_schema_loaded_still_reports_plain_error() {
    let rudof = Rudof::new(RudofConfig::default());

    let result = serialize_to_string(&rudof, Some(ShExFormat::ShExC));
    let err = result.expect_err("no schema at all should fail");
    assert!(err.to_string().contains("No ShEx schema loaded"));
}

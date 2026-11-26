//! Tests for `ShExFormat::Binary`, which unifies precompiled-cache
//! reading/writing with the same `-s`/`-f`/`-r` vocabulary used for every
//! other ShEx format, instead of requiring the dedicated
//! `--compile-to`/`--compiled-schema` flags.

use crate::{
    Rudof, RudofConfig,
    api::shex::implementations::load_shex_schema::load_shex_schema,
    api::shex::implementations::serialize_shex_schema::serialize_shex_schema,
    formats::{InputSpec, ShExFormat},
};

fn serialize_to_bytes(rudof: &Rudof, format: ShExFormat) -> crate::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    serialize_shex_schema(
        rudof,
        None,
        Some(true),
        Some(false),
        Some(false),
        Some(false),
        None,
        Some(&format),
        None,
        &mut buffer,
    )?;
    Ok(buffer)
}

fn serialize_to_string(rudof: &Rudof, format: ShExFormat) -> crate::Result<String> {
    Ok(String::from_utf8(serialize_to_bytes(rudof, format)?).unwrap())
}

#[test]
fn test_result_format_binary_writes_a_loadable_cache() {
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

    let cache_bytes = serialize_to_bytes(&rudof, ShExFormat::Binary).unwrap();
    assert!(
        cache_bytes.starts_with(b"RSIR"),
        "cache should start with the RSIR magic bytes"
    );

    let cache_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(cache_file.path(), &cache_bytes).unwrap();

    let mut loaded = Rudof::new(RudofConfig::default());
    load_shex_schema(
        &mut loaded,
        &InputSpec::Path(cache_file.path().to_path_buf()),
        Some(&ShExFormat::Binary),
        None,
        None,
    )
    .unwrap();

    let internal = serialize_to_string(&loaded, ShExFormat::Internal).unwrap();
    assert!(internal.contains("PersonShape"));
}

#[test]
fn test_schema_format_binary_round_trips_through_load_shex_schema() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }"#,
    );
    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // `-s example.shex -r binary -o example.bin`
    let cache_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(
        cache_file.path(),
        serialize_to_bytes(&rudof, ShExFormat::Binary).unwrap(),
    )
    .unwrap();

    // `-f binary -s example.bin`
    let mut loaded = Rudof::new(RudofConfig::default());
    load_shex_schema(
        &mut loaded,
        &InputSpec::Path(cache_file.path().to_path_buf()),
        Some(&ShExFormat::Binary),
        None,
        None,
    )
    .unwrap();

    // Only `internal` (and statistics/dependencies) are recoverable -- the AST
    // isn't part of the cache.
    assert!(
        serialize_to_string(&loaded, ShExFormat::Internal)
            .unwrap()
            .contains("PersonShape")
    );
    let shexc_err = serialize_to_string(&loaded, ShExFormat::ShExC).unwrap_err();
    assert!(shexc_err.to_string().contains("precompiled cache"));
}

#[test]
fn test_binary_format_round_trips_the_same_as_dedicated_flags() {
    use crate::api::shex::implementations::compile_shex_schema_to_file::compile_shex_schema_to_file;
    use crate::api::shex::implementations::load_shex_schema_precompiled::load_shex_schema_precompiled;

    let mut rudof = Rudof::new(RudofConfig::default());
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }"#,
    );
    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Old, dedicated-flag path (`--compile-to`).
    let mut via_compile_to = Vec::new();
    compile_shex_schema_to_file(&rudof, &mut via_compile_to).unwrap();

    // New, unified-format path (`-r binary`).
    let via_result_format = serialize_to_bytes(&rudof, ShExFormat::Binary).unwrap();

    assert_eq!(via_compile_to, via_result_format);

    // Both caches load identically through their respective entry points
    // (`--compiled-schema` vs. `-f binary`).
    let cache_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(cache_file.path(), &via_result_format).unwrap();
    let input_spec = InputSpec::Path(cache_file.path().to_path_buf());

    let mut loaded_via_compiled_schema = Rudof::new(RudofConfig::default());
    load_shex_schema_precompiled(&mut loaded_via_compiled_schema, &input_spec, None).unwrap();

    let mut loaded_via_schema_format = Rudof::new(RudofConfig::default());
    load_shex_schema(
        &mut loaded_via_schema_format,
        &input_spec,
        Some(&ShExFormat::Binary),
        None,
        None,
    )
    .unwrap();

    assert_eq!(
        serialize_to_string(&loaded_via_compiled_schema, ShExFormat::Internal).unwrap(),
        serialize_to_string(&loaded_via_schema_format, ShExFormat::Internal).unwrap()
    );
}

//! Regression tests for `shex`'s `binary` `ShExFormat`:
//! - `-r binary -o FILE` (and `--compile-to FILE`) reports how many shapes
//!   were saved, since writing raw cache bytes to the terminal/file leaves no
//!   other visible confirmation.
//! - `--no-show-schema` used to be silently ignored (a `ShExFormatCli`
//!   parsing/wiring bug unrelated to `binary` itself, but only visible once
//!   `binary` gave people a reason to load a schema with no default-format
//!   textual serialization available), so `-f binary -s FILE --no-show-schema`
//!   used to fail even though the user explicitly asked not to render it.

#![cfg(not(target_family = "wasm"))]

use std::path::Path;
use std::process::{Command, Stdio};

struct Output {
    stdout: String,
    stderr: String,
    code: i32,
}

fn rudof_in(dir: &Path, args: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_rudof"))
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rudof")
        .wait_with_output()
        .expect("failed to wait for rudof");

    Output {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        code: output.status.code().unwrap_or(-1),
    }
}

const SCHEMA_SHEX: &str = r#"PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:PersonShape {
  :name xsd:string
}
"#;

#[test]
fn result_format_binary_reports_shapes_saved() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    std::fs::write(dir.path().join("schema.shex"), SCHEMA_SHEX).expect("failed to write schema fixture");

    let out = rudof_in(
        dir.path(),
        &["shex", "-s", "schema.shex", "-r", "binary", "-o", "schema.bin"],
    );

    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(
        dir.path().join("schema.bin").exists(),
        "expected schema.bin to be written"
    );
    assert!(
        out.stderr.contains("shape(s) saved in schema.bin"),
        "expected a confirmation message on stderr, got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
    // The confirmation must not have leaked into the binary payload itself.
    assert!(
        out.stdout.is_empty(),
        "expected the cache bytes to go to the file, not stdout: {:?}",
        out.stdout
    );
}

#[test]
fn compile_to_also_reports_shapes_saved() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    std::fs::write(dir.path().join("schema.shex"), SCHEMA_SHEX).expect("failed to write schema fixture");

    let out = rudof_in(dir.path(), &["shex", "-s", "schema.shex", "--compile-to", "schema.bin"]);

    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(
        out.stderr.contains("shape(s) saved in schema.bin"),
        "expected a confirmation message on stderr, got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
}

#[test]
fn loading_binary_schema_with_no_show_schema_succeeds() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    std::fs::write(dir.path().join("schema.shex"), SCHEMA_SHEX).expect("failed to write schema fixture");
    let compile = rudof_in(
        dir.path(),
        &["shex", "-s", "schema.shex", "-r", "binary", "-o", "schema.bin"],
    );
    assert_eq!(compile.code, 0, "setup: failed to compile schema.bin");

    // Regression: this used to fail with "No ShEx schema loaded" /
    // "requires the original parsed ShEx schema" because `--no-show-schema`
    // was silently ignored, so `shex` still tried (and failed) to render the
    // schema in the default `shexc` result format even though the user
    // explicitly asked not to show it.
    let out = rudof_in(
        dir.path(),
        &["shex", "-s", "schema.bin", "-f", "binary", "--no-show-schema"],
    );

    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(
        out.stdout.is_empty(),
        "expected no schema output, got stdout:\n{}",
        out.stdout
    );
}

#[test]
fn loading_binary_schema_without_no_show_schema_still_reports_the_ast_limitation() {
    // Sanity check for the test above: without `--no-show-schema`, the
    // default `shexc` result format still (correctly) fails, since the AST
    // isn't recoverable from a precompiled cache. This guards against a
    // fix for `--no-show-schema` accidentally making `show_schema` always
    // false.
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    std::fs::write(dir.path().join("schema.shex"), SCHEMA_SHEX).expect("failed to write schema fixture");
    let compile = rudof_in(
        dir.path(),
        &["shex", "-s", "schema.shex", "-r", "binary", "-o", "schema.bin"],
    );
    assert_eq!(compile.code, 0, "setup: failed to compile schema.bin");

    let out = rudof_in(dir.path(), &["shex", "-s", "schema.bin", "-f", "binary"]);

    assert_ne!(
        out.code, 0,
        "expected the default shexc result format to fail without --no-show-schema"
    );
    assert!(
        out.stderr.contains("precompiled cache"),
        "expected the actionable precompiled-cache error, got stderr:\n{}",
        out.stderr
    );
}

#[test]
fn no_show_schema_suppresses_a_regular_shexc_schema_too() {
    // Not binary-specific: `--no-show-schema` should suppress output for any
    // schema/format, not just ones loaded via `binary`.
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    std::fs::write(dir.path().join("schema.shex"), SCHEMA_SHEX).expect("failed to write schema fixture");

    let out = rudof_in(dir.path(), &["shex", "-s", "schema.shex", "--no-show-schema"]);

    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(
        out.stdout.is_empty(),
        "expected no schema output, got stdout:\n{}",
        out.stdout
    );
}

#[test]
fn a_later_show_schema_overrides_an_earlier_no_show_schema() {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    std::fs::write(dir.path().join("schema.shex"), SCHEMA_SHEX).expect("failed to write schema fixture");

    let out = rudof_in(
        dir.path(),
        &["shex", "-s", "schema.shex", "--no-show-schema", "--show-schema"],
    );

    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(
        out.stdout.contains("PersonShape"),
        "expected the schema to be shown, got stdout:\n{}",
        out.stdout
    );
}

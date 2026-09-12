//! Regression test for the `convert` command's `-t`/`--target-folder` flag.
//!
//! `commands/convert.rs` used to forward `-o`/`--output-file` (a single-file
//! flag shared by every command) into the HTML backend's output-folder
//! parameter, while silently ignoring the dedicated `-t`/`--target-folder`
//! flag. That made `-t` a no-op and produced "Output folder must be
//! specified for HTML conversion" even when `-t` was passed.

#![cfg(not(target_family = "wasm"))]

use std::path::Path;
use std::process::{Command, Stdio};

struct Output {
    stdout: String,
    stderr: String,
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
    }
}

const SCHEMA_SHEX: &str = r#"PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:PersonShape {
  :name xsd:string
}
"#;

#[test]
fn convert_to_html_accepts_target_folder_flag() {
    let dir = tempfile::tempdir().expect("failed to create temp dir for convert test");
    std::fs::write(dir.path().join("schema.shex"), SCHEMA_SHEX).expect("failed to write schema fixture");

    let out = rudof_in(
        dir.path(),
        &["convert", "-s", "schema.shex", "-m", "shex", "-x", "html", "-t", "html_output"],
    );

    // Whether or not this environment has PlantUML available for diagram
    // rendering, `-t` must be recognized as the output folder: the specific
    // "must be specified" error is what regresses if `-t` gets ignored again.
    assert!(
        !out.stderr.contains("Output folder must be specified"),
        "expected -t/--target-folder to be honored, got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
}

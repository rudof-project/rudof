//! End-to-end tests of the `shexmap` command on the ShExMap examples in `examples/shexmap`:
//! bind and materialize in one step, write and read bindings, check a schema pair, update a
//! graph in place, and write provenance.

#![cfg(not(target_family = "wasm"))]

use std::path::{Path, PathBuf};
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

fn examples() -> PathBuf {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/shexmap/shexjs")).to_path_buf()
}

const BASE: &str = "http://a.example/schema/";

#[test]
fn maps_blood_pressure_from_fhir_to_dam() {
    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "BPfhir-instance.ttl",
            "-s",
            "BPfhir-schema.shex",
            "-n",
            "<tag:BPfhir123>",
            "-O",
            "BPdam-schema.shex",
            "--root",
            "<tag:b0>",
            "--output-shape",
            "BPunitsDAM",
            "--base-schema",
            BASE,
        ],
    );
    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(out.stdout.contains("<tag:b0>"), "{}", out.stdout);
    assert!(out.stdout.contains("systolic"), "{}", out.stdout);
    assert!(out.stdout.contains("\"110\"^^"), "{}", out.stdout);
    assert!(out.stdout.contains("mmHg"), "{}", out.stdout);
}

#[test]
fn bindings_round_trip_through_a_file() {
    let dir = tempfile::tempdir().unwrap();
    let bindings = dir.path().join("bindings.json");
    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "BPfhir-instance.ttl",
            "-s",
            "BPfhir-schema.shex",
            "-n",
            "<tag:BPfhir123>",
            "-b",
            bindings.to_str().unwrap(),
        ],
    );
    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    let text = std::fs::read_to_string(&bindings).unwrap();
    assert!(text.contains("http://shex.io/extensions/Map/#BPDAM-sysVal"), "{text}");

    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "-j",
            bindings.to_str().unwrap(),
            "-O",
            "BPdam-schema.shex",
            "--root",
            "<tag:b0>",
            "--output-shape",
            "BPunitsDAM",
            "--base-schema",
            BASE,
            "-r",
            "ntriples",
        ],
    );
    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert_eq!(
        out.stdout.lines().filter(|l| !l.trim().is_empty()).count(),
        7,
        "{}",
        out.stdout
    );
}

/// The inline `outputSchema` of a manifest entry, written to a file.
fn inline_output_schema(schema_label: &str, dir: &Path) -> PathBuf {
    let text = std::fs::read_to_string(examples().join("manifest.json")).unwrap();
    let entries: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
    let entry = entries
        .iter()
        .find(|e| e["schemaLabel"] == schema_label)
        .unwrap_or_else(|| panic!("no entry {schema_label}"));
    let path = dir.join("out.shex");
    std::fs::write(&path, entry["outputSchema"].as_str().unwrap()).unwrap();
    path
}

#[test]
fn shexjs_bindings_materialize() {
    // bindings JSON written by shex.js, with two iterations of a repeated constraint
    let dir = tempfile::tempdir().unwrap();
    let schema = inline_output_schema("BPPatient multi-bindings", dir.path());
    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "-j",
            "BPPatient-multi-bindings-bindings.json",
            "-O",
            schema.to_str().unwrap(),
            "--root",
            "<tag:BPfhir123>",
            "--output-shape",
            "collector",
            "--base-schema",
            BASE,
            "-r",
            "ntriples",
        ],
    );
    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert_eq!(out.stdout.matches("fhir-rdf/item>").count(), 2, "{}", out.stdout);
    assert_eq!(
        out.stdout.matches("\"110\"^^").count() + out.stdout.matches("\"111\"^^").count(),
        2,
        "{}",
        out.stdout
    );
}

#[test]
fn check_reports_a_coherent_pair_and_an_incoherent_one() {
    // without the static variable the output reads, the pair does not map
    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "--check",
            "-s",
            "BPfhir-schema.shex",
            "-O",
            "BPdam-schema.shex",
            "--output-shape",
            "BPunitsDAM",
            "--base-schema",
            BASE,
        ],
    );
    assert_ne!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(out.stdout.contains("someConstant"), "{}", out.stdout);

    // with it, the pair maps coherently
    let dir = tempfile::tempdir().unwrap();
    let statics = dir.path().join("statics.json");
    std::fs::write(&statics, r#"{"http://abc.example/someConstant": "\"123-456\""}"#).unwrap();
    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "--check",
            "-s",
            "BPfhir-schema.shex",
            "-O",
            "BPdam-schema.shex",
            "--output-shape",
            "BPunitsDAM",
            "--base-schema",
            BASE,
            "--static",
            statics.to_str().unwrap(),
        ],
    );
    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(out.stdout.contains("ok"), "{}", out.stdout);

    // a card schema reads :name, :tel and :email, which the blood-pressure schema never binds
    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "--check",
            "-s",
            "BPfhir-schema.shex",
            "-O",
            "card-flat-schema.shex",
            "--base-schema",
            BASE,
        ],
    );
    assert_ne!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    assert!(
        out.stdout
            .contains("error: card:fullName reads :name, which the input schema never binds"),
        "{}",
        out.stdout
    );
    assert!(out.stdout.contains("warning: bp:sysVal is bound"), "{}", out.stdout);
}

#[test]
fn update_in_place_replaces_what_the_schema_holds() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("dam.ttl");
    // a graph with an ungoverned triple at the root: nothing to replace yet
    std::fs::write(
        &target,
        "PREFIX : <http://dam.example/med#>\n<tag:b0> :note \"kept\" .\n",
    )
    .unwrap();
    let common = [
        "shexmap",
        "-O",
        "BPdam-schema.shex",
        "--root",
        "<tag:b0>",
        "--output-shape",
        "BPunitsDAM",
        "--base-schema",
        BASE,
        "--into",
        target.to_str().unwrap(),
    ];
    let mut first: Vec<&str> = common.to_vec();
    first.extend([
        "BPfhir-instance.ttl",
        "-s",
        "BPfhir-schema.shex",
        "-n",
        "<tag:BPfhir123>",
    ]);
    let out = rudof_in(&examples(), &first);
    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    let text = std::fs::read_to_string(&target).unwrap();
    assert!(text.contains("kept"), "{text}");
    assert!(text.contains("Walker, Alice"), "{text}");
    assert!(text.contains("systolic"), "{text}");
    assert!(out.stderr.contains("7 triples added, 0 removed"), "{}", out.stderr);

    // the same bindings with another given name: the name is replaced, the rest kept
    let bindings = dir.path().join("bindings.json");
    let json = std::fs::read_to_string(examples().join("BP-simple-bindings.json"))
        .unwrap()
        .replace("\"Alice\"", "\"Alicia\"");
    std::fs::write(&bindings, json).unwrap();
    let mut second: Vec<&str> = common.to_vec();
    second.extend(["-j", bindings.to_str().unwrap()]);
    let out = rudof_in(&examples(), &second);
    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    let text = std::fs::read_to_string(&target).unwrap();
    assert!(text.contains("kept"), "{text}");
    assert!(
        text.contains("Walker, Alicia") && !text.contains("Walker, Alice\""),
        "{text}"
    );
    assert!(text.contains("systolic") && text.contains("\"110\"^^"), "{text}");
    // the serializer relabels the readings' blank nodes, so they count as replaced as well
    assert!(out.stderr.contains("7 triples added, 7 removed"), "{}", out.stderr);
    assert_eq!(text.matches(":value").count(), 2, "{text}");
}

#[test]
fn provenance_is_written_per_triple() {
    let dir = tempfile::tempdir().unwrap();
    let prov = dir.path().join("prov.jsonl");
    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "BPfhir-instance.ttl",
            "-s",
            "BPfhir-schema.shex",
            "-n",
            "<tag:BPfhir123>",
            "-O",
            "BPdam-schema.shex",
            "--root",
            "<tag:b0>",
            "--output-shape",
            "BPunitsDAM",
            "--base-schema",
            BASE,
            "--provenance",
            prov.to_str().unwrap(),
            "-r",
            "ntriples",
        ],
    );
    assert_eq!(out.code, 0, "stdout:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    let text = std::fs::read_to_string(&prov).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), out.stdout.lines().filter(|l| !l.trim().is_empty()).count());
    assert!(lines.iter().any(|l| l.contains("\"kind\":\"variable\"")), "{text}");
    assert!(lines.iter().any(|l| l.contains("\"kind\":\"structural\"")), "{text}");
}

#[test]
fn a_non_conformant_node_fails_with_the_validators_reason() {
    let out = rudof_in(
        &examples(),
        &[
            "shexmap",
            "BPfhir-instance.ttl",
            "-s",
            "BPfhir-schema.shex",
            "-n",
            "<tag:nobody>",
            "-O",
            "BPdam-schema.shex",
            "--base-schema",
            BASE,
        ],
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("does not conform"), "{}", out.stderr);
}

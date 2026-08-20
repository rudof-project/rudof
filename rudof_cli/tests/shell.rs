//! Integration tests for the `rudof shell` interactive REPL.
//!
//! These spawn the actual `rudof` binary with `shell` as an argument and
//! drive it over stdin/stdout, the same way a user would, since the
//! interesting behavior here (the readline loop, command dispatch, and
//! session state persisting across commands) only shows up end-to-end.

#[cfg(not(target_family = "wasm"))]
use std::io::Write as _;
#[cfg(not(target_family = "wasm"))]
use std::path::Path;
#[cfg(not(target_family = "wasm"))]
use std::process::{Command, Stdio};

#[cfg(not(target_family = "wasm"))]
struct ShellOutput {
    stdout: String,
    stderr: String,
    code: i32,
}

/// Runs `rudof shell`, feeding it `input` on stdin, and returns its
/// captured stdout/stderr and exit code.
///
/// Each call gets its own `HOME` (a fresh temp dir) so the shell's
/// `~/.rudof_history` file never touches the real home directory or
/// collides with other tests running in parallel.
#[cfg(not(target_family = "wasm"))]
fn run_shell(input: &str) -> ShellOutput {
    let home = tempfile::tempdir().expect("failed to create temp HOME for shell test");

    let mut child = Command::new(env!("CARGO_BIN_EXE_rudof"))
        .arg("shell")
        .env("HOME", home.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn `rudof shell`");

    child
        .stdin
        .take()
        .expect("child stdin was not piped")
        .write_all(input.as_bytes())
        .expect("failed to write to `rudof shell` stdin");

    let output = child.wait_with_output().expect("failed to wait for `rudof shell`");

    ShellOutput {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        code: output.status.code().unwrap_or(-1),
    }
}

#[cfg(not(target_family = "wasm"))]
fn fixture(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_prints_banner_and_exits_cleanly_on_exit() {
    let out = run_shell("exit\n");
    // The ASCII art banner is printed on startup, before the tip line.
    assert!(out.stdout.contains(r"|_|    \____|\____|\___/|_|"));
    assert!(
        out.stdout
            .contains("Type 'help' for available commands, 'exit' to quit.")
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_quit_is_an_alias_for_exit() {
    let out = run_shell("quit\n");
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_ends_cleanly_on_eof_without_exit() {
    // Closing stdin without ever typing `exit` should still terminate the loop.
    let out = run_shell("");
    assert!(out.stdout.contains(r"|_|    \____|\____|\___/|_|"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_help_lists_top_level_commands_and_shell_builtins() {
    let out = run_shell("help\nexit\n");
    assert!(out.stdout.contains("Usage: rudof <COMMAND>"));
    assert!(out.stdout.contains("Shell-only commands:"));
    assert!(out.stdout.contains("exit, quit"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_unknown_subcommand_reports_error_and_keeps_session_alive() {
    let out = run_shell("nonexistent-command\nhelp\nexit\n");
    assert!(out.stdout.contains("unrecognized subcommand"));
    // The session kept going after the bad command: `help` still ran.
    assert!(out.stdout.contains("Shell-only commands:"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_rejects_unterminated_quotes_without_ending_the_session() {
    let out = run_shell("data \"unterminated\nhelp\nexit\n");
    assert!(out.stderr.contains("check quoting"));
    assert!(out.stdout.contains("Shell-only commands:"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_typing_shell_again_does_not_recurse() {
    let out = run_shell("shell\nexit\n");
    assert!(out.stdout.contains("Already inside the shell."));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_bare_data_without_prior_load_reports_no_data_loaded() {
    let out = run_shell("data\nexit\n");
    assert!(out.stderr.contains("No data loaded"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_persists_loaded_data_across_commands() {
    let data = fixture("shell_data.ttl");
    let script = format!("data {data}\ndata\nexit\n");
    let out = run_shell(&script);

    // The second, bare `data` call re-shows the data loaded by the first
    // call, without reloading it from disk.
    let occurrences = out.stdout.matches("Alice").count();
    assert_eq!(
        occurrences, 2,
        "expected the loaded data to be shown twice, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_bare_dctap_without_prior_load_reports_no_dctap_loaded() {
    let out = run_shell("dctap\nexit\n");
    assert!(out.stderr.contains("No DCTap data loaded"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_persists_loaded_dctap_across_commands() {
    let dctap = fixture("shell_dctap.csv");
    let script = format!("dctap -s {dctap}\ndctap\nexit\n");
    let out = run_shell(&script);

    // The second, bare `dctap` call re-shows the DCTap loaded by the first
    // call, without reloading it from disk. `xsd:string` appears exactly
    // once per render (unlike `Person`, which appears twice by itself).
    let occurrences = out.stdout.matches("xsd:string").count();
    assert_eq!(
        occurrences, 2,
        "expected the loaded DCTap to be shown twice, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_bare_service_without_prior_load_reports_no_service_description() {
    let out = run_shell("service\nexit\n");
    assert!(out.stderr.contains("No Service Description loaded"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_persists_loaded_service_description_across_commands() {
    let service = fixture("shell_service.ttl");
    let script = format!("service -s {service}\nservice\nexit\n");
    let out = run_shell(&script);

    // The second, bare `service` call re-shows the service description
    // loaded by the first call, without reloading it from disk.
    let occurrences = out.stdout.matches("SPARQL11Query").count();
    assert_eq!(
        occurrences, 2,
        "expected the loaded service description to be shown twice, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_output_flag_redirects_to_file_instead_of_printing() {
    let out_dir = tempfile::tempdir().expect("failed to create temp output dir");
    let out_file = out_dir.path().join("data.ttl");
    let data = fixture("shell_data.ttl");

    let script = format!("data {data} -o {}\nexit\n", out_file.display());
    let out = run_shell(&script);

    assert!(out_file.exists(), "expected {} to be created", out_file.display());
    let written = std::fs::read_to_string(&out_file).expect("failed to read redirected output file");
    assert!(
        written.contains("Alice"),
        "expected the data in the output file, got:\n{written}"
    );

    // The command's actual output went to the file, not the terminal.
    assert!(
        !out.stdout.contains("Alice"),
        "did not expect the data to be printed to stdout, got:\n{}",
        out.stdout
    );
    assert!(
        out.stdout.contains(&format!("Output saved in {}", out_file.display())),
        "expected a confirmation message naming the output file, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_output_flag_dash_still_prints_to_terminal() {
    // `-o -` is the standard Unix convention for "write to stdout"; it must
    // not be treated as a file redirection (no file created, no confirmation
    // message — the data prints normally, same as with no `-o` at all).
    let data = fixture("shell_data.ttl");
    let script = format!("data {data} -o -\nexit\n");
    let out = run_shell(&script);

    assert!(out.stdout.contains("Alice"));
    assert!(!out.stdout.contains("Output saved in"));
    assert_eq!(out.code, 0);
}

// ============================================================================
// Cross-command state reuse: commands that consume multiple resources
// (data, a ShEx/SHACL/PGSchema schema, a shapemap/typemap) should be able to
// omit args for pieces that were already loaded by an earlier command in the
// same shell session, instead of requiring everything to be re-specified.
// ============================================================================

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_shex_validate_without_any_state_reports_clear_error() {
    let out = run_shell("shex-validate\nexit\n");
    assert!(out.stderr.contains("No data loaded"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_shex_validate_reuses_data_schema_and_shapemap_loaded_separately() {
    let data = fixture("shell_data.ttl");
    let schema = fixture("shell_schema.shex");
    let shapemap = fixture("shell_shapemap.sm");

    let script = format!("data {data}\nshex -s {schema}\nshapemap -m {shapemap}\nshex-validate\nexit\n");
    let out = run_shell(&script);

    // "Shape passed" only appears in shex-validate's own validation report,
    // not in the `data`/`shex`/`shapemap` commands' echoes of what they
    // each loaded — so this can't pass just because an earlier command
    // happened to print matching text.
    assert!(
        out.stdout.contains("Shape passed"),
        "expected a validation report reusing the separately-loaded data/schema/shapemap, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_shacl_validate_without_any_state_reports_clear_error() {
    let out = run_shell("shacl-validate\nexit\n");
    assert!(out.stderr.contains("No data loaded"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_shacl_validate_reuses_data_and_shapes_loaded_separately() {
    let data = fixture("shell_data.ttl");
    let shapes = fixture("shell_shapes.ttl");

    let script = format!("data {data}\nshacl -s {shapes}\nshacl-validate\nexit\n");
    let out = run_shell(&script);

    assert!(
        out.stdout.contains("No Errors found") || out.stdout.contains("Errors"),
        "expected a SHACL validation report reusing the separately-loaded data/shapes, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_node_reuses_data_loaded_separately() {
    let data = fixture("shell_data.ttl");
    let script = format!("data {data}\nnode -n :alice\nexit\n");
    let out = run_shell(&script);

    // "Outgoing arcs" only appears in `node`'s own report, not in `data`'s
    // echo of what it loaded.
    assert!(
        out.stdout.contains("Outgoing arcs"),
        "expected `node` to reuse the separately-loaded data, got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_materialize_reuses_schema_loaded_separately() {
    let schema = fixture("shell_schema.shex");
    let script = format!("shex -s {schema}\nmaterialize\nexit\n");
    let out = run_shell(&script);

    // The schema is reused (no "required arguments" clap error); the only
    // thing still missing is the MapState, which nothing else loaded here.
    assert!(
        out.stderr.contains("No MapState available"),
        "expected materialize to reuse the separately-loaded schema and fail only on missing MapState, got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_pgschema_validate_reuses_schema_typemap_and_data_across_calls() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("rudof_cli has a parent directory");
    let pgs = workspace_root.join("examples/property_graphs/demo.pgs");
    let map = workspace_root.join("examples/property_graphs/demo.map");
    let pg = workspace_root.join("examples/property_graphs/demo.pg");

    let script = format!(
        "pgschema-validate -s {} -m {} {}\npgschema-validate\nexit\n",
        pgs.display(),
        map.display(),
        pg.display()
    );
    let out = run_shell(&script);

    // The second, bare call re-validates by reusing the schema, typemap and
    // data loaded by the first call, so the same report appears twice.
    let occurrences = out.stdout.matches("PersonType").count();
    assert_eq!(
        occurrences, 2,
        "expected the validation report to appear twice, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

// ============================================================================
// Shell-only convenience: `<command> <file>` (a single bare positional, with
// nothing else on the line) is accepted as an alternative to `<command> -s
// <file>` for commands whose primary resource flag is otherwise required.
// This only applies inside the shell, and only when the line is unambiguous
// (exactly one token besides the command name) — the top-level CLI and any
// line combining the bare file with other flags are unaffected.
// ============================================================================

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_shex_accepts_bare_file_as_schema() {
    let schema = fixture("shell_schema.shex");
    let out = run_shell(&format!("shex {schema}\nexit\n"));

    assert!(
        out.stdout.contains("PersonShape"),
        "expected the bare file to be loaded as the schema, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_shacl_accepts_bare_file_as_shapes_not_data() {
    let shapes = fixture("shell_shapes.ttl");
    let out = run_shell(&format!("shacl {shapes}\ndata\nexit\n"));

    assert!(
        out.stdout.contains("SHACL shapes graph IR"),
        "expected the bare file to be loaded as shapes, got:\n{}",
        out.stdout
    );
    // Loading it as shapes must not also set it as the session's RDF data —
    // the follow-up bare `data` call should still report nothing loaded.
    assert!(
        out.stderr.contains("No data loaded"),
        "expected the shapes file to not be treated as RDF data, got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_bare_file_convenience_does_not_apply_alongside_other_flags() {
    // Combining the bare file with another flag is ambiguous, so it's left
    // to clap's normal parsing (and normal error) rather than guessed at.
    let schema = fixture("shell_schema.shex");
    let out = run_shell(&format!("shex {schema} -f shexc\nexit\n"));

    assert!(
        out.stdout.contains("unrecognized") || out.stdout.contains("unexpected argument"),
        "expected a clap parse error, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

// ============================================================================
// `!` escapes to the system shell, e.g. `! ls` or `!code file.shex`.
// ============================================================================

#[cfg(not(target_family = "wasm"))]
#[cfg(unix)]
#[test]
fn shell_bang_with_space_runs_external_command() {
    let out = run_shell("! echo hello-from-outside\nexit\n");
    assert!(
        out.stdout.contains("hello-from-outside"),
        "expected the external command's output in stdout, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[cfg(unix)]
#[test]
fn shell_bang_without_space_runs_external_command() {
    let out = run_shell("!echo hello-from-outside\nexit\n");
    assert!(
        out.stdout.contains("hello-from-outside"),
        "expected the external command's output in stdout, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[cfg(unix)]
#[test]
fn shell_bang_uses_a_real_shell_so_pipes_work() {
    let out = run_shell("!echo hello | tr a-z A-Z\nexit\n");
    assert!(
        out.stdout.contains("HELLO"),
        "expected shell-interpreted output (pipe), got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_bang_alone_reports_no_command_given() {
    let out = run_shell("!\nexit\n");
    assert!(
        out.stdout.contains("No command given after '!'"),
        "expected a hint about the missing command, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[cfg(unix)]
#[test]
fn shell_bang_failing_command_does_not_end_the_session() {
    let out = run_shell("!false\nhelp\nexit\n");
    assert!(
        out.stdout.contains("Shell-only commands:"),
        "expected the session to keep going after a failing external command, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_node_accepts_bare_identifier_as_node_flag() {
    let data = fixture("shell_data.ttl");
    let out = run_shell(&format!("data {data}\nnode :alice\nexit\n"));

    assert!(
        out.stdout.contains("Outgoing arcs"),
        "expected `node :alice` to behave like `node -n :alice`, got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
    assert_eq!(out.code, 0);
}

// ============================================================================
// `endpoint [NAME]` — shell-only command to show or activate a registered
// SPARQL endpoint for the rest of the session.
// ============================================================================

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_endpoint_with_no_active_endpoint_points_at_the_command() {
    let out = run_shell("endpoint\nexit\n");
    assert!(out.stdout.contains("No endpoint is currently active."));
    assert!(out.stdout.contains("Use 'endpoint NAME' to activate one."));
    // The default config registers these endpoints out of the box.
    assert!(out.stdout.contains("Registered endpoints:"));
    assert!(out.stdout.contains("wikidata"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_endpoint_activates_a_registered_endpoint_and_reports_it_back() {
    let out = run_shell("endpoint wikidata\nexit\n");
    assert!(
        out.stdout
            .contains("Active endpoint: wikidata (https://query.wikidata.org/sparql)")
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_endpoint_bare_reports_the_previously_activated_endpoint() {
    let out = run_shell("endpoint wikidata\nendpoint\nexit\n");
    let occurrences = out
        .stdout
        .matches("Active endpoint: wikidata (https://query.wikidata.org/sparql)")
        .count();
    assert_eq!(
        occurrences, 2,
        "expected the activation and the bare follow-up to both report it, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_endpoint_rejects_unknown_name() {
    let out = run_shell("endpoint not-a-real-endpoint\nexit\n");
    assert!(out.stdout.contains("Unknown endpoint 'not-a-real-endpoint'."));
    assert!(out.stdout.contains("Registered endpoints:"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_endpoint_rejects_too_many_arguments() {
    let out = run_shell("endpoint wikidata dbpedia\nexit\n");
    assert!(out.stdout.contains("Usage: endpoint [NAME]"));
    assert_eq!(out.code, 0);
}

// ============================================================================
// `reset [TARGET...]` — shell-only command to clear session state, either
// everything (bare `reset`/`reset all`) or one or more named pieces.
// ============================================================================

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_reset_target_clears_only_that_state() {
    let data = fixture("shell_data.ttl");
    let schema = fixture("shell_schema.shex");
    let out = run_shell(&format!("data {data}\nshex {schema}\nreset shex\nshex\ndata\nexit\n"));

    // `shex` errors after the reset (schema gone)...
    assert!(
        out.stderr.contains("No ShEx schema loaded"),
        "expected the schema to be gone after 'reset shex', got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
    // ...but `data`, never targeted, still shows what was loaded.
    assert!(
        out.stdout.contains("Alice"),
        "expected 'reset shex' to leave data untouched, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_reset_shex_also_drops_the_compiled_validator() {
    // Regression test: loading a shapemap only used to check `shex_validator`,
    // not the schema itself, so a stale validator left behind by an incomplete
    // reset let this silently succeed against a schema that was supposedly gone.
    let data = fixture("shell_data.ttl");
    let schema = fixture("shell_schema.shex");
    let shapemap = fixture("shell_shapemap.sm");
    let out = run_shell(&format!(
        "data {data}\nshex {schema}\nreset shex\nshapemap -m {shapemap}\nexit\n"
    ));

    assert!(
        out.stderr.contains("No ShEx schema loaded"),
        "expected loading a shapemap to fail once the schema (and its compiled validator) were reset, got stdout:\n{}\nstderr:\n{}",
        out.stdout,
        out.stderr
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_reset_bare_clears_everything() {
    let data = fixture("shell_data.ttl");
    let out = run_shell(&format!(
        "data {data}\nendpoint wikidata\nreset\ndata\nendpoint\nexit\n"
    ));

    assert!(out.stderr.contains("No data loaded"));
    assert!(out.stdout.contains("No endpoint is currently active."));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_reset_all_is_an_alias_for_bare_reset() {
    let data = fixture("shell_data.ttl");
    let out = run_shell(&format!("data {data}\nreset all\ndata\nexit\n"));

    assert!(out.stdout.contains("Reset all session state."));
    assert!(out.stderr.contains("No data loaded"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_reset_accepts_multiple_targets() {
    let data = fixture("shell_data.ttl");
    let schema = fixture("shell_schema.shex");
    let out = run_shell(&format!(
        "data {data}\nshex {schema}\nreset data shex\ndata\nshex\nexit\n"
    ));

    assert!(out.stdout.contains("Reset: data, shex."));
    assert!(out.stderr.contains("No data loaded"));
    assert!(out.stderr.contains("No ShEx schema loaded"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_reset_rejects_unknown_target() {
    let out = run_shell("reset bogus\nexit\n");
    assert!(out.stdout.contains("Unknown reset target(s): bogus."));
    assert!(out.stdout.contains("Valid targets:"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_reset_endpoint_clears_the_active_endpoint() {
    let schema = fixture("shell_schema.shex");
    let out = run_shell(&format!(
        "endpoint wikidata\nreset endpoint\nendpoint\nshex {schema}\nshex\nexit\n"
    ));

    assert!(out.stdout.contains("No endpoint is currently active."));
    // A target unrelated to `endpoint` is untouched by `reset endpoint`.
    assert!(out.stdout.contains("PersonShape"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_node_bare_identifier_convenience_does_not_apply_alongside_other_flags() {
    // With another flag present, `:alice` is left to normal parsing, where
    // it falls to the (real) `data` positional and fails as a bad file
    // path — not silently reinterpreted as the node to inspect.
    let data = fixture("shell_data.ttl");
    let out = run_shell(&format!("data {data}\nnode :alice -d 2\nexit\n"));

    assert!(
        out.stdout.contains("does not exist"),
        "expected a clap parse error when combined with another flag, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

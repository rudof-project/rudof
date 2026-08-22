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
fn shell_treats_unterminated_quote_as_multiline_continuation() {
    // An open quote (e.g. a SPARQL query passed inline to `-q`) makes the
    // shell keep reading further lines as part of the same command instead
    // of dispatching or erroring immediately, so a query can be typed
    // across several lines at the prompt.
    let data = fixture("shell_data.ttl");
    let script = format!("query -q 'SELECT ?s ?p ?o WHERE {{\n?s ?p ?o .\n}}' {data}\nexit\n");
    let out = run_shell(&script);
    assert!(
        out.stdout.contains("Alice"),
        "expected query results, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_discards_unterminated_quote_left_open_at_eof() {
    // If stdin closes while still inside an open quote, the pending
    // command is discarded with a warning instead of being misparsed —
    // and the lines absorbed as continuation input (`help`, `exit`) never
    // ran as commands in their own right.
    let out = run_shell("data \"unterminated\nhelp\nexit\n");
    assert!(out.stderr.contains("unterminated quote"));
    assert!(!out.stdout.contains("Shell-only commands:"));
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

    // The first call prints a stats line for the data it just loaded; the
    // second, bare call has nothing new to load, so it re-shows the full
    // loaded data instead (not another stats line).
    assert_eq!(out.stdout.matches("2 triple(s) loaded").count(), 1);
    assert_eq!(
        out.stdout.matches("Alice").count(),
        1,
        "expected the second, bare 'data' call to re-show the full loaded data, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_data_replaces_by_default_instead_of_merging() {
    let data1 = fixture("shell_data.ttl");
    let data2 = fixture("shell_data2.ttl");
    let script = format!("data {data1}\ndata {data2}\ndata\nexit\n");
    let out = run_shell(&script);

    // The second `data FILE` call, with no `--merge`, replaces the first
    // file's data rather than merging into it: its own stats line reports
    // only its own triple, and the final bare `data` shows only that data.
    assert!(
        out.stdout.contains("1 triple(s) loaded"),
        "expected the second load to report only its own triple, got:\n{}",
        out.stdout
    );
    assert!(
        out.stdout.contains("bob") && !out.stdout.contains("Alice"),
        "expected the final bare 'data' to show only the second file's data, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_data_merge_flag_merges_instead_of_replacing() {
    let data1 = fixture("shell_data.ttl");
    let data2 = fixture("shell_data2.ttl");
    let script = format!("data {data1}\ndata {data2} --merge\ndata\nexit\n");
    let out = run_shell(&script);

    // With `--merge`, the second load's stats line counts both files'
    // triples, and the final bare `data` shows the union of both.
    assert!(
        out.stdout.contains("3 triple(s) loaded"),
        "expected the merged load to report triples from both files, got:\n{}",
        out.stdout
    );
    assert!(
        out.stdout.contains("bob") && out.stdout.contains("Alice"),
        "expected the final bare 'data' to show both files' data, got:\n{}",
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

    // The first call prints a stats line for the DCTap it just loaded; the
    // second, bare call has nothing new to load, so it re-shows the full
    // loaded DCTap instead (not another stats line).
    assert_eq!(out.stdout.matches("1 shape(s) loaded").count(), 1);
    assert_eq!(
        out.stdout.matches("xsd:string").count(),
        1,
        "expected the second, bare 'dctap' call to re-show the full loaded DCTap, got:\n{}",
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

    // The first call prints a stats line for the service description it just
    // loaded; the second, bare call has nothing new to load, so it re-shows
    // the full loaded service description instead (not another stats line).
    assert_eq!(out.stdout.matches("feature(s) loaded").count(), 1);
    assert_eq!(
        out.stdout.matches("SPARQL11Query").count(),
        1,
        "expected the second, bare 'service' call to re-show the full loaded service description, got:\n{}",
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

    assert!(out.stdout.contains("2 triple(s) loaded"));
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
        out.stdout.contains("1 shape(s) loaded"),
        "expected the bare file to be loaded as the schema, got:\n{}",
        out.stdout
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
        out.stdout.contains("2 triple(s) loaded"),
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
    assert!(out.stdout.contains("1 shape(s) loaded"));
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

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_config_bare_dumps_the_whole_config_as_toml() {
    let out = run_shell("config\nexit\n");
    assert!(out.stdout.contains("auto_base = false"));
    assert!(out.stdout.contains("[shex]"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_config_get_reports_a_single_key() {
    let out = run_shell("config get auto_base\nexit\n");
    assert!(out.stdout.contains("false"));
    assert!(!out.stdout.contains("Unknown config key"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_config_get_rejects_unknown_key() {
    let out = run_shell("config get not.a.real.key\nexit\n");
    assert!(out.stdout.contains("Unknown config key 'not.a.real.key'."));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_config_set_changes_a_value_for_the_rest_of_the_session() {
    let out = run_shell("config set auto_base true\nconfig get auto_base\nexit\n");
    assert!(out.stdout.contains("auto_base = true"));
    let occurrences = out.stdout.matches("true").count();
    assert_eq!(
        occurrences, 2,
        "expected both the confirmation and the follow-up get to report true, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_config_set_can_populate_an_unset_optional_field() {
    // `base_iri` is `None` by default, so it's omitted from the TOML dump
    // entirely -- setting it still has to work.
    let out = run_shell("config set base_iri http://example.org/\nconfig get base_iri\nexit\n");
    assert!(out.stdout.contains("base_iri = http://example.org/"));
    assert!(out.stdout.contains("http://example.org/"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_config_set_rejects_a_typo_key_without_applying_it() {
    let out = run_shell("config set shex.sho_imports true\nexit\n");
    assert!(out.stdout.contains("Unknown config key 'shex.sho_imports'."));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_config_set_rejects_a_value_of_the_wrong_type() {
    let out = run_shell("config set auto_base not-a-bool\nexit\n");
    assert!(out.stderr.contains("invalid value for 'auto_base'"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_prefixes_bare_reports_empty_by_default() {
    let out = run_shell("prefixes\nexit\n");
    assert!(out.stdout.contains("No default prefixes are defined."));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_prefixes_add_then_shows_it() {
    let out = run_shell("prefixes add rdf http://www.w3.org/1999/02/22-rdf-syntax-ns#\nprefixes\nexit\n");
    assert!(
        out.stdout
            .contains("Added prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>")
    );
    assert!(
        out.stdout
            .contains("prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>")
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_prefixes_rm_removes_it() {
    let out =
        run_shell("prefixes add rdf http://www.w3.org/1999/02/22-rdf-syntax-ns#\nprefixes rm rdf\nprefixes\nexit\n");
    assert!(out.stdout.contains("Removed prefix rdf"));
    assert!(out.stdout.contains("No default prefixes are defined."));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_prefixes_rm_unknown_alias_reports_error() {
    let out = run_shell("prefixes rm rdf\nexit\n");
    assert!(out.stderr.contains("Unknown prefix alias 'rdf'"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_prefixes_rename_changes_the_alias_keeping_the_iri() {
    let out = run_shell(
        "prefixes add rdf http://www.w3.org/1999/02/22-rdf-syntax-ns#\nprefixes rename rdf rdf1\nprefixes\nexit\n",
    );
    assert!(out.stdout.contains("Renamed prefix rdf to rdf1"));
    assert!(
        out.stdout
            .contains("prefix rdf1: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>")
    );
    // Only the "Added prefix rdf: ..." confirmation from the setup line
    // mentions `rdf` at all -- the final listing (its own line, starting
    // with "prefix ") should show only `rdf1` now.
    assert!(!out.stdout.contains("\nprefix rdf:"));
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_prefixes_copy_adds_new_alias_keeping_the_old_one() {
    let out = run_shell(
        "prefixes add rdf http://www.w3.org/1999/02/22-rdf-syntax-ns#\nprefixes copy rdf rdf1\nprefixes\nexit\n",
    );
    assert!(out.stdout.contains("Copied prefix rdf to rdf1"));
    assert!(
        out.stdout
            .contains("prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>")
    );
    assert!(
        out.stdout
            .contains("prefix rdf1: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>")
    );
    assert_eq!(out.code, 0);
}

// The following tests cover the default prefixes being applied when
// loading `data`, `shex`, `shacl`, `query` and `shapemap` -- each of those
// fixtures uses a `p:` alias it never declares itself, so they only parse
// once a `prefixes add p ...` default is in scope.

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_data_without_a_default_prefix_reports_a_parse_error() {
    let data = fixture("shell_data_no_prefix.ttl");
    let out = run_shell(&format!("data {data}\nexit\n"));
    assert!(
        out.stderr.contains("has not been declared") || out.stderr.contains("not declared"),
        "expected an undeclared-prefix parse error, got:\n{}",
        out.stderr
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_data_resolves_prefixed_names_using_default_prefixes() {
    let data = fixture("shell_data_no_prefix.ttl");
    let script = format!("prefixes add p http://example.org/\ndata {data}\ndata\nexit\n");
    let out = run_shell(&script);
    assert!(
        out.stdout.contains("2 triple(s) loaded"),
        "expected the data to load using the default prefix, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_node_resolves_a_default_prefixed_selector() {
    let data = fixture("shell_data_no_prefix.ttl");
    let script = format!("prefixes add p http://example.org/\ndata {data}\nnode p:alice\nexit\n");
    let out = run_shell(&script);
    assert!(
        out.stdout.contains("p:alice"),
        "expected `node` to resolve the default-prefixed selector, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_shex_resolves_prefixed_names_using_default_prefixes() {
    let schema = fixture("shell_schema_no_prefix.shex");
    let script = format!(
        "prefixes add p http://example.org/\nprefixes add xsd http://www.w3.org/2001/XMLSchema#\nshex {schema}\nexit\n"
    );
    let out = run_shell(&script);
    assert!(
        out.stdout.contains("1 shape(s) loaded"),
        "expected the schema to load using the default prefixes, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_query_resolves_prefixed_names_using_default_prefixes() {
    let data = fixture("shell_data_no_prefix.ttl");
    let query = fixture("shell_query_no_prefix.rq");
    let script = format!("prefixes add p http://example.org/\ndata {data}\nquery -q {query}\nexit\n");
    let out = run_shell(&script);
    assert!(
        out.stdout.contains("Alice"),
        "expected the query to resolve using the default prefix, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn shell_shapemap_resolves_prefixed_names_using_default_prefixes() {
    let data = fixture("shell_data_no_prefix.ttl");
    let schema = fixture("shell_schema_no_prefix.shex");
    let shapemap = fixture("shell_shapemap_no_prefix.sm");
    let script = format!(
        "prefixes add p http://example.org/\nprefixes add xsd http://www.w3.org/2001/XMLSchema#\ndata {data}\nshex {schema}\nshapemap {shapemap}\nexit\n"
    );
    let out = run_shell(&script);
    assert!(
        out.stdout.contains("PersonShape"),
        "expected the shapemap to resolve using the default prefixes, got:\n{}",
        out.stdout
    );
    assert_eq!(out.code, 0);
}

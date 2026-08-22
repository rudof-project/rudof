use crate::cli::parser::{Command as CliCommand, CommonArgsAll};
use crate::cli::wrappers::resolve_backend;
use crate::commands::{CommandContext, CommandFactory, extract_common};
use crate::shell::completer::ShellHelper;
use anyhow::{Context, Result, anyhow};
use clap::{CommandFactory as ClapCommandFactory, Parser};
use rudof_lib::formats::{BackendSpec, InputSpec};
use rudof_lib::{DataStats, RudofConfig, TomlConfig};
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::{Cmd, CompletionType, Config, Editor, EventHandler, KeyEvent};
use std::io;
use std::io::Write as _;
use std::process::Command;

pub(super) const PROMPT: &str = "rudof> ";
// Shown for continuation lines of a multi-line command, e.g. a SPARQL query
// spanning several lines inside an open quote. Padded to line up under
// `PROMPT`.
pub(super) const CONTINUATION_PROMPT: &str = "   ... ";

const BANNER: &str = r"                 _        ___
                | |      / __)
  ____ _   _  _ | | ___ | |__
 / ___) | | |/ || |/ _ \|  __)
| |   | |_| ( (_| | |_| | |
|_|    \____|\____|\___/|_|";

// Re-parses a single REPL line into the same `CliCommand` enum used by the
// top-level `rudof` binary, without requiring a leading `rudof` token.
#[derive(Parser)]
#[command(name = "rudof", about = "rudof interactive shell commands", no_binary_name = true)]
struct ReplLine {
    #[command(subcommand)]
    command: CliCommand,
}

/// Reads one full command from `editor`, prompting for further lines (with
/// [`CONTINUATION_PROMPT`]) while the input read so far has unbalanced
/// quoting, so a multi-line inline value (e.g. a SPARQL query passed to
/// `query -q '...'`) can be typed directly instead of requiring a file.
/// `shlex::split` returning `None` is exactly the "still inside an open
/// quote" signal, since that's the same tokenizer [`dispatch`] uses to
/// split the finished line.
fn read_command(editor: &mut Editor<ShellHelper, DefaultHistory>) -> Result<String, ReadlineError> {
    let mut buffer = editor.readline(PROMPT)?;
    while !buffer.trim().is_empty() && shlex::split(&buffer).is_none() {
        // Continuation lines are their own `readline` call, so the helper
        // can't otherwise tell them apart from a fresh command line; tell it
        // explicitly so it doesn't color a continuation line's first word as
        // if it were a subcommand name.
        if let Some(helper) = editor.helper() {
            helper.set_continuation(true);
        }
        let result = editor.readline(CONTINUATION_PROMPT);
        if let Some(helper) = editor.helper() {
            helper.set_continuation(false);
        }
        match result {
            Ok(next) => {
                buffer.push('\n');
                buffer.push_str(&next);
            },
            Err(err @ ReadlineError::Eof) => {
                let _ = writeln!(
                    std::io::stderr(),
                    "Warning: input ended with an unterminated quote; discarding: {buffer}"
                );
                return Err(err);
            },
            Err(err) => return Err(err),
        }
    }
    Ok(buffer)
}

pub fn run(ctx: &mut CommandContext) -> Result<()> {
    let command_names = command_names();
    let endpoint_names = registered_endpoint_names(ctx);
    // List rather than Circular: on Tab with several matches (e.g. "sh" ->
    // shex/shacl/shapemap/shell), print all candidates instead of silently
    // filling in the first one and requiring repeated Tabs to cycle through
    // the rest.
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .completion_show_all_if_ambiguous(true)
        // Belt-and-braces: if a command's output ever ends without a
        // trailing newline, this makes rustyline check the real cursor
        // column before drawing the next prompt and insert one itself,
        // rather than silently drawing over (and later erasing) the tail
        // of that output.
        .check_cursor_position(true)
        .build();
    let mut editor: Editor<ShellHelper, DefaultHistory> = Editor::with_config(config)?;
    editor.set_helper(Some(ShellHelper::new(command_names, endpoint_names)));

    // A key to force-insert a newline instead of submitting, e.g. to keep
    // extending a recalled history entry that already parses as "complete"
    // (matched quoting) and would otherwise run on Enter. Ctrl-J is a plain
    // control byte every terminal delivers unambiguously; Alt-Enter is
    // added too since it works in many terminals, though some intercept it
    // for their own fullscreen toggle before it ever reaches this program —
    // Ctrl-J is the one to rely on.
    editor.bind_sequence(KeyEvent::ctrl('J'), EventHandler::Simple(Cmd::Newline));
    editor.bind_sequence(KeyEvent::alt('\r'), EventHandler::Simple(Cmd::Newline));

    let history_path = history_file();
    if let Some(path) = &history_path {
        // A missing history file is expected on first run; ignore the error.
        let _ = editor.load_history(path);
    }

    writeln!(ctx.writer, "{BANNER}")?;
    writeln!(ctx.writer, "Type 'help' for available commands, 'exit' to quit.")?;
    ctx.writer.flush()?;

    loop {
        match read_command(&mut editor) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Err(err) = editor.add_history_entry(line) {
                    writeln!(std::io::stderr(), "Warning: could not record history entry: {err}")?;
                }
                match line {
                    "exit" | "quit" => break,
                    "help" | "?" => print_help(ctx)?,
                    _ => {
                        if let Err(err) = dispatch(line, ctx) {
                            writeln!(std::io::stderr(), "Error: {err:#}")?;
                        }
                    },
                }
                ctx.writer.flush()?;
            },
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                writeln!(std::io::stderr(), "Readline error: {err}")?;
                break;
            },
        }
    }

    if let Some(path) = &history_path
        && let Err(err) = editor.save_history(path)
    {
        writeln!(std::io::stderr(), "Warning: could not save shell history: {err}")?;
    }

    Ok(())
}

/// Parses `line` as a subcommand invocation and dispatches it through the
/// same [`CommandFactory`] used by the top-level CLI, reusing `ctx` so state
/// loaded by one command (RDF data, a schema, a shapemap...) is visible to
/// the next one.
fn dispatch(line: &str, ctx: &mut CommandContext) -> Result<()> {
    if let Some(command_line) = line.strip_prefix('!') {
        return run_external_command(command_line.trim(), ctx);
    }

    let mut tokens = shlex::split(line).ok_or_else(|| anyhow!("unable to parse line (check quoting): {line}"))?;
    if tokens.is_empty() {
        return Ok(());
    }

    if tokens[0] == "endpoint" {
        return handle_endpoint(&tokens[1..], ctx);
    }

    if tokens[0] == "reset" {
        return handle_reset(&tokens[1..], ctx);
    }

    if tokens[0] == "prefixes" {
        return handle_prefixes(&tokens[1..], ctx);
    }

    if tokens[0] == "config" && tokens.get(1).is_some_and(|sub| sub == "get" || sub == "set") {
        return handle_config(&tokens[1..], ctx);
    }

    // Shell-only `data ... --merge` flag: not part of `DataArgs` (so the
    // top-level CLI has no such flag), stripped out before clap ever sees
    // it. See `should_replace_data` below for what it controls.
    let merge = tokens[0] == "data" && take_merge_flag(&mut tokens);

    apply_bare_resource_convenience(&mut tokens);

    let parsed = match ReplLine::try_parse_from(&tokens) {
        Ok(repl_line) => repl_line.command,
        Err(err) => {
            writeln!(ctx.writer, "{err}")?;
            return Ok(());
        },
    };

    if matches!(parsed, CliCommand::Shell(_)) {
        writeln!(ctx.writer, "Already inside the shell.")?;
        return Ok(());
    }

    // Unlike the top-level CLI (one `data` call per process), the shell can
    // chain several `data` calls in the same session; loading would
    // otherwise *merge* into whatever RDF/PG data is already loaded, which
    // is surprising here since nothing about `data FILE` looks additive.
    // So the shell instead replaces the loaded data by default, unless the
    // new `--merge` flag says otherwise.
    let should_replace_data =
        !merge && matches!(&parsed, CliCommand::Data(args) if has_new_data_source(&args.common, &args.data));

    // `ctx.writer` is set up once for the whole shell session (defaulting
    // to the terminal), so a command's own `-o`/`--output-file` would
    // otherwise be ignored and its output would print to the terminal
    // instead of being written to that file. Redirect just for this one
    // command's execution when it asks for a file.
    let common = extract_common(&parsed);
    let output_path = common.output().cloned().filter(|path| path.to_string_lossy() != "-");
    let force_overwrite = common.force_overwrite();

    // For the "load a resource and dump it" commands, the shell prints a
    // short stats line by default instead of the full dump — the full dump
    // is still available via `-o FILE`, handled below.
    let stats_kind = output_path.is_none().then(|| StatsKind::of(&parsed)).flatten();

    let command = CommandFactory::create(parsed)?;

    if should_replace_data {
        ctx.rudof.reset_data().execute();
    }

    if let Some(kind) = stats_kind {
        let previous_writer = std::mem::replace(&mut ctx.writer, Box::new(io::sink()));
        let result = command.execute(ctx);
        ctx.writer = previous_writer;
        result?;
        writeln!(ctx.writer, "{}", kind.stats_line(ctx))?;
        return Ok(());
    }

    let Some(path) = output_path else {
        return command.execute(ctx);
    };

    let (file_writer, _color) = crate::output::get_writer(&Some(path.clone()), force_overwrite)?;
    let previous_writer = std::mem::replace(&mut ctx.writer, file_writer);

    let result = command.execute(ctx).and_then(|()| Ok(ctx.writer.flush()?));

    ctx.writer = previous_writer;
    result?;

    writeln!(ctx.writer, "Output saved in {}", path.display())?;
    Ok(())
}

/// The subset of subcommands that load a resource (RDF/PG data, a schema,
/// ...) and, by default, dump it in full — in the shell these instead print
/// a short stats line about what got loaded, since a session tends to chain
/// several such commands and the full dump mostly just scrolls past.
#[derive(Clone, Copy)]
enum StatsKind {
    Data,
    Shex,
    Shacl,
    DCTap,
    Pgschema,
    Service,
}

impl StatsKind {
    /// Returns the stats kind for `command`, but only when the line actually
    /// loads a new resource — a bare call with nothing new to load (e.g.
    /// `shex` alone) falls through to the command's normal behavior of
    /// re-showing whatever is already loaded in full, exactly as before this
    /// feature existed.
    fn of(command: &CliCommand) -> Option<Self> {
        match command {
            CliCommand::Data(args) => has_new_data_source(&args.common, &args.data).then_some(Self::Data),
            CliCommand::Shex(args) => args.schema.is_some().then_some(Self::Shex),
            CliCommand::Shacl(args) => (has_new_data_source(&args.common, &args.data) || args.shapes.is_some())
                .then_some(Self::Shacl),
            CliCommand::DCTap(args) => args.file.is_some().then_some(Self::DCTap),
            CliCommand::Pgschema(args) => args.schema.is_some().then_some(Self::Pgschema),
            CliCommand::Service(args) => args.service.is_some().then_some(Self::Service),
            _ => None,
        }
    }

    fn stats_line(self, ctx: &CommandContext) -> String {
        match self {
            Self::Data => match ctx.rudof.data_stats() {
                Some(DataStats::Rdf { triples }) => format!("{triples} triple(s) loaded"),
                Some(DataStats::Pg { nodes, edges }) => format!("{nodes} node(s), {edges} edge(s) loaded"),
                None => "No data loaded".to_string(),
            },
            Self::Shex => match ctx.rudof.shex_schema().and_then(|schema| schema.shapes()) {
                Some(shapes) => format!("{} shape(s) loaded", shapes.len()),
                None => "No shapes loaded".to_string(),
            },
            Self::Shacl => match ctx.rudof.shacl_shapes() {
                Some(shapes) => format!("{} shape(s) loaded", shapes.iter().count()),
                None => "No shapes loaded".to_string(),
            },
            Self::DCTap => match ctx.rudof.dctap() {
                Some(dctap) => format!("{} shape(s) loaded", dctap.shapes().count()),
                None => "No shapes loaded".to_string(),
            },
            Self::Pgschema => match ctx.rudof.pg_schema() {
                Some(schema) => format!(
                    "{} node type(s), {} edge type(s) loaded",
                    schema.node_type_count(),
                    schema.edge_type_count()
                ),
                None => "No schema loaded".to_string(),
            },
            Self::Service => match ctx.rudof.service_description() {
                Some(service) => format!(
                    "{} graph(s), {} feature(s) loaded",
                    service.available_graphs_count(),
                    service.feature_count()
                ),
                None => "No service description loaded".to_string(),
            },
        }
    }
}

/// Mirrors the `has_data_source` check each of [`DataCommand`]/[`ShaclCommand`]
/// makes internally (`rudof_cli/src/commands/data.rs`,
/// `rudof_cli/src/commands/shacl.rs`) to decide whether it's about to load
/// new RDF data, so the shell can tell "loading" and "just re-showing what's
/// already loaded" apart the same way those commands do.
///
/// [`DataCommand`]: crate::commands::DataCommand
/// [`ShaclCommand`]: crate::commands::ShaclCommand
fn has_new_data_source(common: &CommonArgsAll, data: &[InputSpec]) -> bool {
    !data.is_empty() || matches!(resolve_backend(common), BackendSpec::Endpoint(_))
}

/// Shell-only `endpoint [NAME]` command.
///
/// With no argument, reports the endpoint activated (if any) by an earlier
/// `endpoint NAME` call in this session, or points the user at the command
/// otherwise. With a `NAME` argument, activates the matching endpoint
/// registered in the rudof TOML config for the rest of the session (reusing
/// the same mechanism as `data --endpoint NAME`), so later commands that
/// query RDF data (e.g. `query`, `node`) can rely on it without repeating
/// `--endpoint` themselves.
fn handle_endpoint(args: &[String], ctx: &mut CommandContext) -> Result<()> {
    match args {
        [] => show_active_endpoint(ctx),
        [name] => activate_endpoint(name, ctx),
        _ => {
            writeln!(ctx.writer, "Usage: endpoint [NAME]")?;
            Ok(())
        },
    }
}

fn show_active_endpoint(ctx: &mut CommandContext) -> Result<()> {
    let Some(name) = ctx.active_endpoint.clone() else {
        writeln!(ctx.writer, "No endpoint is currently active.")?;
        writeln!(ctx.writer, "Use 'endpoint NAME' to activate one.")?;
        writeln!(ctx.writer, "{}", registered_endpoints_line(ctx))?;
        return Ok(());
    };

    match endpoint_url(ctx, &name) {
        Some(url) => writeln!(ctx.writer, "Active endpoint: {name} ({url})")?,
        None => writeln!(ctx.writer, "Active endpoint: {name}")?,
    }
    Ok(())
}

fn activate_endpoint(name: &str, ctx: &mut CommandContext) -> Result<()> {
    if endpoint_url(ctx, name).is_none() {
        writeln!(ctx.writer, "Unknown endpoint '{name}'.")?;
        writeln!(ctx.writer, "{}", registered_endpoints_line(ctx))?;
        return Ok(());
    }

    ctx.rudof.load_data().with_endpoint(name).execute()?;
    ctx.active_endpoint = Some(name.to_string());

    let url = endpoint_url(ctx, name).unwrap_or_default();
    writeln!(ctx.writer, "Active endpoint: {name} ({url})")?;
    Ok(())
}

fn endpoint_url(ctx: &CommandContext, name: &str) -> Option<String> {
    ctx.rudof
        .config()
        .execute()
        .rdf_data()
        .endpoints()
        .get(name)
        .map(|e| e.query_url().to_string())
}

/// Shell-only `reset [TARGET...]` command.
///
/// With no argument (or `all`), clears every piece of session state loaded so
/// far — RDF data, schemas, shapemap, results, the active endpoint, ... —
/// the same as starting a fresh shell. With one or more target names, clears
/// only the state owned by the matching `rudof` subcommand, leaving
/// everything else untouched.
const RESET_TARGETS: &[&str] = &[
    "data",
    "shex",
    "shacl",
    "pgschema",
    "shapemap",
    "dctap",
    "service",
    "query",
    "sparql",
    "typemap",
    "rdf-config",
    "endpoint",
];

fn handle_reset(args: &[String], ctx: &mut CommandContext) -> Result<()> {
    match args {
        [] => reset_everything(ctx)?,
        [only] if only == "all" => reset_everything(ctx)?,
        targets => {
            let unknown: Vec<&str> = targets
                .iter()
                .map(String::as_str)
                .filter(|target| !RESET_TARGETS.contains(target))
                .collect();
            if !unknown.is_empty() {
                writeln!(ctx.writer, "Unknown reset target(s): {}.", unknown.join(", "))?;
                writeln!(ctx.writer, "Valid targets: {}, or 'all'.", RESET_TARGETS.join(", "))?;
                return Ok(());
            }

            for target in targets {
                reset_target(target, ctx);
            }
            writeln!(ctx.writer, "Reset: {}.", targets.join(", "))?;
        },
    }
    Ok(())
}

fn reset_everything(ctx: &mut CommandContext) -> Result<()> {
    ctx.rudof.reset_all().execute();
    ctx.active_endpoint = None;
    writeln!(ctx.writer, "Reset all session state.")?;
    Ok(())
}

fn reset_target(target: &str, ctx: &mut CommandContext) {
    match target {
        "data" => ctx.rudof.reset_data().execute(),
        "shex" => ctx.rudof.reset_shex_schema().execute(),
        "shacl" => ctx.rudof.reset_shacl_shapes().execute(),
        "pgschema" => ctx.rudof.reset_pg_schema().execute(),
        "shapemap" => ctx.rudof.reset_shapemap().execute(),
        "dctap" => ctx.rudof.reset_dctap().execute(),
        "service" => ctx.rudof.reset_service_description().execute(),
        "query" => {
            ctx.rudof.reset_sparql_query().execute();
            ctx.rudof.reset_query_results().execute();
        },
        "sparql" => ctx.rudof.reset_sparql_query().execute(),
        "typemap" => ctx.rudof.reset_typemap().execute(),
        "rdf-config" => ctx.rudof.reset_rdf_config().execute(),
        "endpoint" => ctx.active_endpoint = None,
        _ => unreachable!("target validated against RESET_TARGETS by handle_reset"),
    }
}

/// Shell-only `prefixes [add ALIAS IRI | rm ALIAS | rename OLD NEW | copy OLD NEW]` command.
///
/// With no argument, shows the current default `PrefixMap` -- the prefix
/// declarations assumed and prepended by default to RDF data, SPARQL
/// queries, ShEx schemas and SHACL shapes, independently of whatever
/// prefixes a loaded resource already declares.
fn handle_prefixes(args: &[String], ctx: &mut CommandContext) -> Result<()> {
    match args {
        [] => show_prefixes(ctx),
        [add, alias, iri] if add == "add" => {
            ctx.rudof.add_prefix(alias, iri).execute()?;
            writeln!(ctx.writer, "Added prefix {alias}: <{iri}>")?;
            Ok(())
        },
        [rm, alias] if rm == "rm" => {
            ctx.rudof.remove_prefix(alias).execute()?;
            writeln!(ctx.writer, "Removed prefix {alias}")?;
            Ok(())
        },
        [rename, old_alias, new_alias] if rename == "rename" => {
            ctx.rudof.rename_prefix(old_alias, new_alias).execute()?;
            writeln!(ctx.writer, "Renamed prefix {old_alias} to {new_alias}")?;
            Ok(())
        },
        [copy, old_alias, new_alias] if copy == "copy" => {
            ctx.rudof.copy_prefix(old_alias, new_alias).execute()?;
            writeln!(ctx.writer, "Copied prefix {old_alias} to {new_alias}")?;
            Ok(())
        },
        _ => {
            writeln!(
                ctx.writer,
                "Usage: prefixes [add ALIAS IRI | rm ALIAS | rename OLD NEW | copy OLD NEW]"
            )?;
            Ok(())
        },
    }
}

fn show_prefixes(ctx: &mut CommandContext) -> Result<()> {
    let prefixes = ctx.rudof.prefixes().execute();
    if prefixes.is_empty() {
        writeln!(ctx.writer, "No default prefixes are defined.")?;
        return Ok(());
    }
    write!(ctx.writer, "{prefixes}")?;
    Ok(())
}

/// Shell-only `config get [KEY]` / `config set KEY VALUE` commands.
///
/// `KEY` is a dot-separated path into the effective TOML configuration (e.g.
/// `base_iri`, or `shex_validator.max_steps`). `config` with no `get`/`set`
/// subcommand falls through to the ordinary top-level `config` command,
/// which dumps the whole configuration as TOML.
fn handle_config(args: &[String], ctx: &mut CommandContext) -> Result<()> {
    match args {
        [get] if get == "get" => show_config(None, ctx),
        [get, key] if get == "get" => show_config(Some(key), ctx),
        [set, key, value] if set == "set" => set_config(key, value, ctx),
        _ => {
            writeln!(ctx.writer, "Usage: config get [KEY] | config set KEY VALUE")?;
            Ok(())
        },
    }
}

fn show_config(key: Option<&str>, ctx: &mut CommandContext) -> Result<()> {
    let toml_str = ctx.rudof.config().execute().to_toml_string()?;
    let Some(key) = key else {
        writeln!(ctx.writer, "{toml_str}")?;
        return Ok(());
    };

    let root: toml::Value = toml::from_str(&toml_str)?;
    match config_lookup(&root, key) {
        Some(value) => {
            writeln!(ctx.writer, "{}", format_config_value(value))?;
            Ok(())
        },
        None => unknown_config_key(key, ctx),
    }
}

fn set_config(key: &str, value: &str, ctx: &mut CommandContext) -> Result<()> {
    let toml_str = ctx.rudof.config().execute().to_toml_string()?;
    let mut root: toml::Value = toml::from_str(&toml_str)?;

    if config_set_path(&mut root, key, parse_config_scalar(value)).is_none() {
        return unknown_config_key(key, ctx);
    }

    let updated_toml = toml::to_string(&root)?;
    let updated_config =
        RudofConfig::from_toml_str(&updated_toml).map_err(|err| anyhow!("invalid value for '{key}': {err}"))?;

    // Re-derive the effective TOML from the *parsed* config, not the raw
    // table just edited, to confirm the key was actually recognized: a
    // typo'd key would otherwise be silently dropped by serde (no section
    // in this config uses `deny_unknown_fields`) and have no effect.
    let effective_toml = updated_config.to_toml_string()?;
    let effective_root: toml::Value = toml::from_str(&effective_toml)?;
    let Some(confirmed) = config_lookup(&effective_root, key) else {
        return unknown_config_key(key, ctx);
    };
    let confirmed = confirmed.clone();

    ctx.rudof.update_config(updated_config).execute();
    writeln!(ctx.writer, "{key} = {}", format_config_value(&confirmed))?;
    Ok(())
}

fn unknown_config_key(key: &str, ctx: &mut CommandContext) -> Result<()> {
    writeln!(ctx.writer, "Unknown config key '{key}'.")?;
    writeln!(ctx.writer, "Use 'config' with no arguments to see all available keys.")?;
    Ok(())
}

fn config_lookup<'a>(root: &'a toml::Value, key: &str) -> Option<&'a toml::Value> {
    key.split('.').try_fold(root, |current, segment| current.get(segment))
}

/// Assigns `new_value` at the dotted `key` path, in place, creating the leaf
/// if needed (e.g. an unset `Option` field omitted from the TOML dump) as
/// long as every parent segment already exists as a table. Returns `None`
/// without modifying `root` if some parent segment doesn't resolve.
fn config_set_path(root: &mut toml::Value, key: &str, new_value: toml::Value) -> Option<()> {
    let mut current = root;
    let mut segments = key.split('.').peekable();
    while let Some(segment) = segments.next() {
        if segments.peek().is_none() {
            current.as_table_mut()?.insert(segment.to_string(), new_value);
            return Some(());
        }
        current = current.get_mut(segment)?;
    }
    None
}

fn parse_config_scalar(value: &str) -> toml::Value {
    if let Ok(b) = value.parse::<bool>() {
        toml::Value::Boolean(b)
    } else if let Ok(i) = value.parse::<i64>() {
        toml::Value::Integer(i)
    } else if let Ok(f) = value.parse::<f64>() {
        toml::Value::Float(f)
    } else {
        toml::Value::String(value.to_string())
    }
}

fn format_config_value(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn registered_endpoint_names(ctx: &CommandContext) -> Vec<String> {
    let mut names: Vec<String> = ctx.rudof.config().execute().rdf_data().endpoints().keys().cloned().collect();
    names.sort_unstable();
    names
}

fn registered_endpoints_line(ctx: &CommandContext) -> String {
    let names = registered_endpoint_names(ctx);
    if names.is_empty() {
        "No endpoints are registered in the config.".to_string()
    } else {
        format!("Registered endpoints: {}", names.join(", "))
    }
}

/// Commands whose primary argument is otherwise only settable through a
/// named flag. Inside the shell (never the top-level CLI, which this
/// doesn't touch), a line consisting of just `<command> <value>` — nothing
/// else — is treated as shorthand for `<command> <flag> <value>`, e.g.
/// `shex examples/user.shex` behaves like `shex -s examples/user.shex`,
/// and `node :alice` behaves like `node -n :alice`.
///
/// This only fires when the value is the *only* token after the command
/// name, so it can never be confused with a value belonging to some other
/// flag; combine it with any other flag and it must be spelled out.
const BARE_RESOURCE_FLAG: &[(&str, &str)] = &[
    ("shex", "-s"),
    ("shacl", "-s"),
    ("shapemap", "-m"),
    ("pgschema", "-s"),
    ("dctap", "-s"),
    ("service", "-s"),
    ("materialize", "-s"),
    ("generate", "-s"),
    ("rdf-config", "-s"),
    ("node", "-n"),
    ("sparql", "-q"),
];

/// Runs `command_line` in the system shell, e.g. `! ls` or `!code file.shex`.
///
/// The line is handed to a real shell (`$SHELL`, falling back to `/bin/sh`
/// on Unix; `cmd /C` on Windows) rather than tokenized ourselves, so
/// pipes/redirects/globs work exactly like they would outside the shell.
/// Stdio is inherited, so interactive commands (an editor, a pager) work
/// too. Only the failure to *launch* the shell itself is an error here — a
/// nonzero exit from the command is left to speak for itself, the same way
/// a plain shell would report it.
fn run_external_command(command_line: &str, ctx: &mut CommandContext) -> Result<()> {
    if command_line.is_empty() {
        writeln!(ctx.writer, "No command given after '!'.")?;
        return Ok(());
    }

    // Flush any buffered rudof output first so it can't appear interleaved
    // after the external command's own (unbuffered, inherited) output.
    ctx.writer.flush()?;

    system_shell(command_line)
        .status()
        .with_context(|| format!("failed to run external command: {command_line}"))?;

    Ok(())
}

#[cfg(not(windows))]
fn system_shell(command_line: &str) -> Command {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut command = Command::new(shell);
    command.arg("-c").arg(command_line);
    command
}

#[cfg(windows)]
fn system_shell(command_line: &str) -> Command {
    let mut command = Command::new("cmd");
    command.arg("/C").arg(command_line);
    command
}

/// Removes a bare `--merge` token from `tokens`, if present, and reports
/// whether it was there. Must run before [`apply_bare_resource_convenience`]
/// so a line like `data FILE --merge` still counts as "exactly one bare
/// value" once `--merge` is out of the way.
fn take_merge_flag(tokens: &mut Vec<String>) -> bool {
    let had_merge = tokens.iter().any(|token| token == "--merge");
    tokens.retain(|token| token != "--merge");
    had_merge
}

fn apply_bare_resource_convenience(tokens: &mut Vec<String>) {
    let [command, value] = tokens.as_slice() else {
        return;
    };
    if value.starts_with('-') {
        return;
    }
    if let Some((_, flag)) = BARE_RESOURCE_FLAG.iter().find(|(name, _)| name == command) {
        tokens.insert(1, (*flag).to_string());
    }
}

fn print_help(ctx: &mut CommandContext) -> Result<()> {
    let help = ReplLine::command().render_long_help();
    writeln!(ctx.writer, "{help}")?;
    writeln!(ctx.writer, "Shell-only commands:")?;
    writeln!(ctx.writer, "  help, ?        Show this help message")?;
    writeln!(ctx.writer, "  exit, quit     Exit the shell")?;
    writeln!(
        ctx.writer,
        "  !<command>     Run <command> in the system shell (e.g. !ls, !code file.shex)"
    )?;
    writeln!(
        ctx.writer,
        "  endpoint [NAME]  Show the active SPARQL endpoint, or activate a registered one"
    )?;
    writeln!(
        ctx.writer,
        "  reset [TARGET...]  Clear session state ({}), or everything with no argument",
        RESET_TARGETS.join(", ")
    )?;
    writeln!(
        ctx.writer,
        "  config get [KEY]      Show the whole config, or one dotted key (e.g. shex_validator.max_steps)"
    )?;
    writeln!(
        ctx.writer,
        "  config set KEY VALUE  Change one config value for the rest of the session"
    )?;
    writeln!(
        ctx.writer,
        "  prefixes [add ALIAS IRI | rm ALIAS | rename OLD NEW | copy OLD NEW]"
    )?;
    writeln!(
        ctx.writer,
        "                     Show, or manage, the default prefix declarations"
    )?;
    writeln!(
        ctx.writer,
        "  data FILE --merge  Merge FILE into the currently loaded data instead of replacing it"
    )?;
    Ok(())
}

fn command_names() -> Vec<String> {
    let mut names: Vec<String> = ReplLine::command()
        .get_subcommands()
        .map(|cmd| cmd.get_name().to_string())
        .collect();
    names.push("help".to_string());
    names.push("exit".to_string());
    names.push("quit".to_string());
    names.push("endpoint".to_string());
    names.push("reset".to_string());
    names.push("prefixes".to_string());
    names
}

fn history_file() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|home| home.join(".rudof_history"))
}

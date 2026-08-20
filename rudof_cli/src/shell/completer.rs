use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result as RustylineResult};

/// `rustyline` helper that completes the first word of a line against the
/// known set of `rudof` subcommand names, and falls back to filename
/// completion for the rest of the line (most subcommand arguments are paths).
pub struct ShellHelper {
    filename_completer: FilenameCompleter,
    commands: Vec<String>,
}

impl ShellHelper {
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            filename_completer: FilenameCompleter::new(),
            commands,
        }
    }
}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> RustylineResult<(usize, Vec<Pair>)> {
        let prefix = &line[..pos];
        if !prefix.contains(' ') {
            let candidates = self
                .commands
                .iter()
                .filter(|name| name.starts_with(prefix))
                .map(|name| Pair {
                    display: name.clone(),
                    replacement: name.clone(),
                })
                .collect();
            return Ok((0, candidates));
        }
        self.filename_completer.complete(line, pos, ctx)
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}

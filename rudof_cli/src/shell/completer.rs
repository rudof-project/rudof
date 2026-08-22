use crate::shell::repl::{CONTINUATION_PROMPT, PROMPT};
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result as RustylineResult};
use std::borrow::Cow::{self, Borrowed, Owned};
use std::cell::Cell;

/// `rustyline` helper that completes the first word of a line against the
/// known set of `rudof` subcommand names, the argument of `endpoint` against
/// the endpoint names registered in the rudof TOML config, and falls back to
/// filename completion for everything else (most subcommand arguments are
/// paths).
///
/// It also drives the shell's interactive highlighting: matching brackets
/// (handy now that a query/shape can be typed across several lines), the
/// command word colored by whether it's recognized, and the prompt itself.
pub struct ShellHelper {
    filename_completer: FilenameCompleter,
    commands: Vec<String>,
    endpoints: Vec<String>,
    bracket_highlighter: MatchingBracketHighlighter,
    // Set by the REPL loop while reading a continuation line of a
    // multi-line command, so `highlight` doesn't color that line's first
    // word as if it were a fresh subcommand name.
    continuation: Cell<bool>,
}

impl ShellHelper {
    pub fn new(commands: Vec<String>, endpoints: Vec<String>) -> Self {
        Self {
            filename_completer: FilenameCompleter::new(),
            commands,
            endpoints,
            bracket_highlighter: MatchingBracketHighlighter::new(),
            continuation: Cell::new(false),
        }
    }

    pub fn set_continuation(&self, continuation: bool) {
        self.continuation.set(continuation);
    }

    fn first_word_end(line: &str) -> usize {
        line.find(' ').unwrap_or(line.len())
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

        if &line[..Self::first_word_end(line)] == "endpoint" {
            let word_start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
            let word_prefix = &line[word_start..pos];
            let candidates = self
                .endpoints
                .iter()
                .filter(|name| name.starts_with(word_prefix))
                .map(|name| Pair {
                    display: name.clone(),
                    replacement: name.clone(),
                })
                .collect();
            return Ok((word_start, candidates));
        }

        self.filename_completer.complete(line, pos, ctx)
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}

impl Highlighter for ShellHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let highlighted = self.bracket_highlighter.highlight(line, pos);

        // Color the command word: green if it's a recognized subcommand,
        // red otherwise, as an early typo signal. Left alone for `!...`
        // external-shell lines (not `rudof` subcommands) and continuation
        // lines (not a command word at all).
        let first_word_end = Self::first_word_end(line);
        let word = &line[..first_word_end];
        if word.is_empty() || word.starts_with('!') || self.continuation.get() {
            return highlighted;
        }

        let color = if self.commands.iter().any(|c| c == word) {
            "\x1b[32m" // green
        } else {
            "\x1b[31m" // red
        };
        // `highlighted` only ever diverges from `line` at a matched bracket
        // position, which is always past the command word in practice; if
        // that assumption is ever wrong, fall back to the bracket-only
        // highlighting rather than slicing at a bad byte boundary.
        let Some(rest) = highlighted.get(first_word_end..) else {
            return highlighted;
        };
        Owned(format!("{color}{word}\x1b[0m{rest}"))
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, _default: bool) -> Cow<'b, str> {
        if prompt == PROMPT {
            Owned(format!("\x1b[1;36m{prompt}\x1b[0m")) // bold cyan
        } else if prompt == CONTINUATION_PROMPT {
            Owned(format!("\x1b[2m{prompt}\x1b[0m")) // dim
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        // Bracket matching needs a refresh when the tracked bracket state
        // changes; command-word coloring needs one on every keystroke while
        // still typing that word, since typing normally skips highlight()
        // for a trivial single-character append otherwise.
        let bracket_changed = self.bracket_highlighter.highlight_char(line, pos, kind);
        bracket_changed || (!self.continuation.get() && pos <= Self::first_word_end(line))
    }
}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}

#[cfg(test)]
mod tests {
    use super::*;
    use rustyline::history::DefaultHistory;

    #[test]
    fn completes_endpoint_argument_against_registered_endpoint_names() {
        let helper = ShellHelper::new(
            vec!["endpoint".to_string()],
            vec!["wikidata".to_string(), "dbpedia".to_string()],
        );
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);

        let line = "endpoint wi";
        let (start, candidates) = helper.complete(line, line.len(), &ctx).unwrap();

        assert_eq!(start, "endpoint ".len());
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].replacement, "wikidata");
    }

    #[test]
    fn other_commands_still_fall_back_to_filename_completion() {
        let helper = ShellHelper::new(vec!["shex".to_string()], vec!["wikidata".to_string()]);
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);

        let line = "shex wikidata-not-a-file";
        let (_, candidates) = helper.complete(line, line.len(), &ctx).unwrap();

        assert!(candidates.is_empty());
    }
}

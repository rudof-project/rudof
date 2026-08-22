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
/// the endpoint names registered in the rudof TOML config, the `KEY` argument
/// of `config get`/`config set` against the dotted config-key paths in that
/// same config, and falls back to filename completion for everything else
/// (most subcommand arguments are paths).
///
/// It also drives the shell's interactive highlighting: matching brackets
/// (handy now that a query/shape can be typed across several lines), the
/// command word colored by whether it's recognized, and the prompt itself.
pub struct ShellHelper {
    filename_completer: FilenameCompleter,
    commands: Vec<String>,
    endpoints: Vec<String>,
    config_keys: Vec<String>,
    bracket_highlighter: MatchingBracketHighlighter,
    // Set by the REPL loop while reading a continuation line of a
    // multi-line command, so `highlight` doesn't color that line's first
    // word as if it were a fresh subcommand name.
    continuation: Cell<bool>,
}

impl ShellHelper {
    pub fn new(commands: Vec<String>, endpoints: Vec<String>, config_keys: Vec<String>) -> Self {
        Self {
            filename_completer: FilenameCompleter::new(),
            commands,
            endpoints,
            config_keys,
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

    /// If `pos` is positioned at `config`'s own subcommand word (i.e. the
    /// second word on the line, right after `config `), returns the partial
    /// subcommand typed so far, so it can be completed against `get`/`set`.
    fn config_subcommand_word(line: &str, pos: usize) -> Option<&str> {
        let word_start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let mut words = line[..word_start].split_whitespace();
        match (words.next(), words.next()) {
            (Some("config"), None) => Some(&line[word_start..pos]),
            _ => None,
        }
    }

    /// If `pos` is positioned at the `KEY` argument of `config get`/`config
    /// set` (i.e. exactly the third word on the line, right after `config
    /// get `/`config set `), returns the partial key typed so far. `None`
    /// for any other position, including the `VALUE` argument of `config set
    /// KEY VALUE` — that one isn't a config key and shouldn't be completed
    /// as one.
    fn config_key_word(line: &str, pos: usize) -> Option<&str> {
        let word_start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let mut words = line[..word_start].split_whitespace();
        match (words.next(), words.next(), words.next()) {
            (Some("config"), Some("get") | Some("set"), None) => Some(&line[word_start..pos]),
            _ => None,
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

        if let Some(sub_prefix) = Self::config_subcommand_word(line, pos) {
            let word_start = pos - sub_prefix.len();
            let candidates = ["get", "set"]
                .into_iter()
                .filter(|name| name.starts_with(sub_prefix))
                .map(|name| Pair {
                    display: name.to_string(),
                    replacement: name.to_string(),
                })
                .collect();
            return Ok((word_start, candidates));
        }

        if let Some(key_prefix) = Self::config_key_word(line, pos) {
            let word_start = pos - key_prefix.len();
            let candidates = self
                .config_keys
                .iter()
                .filter(|key| key.starts_with(key_prefix))
                .map(|key| Pair {
                    display: key.clone(),
                    replacement: key.clone(),
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
            vec![],
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
        let helper = ShellHelper::new(vec!["shex".to_string()], vec!["wikidata".to_string()], vec![]);
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);

        let line = "shex wikidata-not-a-file";
        let (_, candidates) = helper.complete(line, line.len(), &ctx).unwrap();

        assert!(candidates.is_empty());
    }

    #[test]
    fn completes_config_subcommand_against_get_and_set() {
        let helper = ShellHelper::new(vec!["config".to_string()], vec![], vec![]);
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);

        let line = "config ";
        let (start, candidates) = helper.complete(line, line.len(), &ctx).unwrap();

        assert_eq!(start, "config ".len());
        let mut replacements: Vec<&str> = candidates.iter().map(|c| c.replacement.as_str()).collect();
        replacements.sort_unstable();
        assert_eq!(replacements, vec!["get", "set"]);
    }

    #[test]
    fn completes_config_get_key_against_known_config_paths() {
        let helper = ShellHelper::new(
            vec!["config".to_string()],
            vec![],
            vec![
                "shex_validator.max_steps".to_string(),
                "shex_validator.width".to_string(),
                "shex".to_string(),
            ],
        );
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);

        let line = "config get shex_validator.";
        let (start, candidates) = helper.complete(line, line.len(), &ctx).unwrap();

        assert_eq!(start, "config get ".len());
        let mut replacements: Vec<&str> = candidates.iter().map(|c| c.replacement.as_str()).collect();
        replacements.sort_unstable();
        assert_eq!(replacements, vec!["shex_validator.max_steps", "shex_validator.width"]);
    }

    #[test]
    fn does_not_complete_config_set_value_as_a_config_key() {
        let helper = ShellHelper::new(
            vec!["config".to_string()],
            vec![],
            vec!["shex_validator.max_steps".to_string()],
        );
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);

        // The VALUE position (4th word) must fall back to filename
        // completion, not be treated as a config key.
        let line = "config set shex_validator.max_steps shex_valid";
        let (_, candidates) = helper.complete(line, line.len(), &ctx).unwrap();

        assert!(candidates.is_empty());
    }
}

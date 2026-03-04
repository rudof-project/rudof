//! Terminal color support detection.

use std::{env, sync::OnceLock};

/// Represents the level of color support available in the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
    /// No color support (or colors explicitly disabled).
    NoColor,
    /// Standard 16-color support (ANSI).
    Basic,
    /// 256-color support (xterm).
    Ansi256,
    /// 24-bit color support (16.7 million colors).
    TrueColor,
}

impl ColorSupport {
    /// Returns `true` if any level of color is enabled.
    pub fn enabled(self) -> bool {
        !matches!(self, ColorSupport::NoColor)
    }
}

/// Maps an internal [ColorLevel] to the public [ColorSupport] enum.
fn from_level(level: ColorLevel) -> ColorSupport {
    if level.has_16m {
        ColorSupport::TrueColor
    } else if level.has_256 {
        ColorSupport::Ansi256
    } else if level.has_basic {
        ColorSupport::Basic
    } else {
        ColorSupport::NoColor
    }
}

/// Detects color support for Stdout using a thread-safe cache.
///
/// This is the recommended entry point to avoid repeated environment
/// checks and system calls during the application lifecycle.
pub fn detect_color_support_cached() -> ColorSupport {
    match on_cached(Stream::Stdout) {
        None => ColorSupport::NoColor,
        Some(level) => from_level(level),
    }
}

// ============================================================================
// Internal Detection Logic
// ============================================================================

/// Represents a standard output stream.
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
enum Stream {
    Stdout = 0,
    Stderr = 1,
}

/// Internal representation of color capabilities with granular flags.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct ColorLevel {
    level: usize,
    has_basic: bool,
    has_256: bool,
    has_16m: bool,
}

/// Checks `FORCE_COLOR` and `CLICOLOR_FORCE` environment variables.
///
/// Follows the convention where `FORCE_COLOR=1` enables basic colors,
/// and `FORCE_COLOR=3` enables TrueColor.
fn env_force_color() -> usize {
    if let Ok(force) = env::var("FORCE_COLOR") {
        match force.as_ref() {
            "true" | "" => 1,
            "false" => 0,
            f => f.parse::<usize>().unwrap_or(1).min(3),
        }
    } else if let Ok(cli_clr_force) = env::var("CLICOLOR_FORCE") {
        if cli_clr_force != "0" { 1 } else { 0 }
    } else {
        0
    }
}

/// Checks the `NO_COLOR` environment variable.
fn env_no_color() -> bool {
    env::var("NO_COLOR").is_ok_and(|val| val != "0")
}

/// Converts a numeric color level into a [ColorLevel] struct.
fn translate_level(level: usize) -> Option<ColorLevel> {
    if level == 0 {
        None
    } else {
        Some(ColorLevel {
            level,
            has_basic: true,
            has_256: level >= 2,
            has_16m: level >= 3,
        })
    }
}

/// Returns true if the stream is a terminal (TTY).
fn is_a_tty(stream: Stream) -> bool {
    use std::io::IsTerminal;
    match stream {
        Stream::Stdout => std::io::stdout().is_terminal(),
        Stream::Stderr => std::io::stderr().is_terminal(),
    }
}

/// Core detection logic that evaluates TTY status and environment variables.
fn supports_color(stream: Stream) -> usize {
    let force_color = env_force_color();
    if force_color > 0 {
        return force_color;
    }

    // Colors are disabled if NO_COLOR is set, TERM is dumb, or it's not a TTY.
    if env_no_color()
        || env::var("TERM").is_ok_and(|t| t == "dumb")
        || !(is_a_tty(stream) || env::var("IGNORE_IS_TERMINAL").is_ok_and(|v| v != "0"))
    {
        return 0;
    }

    // Level 3: TrueColor detection
    if env::var("COLORTERM").is_ok_and(|v| check_colorterm_16m(&v))
        || env::var("TERM").is_ok_and(|v| check_term_16m(&v))
        || env::var("TERM_PROGRAM").is_ok_and(|v| v == "iTerm.app")
    {
        return 3;
    }

    // Level 2: 256-color detection
    if env::var("TERM_PROGRAM").is_ok_and(|v| v == "Apple_Terminal")
        || env::var("TERM_PROGRAM").is_ok_and(|v| v == "iTerm.app")
    {
        return 2;
    }

    // Level 1: Basic color detection
    if env::var("COLORTERM").is_ok()
        || check_ansi_color(env::var("TERM").ok().as_deref())
        || env::var("CLICOLOR").is_ok_and(|v| v != "0")
        || is_ci::uncached()
    {
        return 1;
    }

    0
}

#[cfg(windows)]
fn check_ansi_color(term: Option<&str>) -> bool {
    // Windows supports ANSI if it's not a known restricted terminal
    term.is_none_or(|t| t != "dumb" && t != "cygwin")
}

#[cfg(not(windows))]
fn check_ansi_color(term: Option<&str>) -> bool {
    term.is_none_or(|t| t != "dumb" && t != "cygwin")
}

fn check_colorterm_16m(colorterm: &str) -> bool {
    colorterm == "truecolor" || colorterm == "24bit"
}

fn check_term_16m(term: &str) -> bool {
    term.ends_with("direct") || term.ends_with("truecolor")
}

/// Detects color support for a specific stream, caching the result.
fn on_cached(stream: Stream) -> Option<ColorLevel> {
    static CACHE: [OnceLock<Option<ColorLevel>>; 2] = [OnceLock::new(), OnceLock::new()];
    let idx = stream as usize;
    *CACHE[idx].get_or_init(|| translate_level(supports_color(stream)))
}

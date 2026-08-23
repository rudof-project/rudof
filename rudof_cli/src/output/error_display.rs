/// Formats an error for display to the user: the top-level message, plus
/// any further "caused by" context, each on its own line — skipping any
/// cause whose text is already fully contained in a message already kept.
///
/// Some of this workspace's error types (`rudof_lib`'s `RudofError` and the
/// domain errors it wraps, e.g. `ShExError`, `DataError`, ...) already embed
/// their full wrapped-error text into their own `Display` via `#[error("X
/// error: {0}")]` — so anyhow's usual approach of walking the `source()`
/// chain and printing every link (`{:#}`, or the default `{:?}` used when
/// `main()` returns `Err`) ends up repeating that same text a second time.
/// This only prints the parts that add genuinely new information, which
/// also keeps a multi-line source-location + reason error (like an I/O
/// error wrapped with `.context(...)`) readable instead of one long
/// colon-joined line.
pub fn format_error(err: &anyhow::Error) -> String {
    let mut lines: Vec<String> = Vec::new();
    for cause in err.chain() {
        let text = cause.to_string();
        if lines.iter().any(|line| line.contains(&text)) {
            continue;
        }
        lines.push(text);
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::format_error;
    use anyhow::anyhow;
    use std::fmt;

    #[derive(Debug)]
    struct Wrapping {
        message: &'static str,
        cause: Inner,
    }

    impl fmt::Display for Wrapping {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "outer: {}", self.message)
        }
    }

    impl std::error::Error for Wrapping {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.cause)
        }
    }

    #[derive(Debug)]
    struct Inner(&'static str);

    impl fmt::Display for Inner {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for Inner {}

    #[test]
    fn drops_a_cause_whose_text_is_already_embedded_in_the_outer_message() {
        // Mirrors this workspace's `RudofError`-style errors: the outer
        // Display already contains the inner cause's full text verbatim.
        let err = anyhow::Error::new(Wrapping {
            message: "something went wrong",
            cause: Inner("something went wrong"),
        });
        assert_eq!(format_error(&err), "outer: something went wrong");
    }

    #[test]
    fn keeps_a_genuinely_separate_context_cause_on_its_own_line() {
        // Mirrors `anyhow::Context`-style wrapping: the context message
        // doesn't embed the underlying error's text, so both are kept.
        let io_err = std::io::Error::other("disk full");
        let err = anyhow!(io_err).context("Failed to write the output file");
        assert_eq!(format_error(&err), "Failed to write the output file\ndisk full");
    }
}

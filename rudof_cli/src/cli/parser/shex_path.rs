use crate::cli::parser::CommonArgsNoBackend;
use crate::cli::wrappers::ShExFormatCli;
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `shex-path` command
#[derive(Debug, Clone, Args)]
pub struct ShexPathArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema, FILE, URI or - for stdin"
    )]
    pub schema: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Schema format (ShExC, ShExJ, Turtle, ...), default = ShExC",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub schema_format: ShExFormatCli,

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
    pub base: Option<String>,

    #[arg(
        short = 'p',
        long = "path",
        value_name = "SHEXPATH",
        help = "ShExPath to evaluate, written with the prefixes and base of the schema"
    )]
    pub path: String,

    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}

#[cfg(test)]
mod tests {
    use super::ShexPathArgs;
    use crate::cli::wrappers::ShExFormatCli;
    use clap::Parser;

    /// Standalone wrapper so `ShexPathArgs` can be parsed on its own.
    #[derive(Debug, Parser)]
    struct TestCli {
        #[command(flatten)]
        args: ShexPathArgs,
    }

    // The schema is `-`, stdin: a file name has to name an existing file to be parsed.

    #[test]
    fn schema_and_path_are_parsed() {
        let cli = TestCli::parse_from(["test", "-s", "-", "-p", "@<#IssueShape>~:category"]);
        assert_eq!(cli.args.path, "@<#IssueShape>~:category");
        assert_eq!(cli.args.schema.source_name(), "stdin");
        assert_eq!(cli.args.base, None);
    }

    #[test]
    fn long_names_are_accepted() {
        let cli = TestCli::parse_from([
            "test",
            "--schema",
            "-",
            "--path",
            "/shapes/*",
            "--base",
            "http://ex.example/schema",
        ]);
        assert_eq!(cli.args.path, "/shapes/*");
        assert_eq!(cli.args.base.as_deref(), Some("http://ex.example/schema"));
    }

    #[test]
    fn the_schema_format_defaults_to_shexc() {
        let cli = TestCli::parse_from(["test", "-s", "-", "-p", "/shapes/*"]);
        assert!(matches!(cli.args.schema_format, ShExFormatCli::ShExC));
        let cli = TestCli::parse_from(["test", "-s", "-", "-f", "shexj", "-p", "/shapes/*"]);
        assert!(matches!(cli.args.schema_format, ShExFormatCli::ShExJ));
    }

    #[test]
    fn the_path_is_required() {
        assert!(TestCli::try_parse_from(["test", "-s", "-"]).is_err());
    }

    #[test]
    fn the_schema_is_required() {
        assert!(TestCli::try_parse_from(["test", "-p", "/shapes/*"]).is_err());
    }
}

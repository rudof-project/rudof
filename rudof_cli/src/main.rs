#[cfg(not(target_family = "wasm"))]
use rudof_cli::{
    cli::parser::{Cli, Command},
    commands::{CommandContext, CommandFactory},
};
#[cfg(not(target_family = "wasm"))]
use std::{env, io};
#[cfg(not(target_family = "wasm"))]
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(not(target_family = "wasm"))]
fn main() -> anyhow::Result<()> {
    use clap::Parser;

    // Initialize logging, environment variables, and global settings
    setup();

    // Use args_os to safely handle non-UTF8 paths/arguments from the system
    let args = clientele::args_os()?;
    let cli = Cli::parse_from(args);

    // Dispatch the command if present, otherwise exit with a clean error message
    match &cli.command {
        Some(cmd) => execute(cmd, cli.debug)?,
        None => anyhow::bail!("Command not specified. Use --help for available commands."),
    }

    Ok(())
}

#[cfg(not(target_family = "wasm"))]
/// Sets up the application environment including logging and signal handling.
fn setup() {
    clientele::dotenv().ok();

    unsafe {
        env::set_var("RUSTEMO_NOTRACE", "1");
    }

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .with_writer(io::stderr)
        .without_time();

    tracing_subscriber::registry()
        .with(env_filter.clone())
        .with(fmt_layer)
        .init();

    tracing::trace!("rudof running with tracing filter {}", env_filter);
}

#[cfg(not(target_family = "wasm"))]
/// Orchestrates the command lifecycle: Creation -> Validation -> Execution.
fn execute(cli_command: &Command, debug: u8) -> anyhow::Result<()> {
    // Convert CLI enum into a Command Trait Object
    let command = CommandFactory::create(cli_command.clone())?;

    // Prepare the execution context (writers, colors, config)
    let mut ctx = CommandContext::from_cli(cli_command, debug)?;

    // Run the core logic
    command.execute(&mut ctx)?;

    Ok(())
}

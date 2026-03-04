# `rudof_cli`

## Overview

The `rudof_cli` crate provides the command-line interface for the Rudof toolkit, that provides a comprehensive suite of tools for working with RDF data and schema languages. It enables users to inspect and validate RDF data using ShEx or SHACL, as well as convert between different RDF modeling languages such as ShEx, SHACL, and DCTAP.

## Architecture and Package Structure

The `rudof_cli` crate is organized around a **command pattern** with the following layers:

### 1. CLI Layer (`cli/`)

Handles command-line argument parsing using Clap, defining the user interface and available commands.

- **`parser.rs`**: Defines the complete CLI structure using Clap's derive macros, including all commands, subcommands, and their arguments. This module uses a declarative approach where commands are represented as enum variants, ensuring type safety and exhaustive pattern matching. Common arguments (configuration paths, output destinations, force overwrite flags) are abstracted into reusable structures like `CommonArgs`, `CommonArgsAll`, and `CommonArgsOutputForceOverWrite` to eliminate duplication. The parser exclusively uses CLI wrapper types (e.g., `DataFormatCli`) instead of library types directly, creating a clean decoupling between the presentation layer and core business logic.

- **`wrappers.rs`**: Provides CLI-friendly wrappers for core library types using a macro-based approach implemented through the `cli_wrapper!` macro. This macro automatically generates all necessary boilerplate code for creating enums with Clap's `ValueEnum` support, bidirectional conversion between CLI and library types, display formatting for help messages, and optional `MimeType` trait implementations.

### 2. Command Layer (`commands/`)

Implements each command using a unified trait-based approach defined in `commands/base.rs`.

- **`base.rs`**: The foundation of the command system, containing three key components:
  
  - **`Command` trait**: The core interface that all commands must implement, defining an `execute(&self, ctx: &mut CommandContext)` method for command logic and a `name(&self)` method for identification. This enables polymorphic command execution where the main application works with trait objects without knowing concrete command types.
  
  - **`CommandContext`**: The shared execution environment that acts as a dependency injection container. It provides commands with a `Box<dyn Write>` output writer (supporting stdout, files, or other destinations), a configured `Rudof` instance initialized with all settings, a debug level for verbosity control, and color support detection. The `from_cli()` factory method bridges CLI parsing and command execution by loading configuration files, initializing the Rudof library, creating appropriate output writers, and detecting terminal capabilities—shielding commands from initialization complexity.
  
  - **`CommandFactory`**: A factory pattern implementation that centralizes command instantiation logic. The `create()` method maps CLI command enum variants to their corresponding `Command` trait objects through type erasure. This design follows the Open-Closed Principle: the system is open for extension (new commands can be added by adding a single match arm) but closed for modification (existing command handling remains unchanged).

- **Individual command modules** (`shex.rs`, `shacl.rs`, `validate.rs`, `data.rs`, `node.rs`, etc.): Each implements the `Command` trait to provide specific functionality. Commands follow a consistent execution pattern: (1) convert CLI wrapper types to library types, (2) execute core logic by calling Rudof library methods and (3) write results to the context's output writer.

### 3. Output Layer (`output/`)

Manages output formatting with automatic color support detection and configurable writers.

- **`color.rs`**: Detects terminal color capabilities through a three-state model (Always, Never, Auto). Respects explicit user preferences via `FORCE_COLOR` and `NO_COLOR` environment variables, automatically detects terminal capabilities using the `supports-color` crate, handles CI environment detection where colored output may not render correctly, and caches detection results to avoid repeated system calls for performance optimization.

- **`writer.rs`**: Creates appropriate output writers based on command-line arguments. When no output file is specified, returns stdout with automatic color detection. When an output file is specified, creates a file handle with overwrite protection and disables color output (since files don't support ANSI codes).

### Command Lifecycle

The `main.rs` orchestrates the complete command lifecycle through five distinct phases:

1. **Setup**: Initializes the application environment by loading `.env` files with environment variables, configuring the tracing subsystem for structured logging (writing to stderr with file and line number information), and setting necessary environment variables for dependent libraries. This establishes the foundational runtime environment.

2. **Parsing**: Safely handles command-line arguments using Clap, including non-UTF8 paths that can occur on some operating systems. Through `clientele::args_os()` and `Cli::parse_from()`, raw arguments are transformed into the strongly-typed `Cli` structure with all validation applied, ensuring only well-formed commands proceed to execution.

3. **Factory**: Uses `CommandFactory::create()` to instantiate the appropriate command implementation from the parsed CLI enum. The factory performs type erasure, returning a `Box<dyn Command>` trait object that enables polymorphic handling without the main function needing to know concrete command types.

4. **Context**: Builds the execution environment through `CommandContext::from_cli()`, which loads the configuration file (if specified), initializes the Rudof library with loaded settings, creates the appropriate output writer (stdout or file) based on CLI flags, and detects terminal color capabilities. Commands receive a fully prepared context with all dependencies resolved.

5. **Execution**: Invokes `command.execute(&mut ctx)` to run the command's business logic. Errors are automatically propagated up the call stack using the `?` operator and handled uniformly by the main function, where they're reported to the user with rich context information. This separation of phases makes the application's execution flow predictable and maintainable.

## Adding a New Command

This section provides a step-by-step guide to adding a new command to the CLI.

### Step 1: Define Command Arguments in `cli/parser.rs`

Add your command's arguments structure:

```rust
#[derive(Args, Debug, Clone)]
pub struct MyCommandArgs {
    /// Common arguments (config, output, etc.)
    #[command(flatten)]
    pub common: CommonArgsAll,

    /// Input data file
    #[arg(short, long)]
    pub input: String,

    /// Optional parameter with default
    #[arg(short, long, default_value = "default_value")]
    pub option: String,

    /// Using a CLI wrapper type
    #[arg(short, long, value_enum)]
    pub format: DataFormatCli,
}
```

Add the command variant to the `Command` enum:

```rust
#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    // ... existing commands ...
    
    /// Brief description of your command
    MyCommand(MyCommandArgs),
}
```

### Step 2: Add Common Args Handling in `commands/base.rs`

Update the `extract_common()` function to handle your command:

```rust
fn extract_common(cmd: &CliCommand) -> CommonArgs {
    match cmd {
        // ... existing matches ...
        
        CliCommand::MyCommand(a) => CommonArgs::All(CommonArgsAll {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
    }
}
```

### Step 3: Create Command Implementation File

Create `commands/my_command.rs`:

```rust
use crate::cli::parser::MyCommandArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `my-command` command.
///
/// Detailed description of what this command does.
pub struct MyCommand {
    /// Arguments specific to this command.
    args: MyCommandArgs,
}

impl MyCommand {
    pub fn new(args: MyCommandArgs) -> Self {
        Self { args }
    }
}

impl Command for MyCommand {
    fn name(&self) -> &'static str {
        "my-command"
    }

    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        // 1. Convert CLI types to library types
        let format = (&self.args.format).into();

        // 2. Execute your command logic
        // ... your implementation ...

        // 3. Write output
        writeln!(ctx.writer, "Command executed successfully")?;

        Ok(())
    }
}
```

### Step 4: Register Command in Module System

Update `commands/mod.rs`:

```rust
mod my_command;  // Add module declaration

pub use my_command::MyCommand;  // Export the command
```

### Step 5: Add to Command Factory

Update the factory in `commands/base.rs`:

```rust
impl CommandFactory {
    pub fn create(cli_command: CliCommand) -> Result<Box<dyn Command>> {
        match cli_command {
            // ... existing matches ...
            
            CliCommand::MyCommand(args) => Ok(Box::new(MyCommand::new(args))),
        }
    }
}
```

### Step 6: Update Documentation

Add usage documentation in appropriate places.

## Dependencies

### Main Dependencies

The CLI relies on the following external crates:

- **`clap`**: Command-line argument parsing with derive macros
- **`clap_complete_command`**: Shell completion generation
- **`tokio`**: Async runtime for concurrent operations
- **`anyhow`** / **`thiserror`**: Error handling
- **`tracing`** / **`tracing-subscriber`**: Structured logging
- **`tabled`**: Table formatting for output
- **`supports-color`**: Terminal color capability detection
- **`clientele`**: CLI utility helpers

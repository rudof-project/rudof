# `rudof_cli`

The command-line interface for the Rudof toolkit, that provides a comprehensive suite of tools for working with RDF data and schema languages. It enables users to inspect and validate RDF data using ShEx or SHACL, as well as convert between different RDF modeling languages such as ShEx, SHACL, and DCTAP.

## Architecture and Package Structure

The `rudof_cli` crate is organized around a **command pattern** with the following layers:

### 1. CLI Layer (`cli/`)

Handles command-line argument parsing using Clap, defining the user interface and available commands.

- **`parser.rs`**: Defines the complete CLI structure using Clap's derive macros, including all commands, subcommands, and their arguments.

- **`wrappers.rs`**: Provides CLI-friendly wrappers for core library types using a macro-based approach. This abstraction layer keeps the CLI parsing separate from the core library, allowing independent evolution of both.

### 2. Command Layer (`commands/`)

Implements each command using a unified trait-based approach defined in `commands/base.rs`:

- **`base.rs`**: The foundation of the command system, containing three key components:
  
  - **`Command` trait**: The core interface that all commands must implement.
  
  - **`CommandContext`**: The shared execution environment.
  
  - **`CommandFactory`**: A **factory pattern** implementation that maps CLI command variants to their corresponding `Command` trait objects, centralizing command instantiation logic.

- **Individual command modules** (`shex.rs`, `shacl.rs`, `validate.rs`, etc.): Each implements the `Command` trait to provide specific functionality.

### 3. Output Layer (`output/`)

Manages output formatting with automatic color support detection and configurable writers.

- **`color.rs`**: Detects terminal color capabilities
- **`writer.rs`**: Creates appropriate output writers (stdout, files, etc.)

### Command Lifecycle

The `main.rs` orchestrates the complete command lifecycle:

1. **Setup**: Initialize logging with tracing, load environment variables, configure signal handling
2. **Parsing**: Parse CLI arguments using Clap into structured `Command` enum
3. **Factory**: Use `CommandFactory::create()` to instantiate the appropriate command
4. **Context**: Build `CommandContext` with configuration, output writers, and color support
5. **Execution**: Call `command.execute(&mut ctx)` to run the command logic

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

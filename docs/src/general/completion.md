# Completion Scripts

## Overview

Shell completion scripts enables tab completion for commands, subcommands, options, and arguments in your preferred shell.

Once installed, shell completion allows you to:

- **Tab-complete commands**: Type `rudof val<TAB>` → `rudof validate`
- **Tab-complete options**: Type `rudof validate --sh<TAB>` → `rudof validate --schema`
- **Tab-complete file paths**: Automatically suggest files and directories for file arguments
- **View available options**: Press `TAB` twice to see all available options at any point
- **Reduce typos**: Let the shell validate command names and options before execution

## Supported Shells

Completion scripts are generated for the following shells:

- **Bash** - The Bourne Again SHell, default on most Linux distributions
- **Zsh** - Z shell, default on macOS (10.15+) and popular among advanced users
- **Fish** - The friendly interactive shell with built-in completion support
- **PowerShell** - Microsoft PowerShell for Windows, Linux, and macOS
- **Elvish** - A modern shell with a unique approach to scripting
- **Nushell** - A modern shell with structured data pipelines

## Getting the scripts

### From a release (recommended)

Every [GitHub release](https://github.com/rudof-project/rudof/releases) ships a
`rudof_completions.zip` asset containing the completion script for each shell
above. Download the archive matching the release you installed, pick
the file for your shell, and skip to [Installation Instructions](#installation-instructions)
below.

### Via the Nix module

If you use the `programs.rudof` NixOS/home-manager module (see the
[Nix module options reference](../references/nix-options.md)), completions
are installed automatically alongside the package — nothing extra to do.

### Generating them yourself

The scripts are produced from `rudof`'s real argument parser by a small
helper binary in the `rudof_cli` crate, `rudof-completions`, rather than by a
runtime `rudof` subcommand. From a checkout:

```bash
cargo run --release --bin rudof-completions -- <SHELL>
```

## Installation Instructions

After obtaining the completion script, you need to install it in the appropriate location for your shell. The installation process varies by shell.

> **Note**: For detailed information about completion systems, refer to the official documentation:
> - [Bash Programmable Completion](https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion.html)
> - [Zsh Completion System](https://zsh.sourceforge.io/Doc/Release/Completion-System.html)
> - [Fish Shell Completions](https://fishshell.com/docs/current/completions.html)
> - [PowerShell Tab Completion](https://learn.microsoft.com/en-us/powershell/scripting/learn/shell/tab-completion)

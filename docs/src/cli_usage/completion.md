# completion

## Overview

The `completion` command generates shell completion scripts for `rudof`, enabling tab completion for commands, subcommands, options, and arguments in your preferred shell.

Once installed, shell completion allows you to:

- **Tab-complete commands**: Type `rudof val<TAB>` → `rudof validate`
- **Tab-complete options**: Type `rudof validate --sh<TAB>` → `rudof validate --schema`
- **Tab-complete file paths**: Automatically suggest files and directories for file arguments
- **View available options**: Press `TAB` twice to see all available options at any point
- **Reduce typos**: Let the shell validate command names and options before execution

## Supported Shells

The `completion` command supports completion script generation for the following shells:

- **Bash** - The Bourne Again SHell, default on most Linux distributions
- **Zsh** - Z shell, default on macOS (10.15+) and popular among advanced users
- **Fish** - The friendly interactive shell with built-in completion support
- **PowerShell** - Microsoft PowerShell for Windows, Linux, and macOS
- **Elvish** - A modern shell with a unique approach to scripting

## Command Syntax

```bash
rudof completion <SHELL> [OPTIONS]
```

### Arguments

- `<SHELL>` - The shell for which to generate the completion script
  - Required argument
  - Possible values: `bash`, `zsh`, `fish`, `powershell`, `elvish`
  - Case-insensitive

### Options

- `-o, --output <FILE>` - Write completion script to a file instead of stdout
- `--force-overwrite` - Overwrite the output file if it already exists

## Basic Usage

### Save to a file

You can save the completion script to a file for later installation:

```bash
# Save bash completion to a file
rudof completion bash -o rudof-completion.bash

# Save zsh completion to a file
rudof completion zsh -o _rudof

# Save fish completion to a file
rudof completion fish -o rudof.fish

# Save PowerShell completion to a file
rudof completion powershell -o rudof-completion.ps1
```

## Installation Instructions

After generating the completion script, you need to install it in the appropriate location for your shell. The installation process varies by shell.

> **Note**: For detailed information about completion systems, refer to the official documentation:
> - [Bash Programmable Completion](https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion.html)
> - [Zsh Completion System](https://zsh.sourceforge.io/Doc/Release/Completion-System.html)
> - [Fish Shell Completions](https://fishshell.com/docs/current/completions.html)
> - [PowerShell Tab Completion](https://learn.microsoft.com/en-us/powershell/scripting/learn/shell/tab-completion)


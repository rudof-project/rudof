#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(not(target_family = "wasm"))]
fn main() -> std::io::Result<()> {
    use clap::CommandFactory;
    use clap_complete::{Shell, generate};
    use clap_complete_nushell::Nushell;
    use rudof_cli::cli::parser::Cli;
    use std::{env, io};

    let mut args = env::args().skip(1);
    let shell = args
        .next()
        .expect("usage: rudof-completions <bash|zsh|fish|elvish|nushell|powershell>");

    let mut cmd = Cli::command();
    if shell.eq_ignore_ascii_case("nushell") {
        generate(Nushell, &mut cmd, "rudof", &mut io::stdout());
    } else {
        let shell: Shell = shell.parse().expect("unrecognized shell");
        generate(shell, &mut cmd, "rudof", &mut io::stdout());
    }

    Ok(())
}

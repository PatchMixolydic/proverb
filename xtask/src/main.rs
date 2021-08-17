#[macro_use]
mod cmd;
mod install;

#[cfg(not(unix))]
compile_error!("`proverb`'s `cargo xtask` currently doesn't support non-Unix-like systems");

use clap::{AppSettings, Clap};
use install::Install;

use crate::install::install;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    Install(Install),
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Install(args) => install(args),
    }
}

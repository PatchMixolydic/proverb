#[macro_use]
mod cmd;
mod install;
mod man_pages;

#[cfg(not(unix))]
compile_error!("`proverb`'s `cargo xtask` currently doesn't support non-Unix-like systems");

use anyhow::anyhow;
use clap::{AppSettings, Clap};
use directories::BaseDirs;
use std::{
    env,
    path::{Component, PathBuf},
};

use crate::{
    install::{install, Install},
    man_pages::{man_pages, ManPages},
};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    Install(Install),
    ManPages(ManPages),
}

// TODO: non-unix
fn install_prefix_from_env() -> Option<PathBuf> {
    env::var("PREFIX")
        .or_else(|_| env::var("CARGO_INSTALL_ROOT"))
        .or_else(|_| env::var("CARGO_HOME"))
        .map(|s| PathBuf::from(s))
        .ok()
        .or_else(|| Some(BaseDirs::new()?.home_dir().join(".cargo")))
}

fn canonicalize_prefix(maybe_prefix: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    maybe_prefix
        .map(|path| path.into())
        .or_else(install_prefix_from_env)
        .ok_or_else(|| anyhow!("couldn't determine installation prefix"))?
        .canonicalize()
        .map_err(Into::into)
}

fn canonicalize_dest_dir(maybe_dest_dir: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    maybe_dest_dir
        .unwrap_or_else(|| PathBuf::from("/"))
        .canonicalize()
        .map_err(Into::into)
}

fn combine_prefix_and_dest_dir(prefix: PathBuf, dest_dir: PathBuf) -> PathBuf {
    let unrooted_prefix = prefix
        .components()
        .filter(|component| !matches!(component, Component::RootDir | Component::Prefix(_)))
        .collect::<PathBuf>();

    dest_dir.join(unrooted_prefix)
}

fn target_dir() -> PathBuf {
    option_env!("CARGO_TARGET_DIR")
        .unwrap_or("./target")
        .into()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Install(args) => install(args),
        Subcommand::ManPages(args) => man_pages(args),
    }
}

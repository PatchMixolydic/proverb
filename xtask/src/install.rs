use anyhow::anyhow;
use clap::{AppSettings, Clap};
use directories::BaseDirs;
use std::{
    env,
    fs::read_dir,
    io::{self, stdin},
    path::{Component, PathBuf},
    process::exit,
};

use crate::cmd::DisplayCmd;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub(crate) struct Install {
    /// Don't run `cargo clean --release`.
    ///
    /// This might result in unexpected behaviour.
    /// Please be cautious!
    #[clap(short, long)]
    skip_clean: bool,

    /// Installation prefix for this package.
    ///
    /// For instance,
    /// `cargo xtask install --prefix /usr/local/`
    /// would result in the `proverb` binary
    /// being installed to `/usr/local/bin/proverb`.
    #[clap(long)]
    prefix: Option<PathBuf>,

    /// Perform the installation as if the given directory
    /// were the root directory.
    ///
    /// For instance,
    /// `cargo xtask install --prefix /usr/local/ --dest-dir ./stage`
    /// would result in the `proverb` binary being installed to
    /// `./stage/usr/local/bin/proverb`. However, the library would be
    /// built as if it would be installed to  `/usr/local`. This is useful
    /// for package maintainers.
    #[clap(long)]
    dest_dir: Option<PathBuf>,
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

fn run_elevated(command: &DisplayCmd) -> io::Result<()> {
    command.clone().prepend("sudo").run()
}

pub(crate) fn install(args: Install) -> anyhow::Result<()> {
    // TODO: non-unix
    let prefix = args
        .prefix
        .map(|path| path.into())
        .or_else(install_prefix_from_env)
        .ok_or_else(|| anyhow!("couldn't determine installation prefix"))?
        .canonicalize()?;

    let dest_dir = args
        .dest_dir
        .unwrap_or_else(|| PathBuf::from("/"))
        .canonicalize()?;

    env::set_var("PREFIX", &prefix);

    // Since `prefix` was canonicalized, it is absolute.
    // `dest_dir.join(prefix)` would just replace `dest_dir` with `prefix`.
    // Skip the root (`/`) and the prefix (`C:` on Windows), then tack on
    // `dest_dir`.
    let prefix = {
        let unrooted_prefix = prefix
            .components()
            .filter(|component| !matches!(component, Component::RootDir | Component::Prefix(_)))
            .collect::<PathBuf>();

        dest_dir.join(unrooted_prefix)
    };

    let bin_dir = prefix.join("bin");
    let data_dir = prefix.join("share").join("proverb");

    if !args.skip_clean {
        // run `cargo clean` so we can bake the correct path into the binary
        display_cmd!("cargo", "clean", "--release").run()?;
    }

    display_cmd!("cargo", "build", "--release",).run()?;
    display_cmd!("strip", "target/release/proverb").run()?;

    let install_bin = display_cmd!(
        "install",
        "-m",
        "755",
        "-D",
        "-t",
        bin_dir,
        "./target/release/proverb",
    );

    let proverb_files = read_dir("./proverb_files")?
        .filter_map(|maybe_file| maybe_file.map(|file| file.path()).ok());

    let install_data =
        display_cmd!("install", "-m", "644", "-D", "-t", data_dir).args(proverb_files);

    if install_bin.run().and_then(|_| install_data.run()).is_err() {
        eprint!(
            "Cannot install to `{}`. Try again with elevated permissions? [y/N] ",
            prefix.display()
        );

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input.make_ascii_lowercase();

        if !input.starts_with('y') {
            eprintln!("Aborting. Please run the following commands as root:");
            eprintln!("{}", install_bin);
            eprintln!("{}", install_data);
            exit(1);
        }

        run_elevated(&install_bin)
            .and_then(|_| run_elevated(&install_data))
            .unwrap_or_else(|_| {
                eprintln!(
                    "Failed to either elevate permissions or install into `{}`.",
                    prefix.display()
                );
                eprintln!("Please run the following commands as root:");
                eprintln!("{}", install_bin);
                eprintln!("{}", install_data);
                exit(1);
            });
    }

    Ok(())
}

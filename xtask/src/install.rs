use clap::{AppSettings, Clap};
use std::{
    env,
    fs::read_dir,
    io::{self, stdin},
    path::PathBuf,
    process::exit,
};

use crate::{
    canonicalize_dest_dir, canonicalize_prefix,
    cmd::DisplayCmd,
    combine_prefix_and_dest_dir,
    man_pages::{man_pages, ManPages},
    target_dir,
};

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
    #[clap(short, long)]
    prefix: Option<PathBuf>,

    /// Perform the installation as if the given directory
    /// were the root directory.
    ///
    /// For instance,
    /// `cargo xtask install --prefix /usr/local/ --dest-dir ./stage`
    /// would result in the `proverb` binary being installed to
    /// `./stage/usr/local/bin/proverb`. However, the library would be
    /// built as if it would be installed to  `/usr/local`. This is
    /// primarily useful for package maintainers.
    #[clap(short, long)]
    dest_dir: Option<PathBuf>,
}

fn run_elevated(command: &DisplayCmd) -> io::Result<()> {
    command.clone().prepend("sudo").run()
}

pub(crate) fn install(args: Install) -> anyhow::Result<()> {
    // make sure the manual pages have been generated
    man_pages(ManPages::new(args.prefix.as_ref()))?;

    // TODO: non-unix
    let prefix = canonicalize_prefix(args.prefix)?;
    let dest_dir = canonicalize_dest_dir(args.dest_dir)?;

    env::set_var("PREFIX", &prefix);

    // Since `prefix` was canonicalized, it is absolute.
    // `dest_dir.join(prefix)` would just replace `dest_dir` with `prefix`.
    // Skip the root (`/`) and the prefix (`C:` on Windows), then tack on
    // `dest_dir`.
    let prefix = combine_prefix_and_dest_dir(prefix, dest_dir);

    let bin_dir = prefix.join("bin");
    let data_dir = prefix.join("share").join("proverb");
    let manual_dir = prefix.join("share").join("man");

    if !args.skip_clean {
        // run `cargo clean` so we can bake the correct path into the binary
        display_cmd!("cargo", "clean", "--release").run()?;
    }

    display_cmd!("cargo", "build", "--release",).run()?;
    display_cmd!("strip", "target/release/proverb").run()?;

    // ./proverb_files/*
    let proverb_files = read_dir("./proverb_files")?
        .filter_map(|maybe_file| maybe_file.map(|file| file.path()).ok());

    let commands = [
        display_cmd!(
            "install",
            "-m",
            "755",
            "-D",
            "-t",
            bin_dir,
            "./target/release/proverb",
        ),

        display_cmd!("install", "-m", "644", "-D", "-t", data_dir).args(proverb_files),

        display_cmd!(
            "install",
            "-m",
            "644",
            "-D",
            "-t",
            manual_dir.join("man6"),
            target_dir().join("man_pages").join("proverb.6.gz")
        ),

        display_cmd!(
            "install",
            "-m",
            "644",
            "-D",
            "-t",
            manual_dir.join("man5"),
            target_dir().join("man_pages").join("proverb-files.5.gz")
        ),
    ];

    if commands
        .iter()
        .try_for_each(|command| command.run())
        .is_err()
    {
        eprint!(
            "Cannot install to `{}`. Try again with elevated permissions? [y/N] ",
            prefix.display()
        );

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input.make_ascii_lowercase();

        if !input.starts_with('y') {
            eprintln!("Aborting. Please run the following commands as root:");
            for command in commands {
                eprintln!("{}", command);
            }

            exit(1);
        }

        commands
            .iter()
            .try_for_each(|command| run_elevated(command))
            .unwrap_or_else(|_| {
                eprintln!(
                    "Failed to either elevate permissions or install into `{}`.",
                    prefix.display()
                );
                eprintln!("Please run the following commands as root:");
                for command in commands {
                    eprintln!("{}", command);
                }

                exit(1);
            });
    }

    Ok(())
}

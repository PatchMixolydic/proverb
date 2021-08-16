use anyhow::anyhow;
use directories::BaseDirs;
use std::{env, fs::read_dir, io::stdin, path::PathBuf, process::exit};

use crate::Install;

// TODO: non-unix
fn install_prefix_from_env() -> Option<PathBuf> {
    env::var("PREFIX")
        .or_else(|_| env::var("CARGO_INSTALL_ROOT"))
        .or_else(|_| env::var("CARGO_HOME"))
        .map(|s| PathBuf::from(s))
        .ok()
        .or_else(|| Some(BaseDirs::new()?.home_dir().join(".cargo")))
}

pub(crate) fn install(args: Install) -> anyhow::Result<()> {
    let prefix = args
        .prefix
        .map(|path| path.into())
        .or_else(install_prefix_from_env)
        .ok_or_else(|| anyhow!("couldn't determine installation prefix"))?
        .canonicalize()?;

    env::set_var("PREFIX", &prefix);

    // TODO: non-unix

    let bin_dir = prefix.join("bin");
    let data_dir = prefix.join("share").join("proverb");

    // run `cargo clean` so we can bake the correct path into the binary
    display_cmd!("cargo", "clean", "--release").run()?;
    display_cmd!(
        "cargo",
        "build",
        "--release",
        "--workspace",
        "--exclude",
        "xtask"
    )
    .run()?;
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
        display_cmd!("install", "-m", "755", "-D", "-t", data_dir,).args(proverb_files);

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

        install_bin
            .clone()
            .prepend("sudo")
            .run()
            .and_then(|_| install_data.clone().prepend("sudo").run())
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

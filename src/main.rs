use crossterm::{terminal, tty::IsTty};
use directories::BaseDirs;
use rand::{thread_rng, Rng};
use std::{
    borrow::Cow,
    fs::{read_dir, File},
    io::{stdout, Read},
    path::Path,
    process::exit,
};

fn install_prefix() -> Option<Cow<'static, Path>> {
    option_env!("PREFIX")
        .or_else(|| option_env!("CARGO_INSTALL_ROOT"))
        .or_else(|| option_env!("CARGO_HOME"))
        .map(|s| Path::new(s).into())
        .or_else(|| Some(BaseDirs::new()?.home_dir().join(".cargo").into()))
}

fn source_directories() -> Vec<Cow<'static, Path>> {
    // At least two pushes are likely to happen on all platforms.
    // Failure of `BaseDirs::new()` seems to be unlikely.
    let mut res = Vec::with_capacity(2);

    if cfg!(unix) {
        res.push(Path::new("/usr/share/proverb").into());
        res.push(Path::new("/usr/local/share/proverb").into());
    }

    if let Some(dir) = BaseDirs::new().map(|dirs| dirs.data_dir().to_path_buf()) {
        res.push(dir.join("proverb").into());
    }

    if let Some(prefix) = install_prefix() {
        res.push(prefix.join("share").join("proverb").into());
    }

    // For ease of debugging
    #[cfg(debug_assertions)]
    res.push(Path::new("./proverb").into());

    res
}

fn wrap_if_needed<'a>(s: &'a str) -> Cow<'a, str> {
    if !stdout().is_tty() {
        return s.into();
    }

    match terminal::size() {
        Ok((width, _)) => textwrap::fill(s, width as usize).into(),
        Err(_) => s.into(),
    }
}

fn random_from<T>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        Some(&list[thread_rng().gen_range(0..list.len())])
    }
}

fn main() {
    let proverb_files = source_directories()
        .into_iter()
        .map(read_dir)
        .filter(|res| res.is_ok())
        .map(|iter| {
            iter.unwrap()
                .filter(|entry| entry.is_ok())
                .map(|entry| entry.unwrap())
                .filter(|entry| entry.metadata().map(|data| data.is_file()).unwrap_or(false))
                .map(|entry| entry.path())
        })
        .flatten()
        .collect::<Vec<_>>();

    let proverb_file = random_from(&proverb_files).unwrap_or_else(|| {
        eprintln!("No fortune files found in the following directories:");
        for directory in source_directories() {
            eprintln!("{}", directory.display());
        }

        exit(1);
    });

    let proverbs_file_contents = File::open(proverb_file)
        .and_then(|mut file| {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            Ok(buf)
        })
        .unwrap_or_else(|_| {
            eprintln!(
                "TOCTOU errors can hit at the worst of times.\n\
                (Fortune file removed before it could be opened.)"
            );
            exit(1);
        });

    let proverbs_list = proverbs_file_contents
        .split('%')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let proverb = random_from(&proverbs_list).unwrap_or_else(|| {
        eprintln!(
            "No proverbs found in proverb file {}",
            proverb_file.display()
        );
        exit(1);
    });
    println!("{}", wrap_if_needed(proverb));
}

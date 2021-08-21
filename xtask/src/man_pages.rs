use clap::{AppSettings, Clap};
use flate2::{write::GzEncoder, Compression};
use roff::*;
use std::{
    fs::{create_dir_all, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use crate::{canonicalize_prefix, target_dir};

const ROFF_PARAGRAPH: &str = "\n.PP\n";
const ROFF_LINE_BREAK: &str = "\n.br\n";

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub(crate) struct ManPages {
    /// Installation prefix for this package.
    ///
    /// For instance,
    /// `cargo xtask man_pages --prefix /usr/local/`
    /// would result in the manual page being installed
    /// to `/usr/local/share/man/man6/proverb.1.gz`.
    #[clap(short, long)]
    prefix: Option<PathBuf>,
}

impl ManPages {
    pub fn new<P: Into<PathBuf>>(maybe_prefix: Option<P>) -> Self {
        Self {
            prefix: maybe_prefix.map(|prefix| prefix.into()),
        }
    }
}

fn write_man_page(page: Roff, page_name: &str, section: u8) -> io::Result<()> {
    let page_directory = target_dir().join("man_pages");

    create_dir_all(&page_directory)?;
    let output_file = File::create(page_directory.join(format!("{}.{}.gz", page_name, section)))?;
    let mut writer = GzEncoder::new(BufWriter::new(output_file), Compression::best());
    writer.write_all(page.render().as_bytes())?;
    Ok(())
}

fn example(contents: impl Troffable) -> String {
    format!(".EX\n{}\n.EE", contents.render())
}

pub(crate) fn man_pages(args: ManPages) -> anyhow::Result<()> {
    let prefix = canonicalize_prefix(args.prefix)?;

    let mut proverb_dirs_list = vec![
        italic("/usr/share/proverb/*"),
        italic("/usr/local/share/proverb/*"),
        italic("$HOME/.local/share/proverb/*"),
        italic(&format!("{}/share/proverb/*", prefix.display())),
    ];
    // Remove any duplicates in case `prefix` was set to something
    // like `/usr/local`.
    proverb_dirs_list.sort_unstable();
    proverb_dirs_list.dedup();

    let files_section = proverb_dirs_list
        .iter()
        .map(|dir| {
            list(
                &[dir.as_str()],
                &[
                    "proverb files; see ".into(),
                    bold("proverb-files"),
                    "(5).\n".into(),
                ],
            )
        })
        .collect::<Vec<_>>();

    let proverb = Roff::new("proverb", 6)
        .section(
            "name",
            &["proverb - print a random, likely uninteresting, adage"],
        )
        .section("synopsis", &[bold("proverb")])
        .section(
            "description",
            &[
                bold("proverb"),
                " prints a random proverb when invoked. Proverbs are read from ".into(),
                bold("proverb-files"),
                "(5).".into(),
            ],
        )
        .section("files", &files_section)
        .section(
            "copyright",
            &[
                "SPDX-License-Identifier: ".into(),
                bold("MIT"),
                " OR ".into(),
                bold("Apache-2.0"),
            ],
        )
        .section("see also", &[bold("proverb-files"), "(5)".into()]);
    write_man_page(proverb, "proverb", 6)?;

    let proverb_files = Roff::new("proverb-files", 5)
        .section(
            "name",
            &[
                "proverb files - contain various, likely uninteresting, adages for ".into(),
                bold("proverb"),
                "(6)".into(),
            ],
        )
        .section("synopsis", &[proverb_dirs_list.join(ROFF_LINE_BREAK)])
        .section(
            "description",
            &[
                ROFF_PARAGRAPH.into(),
                bold("proverb files"),
                " store a number of proverbs. These proverbs are UTF-8 strings separated by "
                    .into(),
                "a line containing only a percent sign ('%'). In this way, they are similar to ".into(),
                bold("strfile"),
                "(8)'s text files. However, there are some differences between the two formats:".into(),

                ROFF_PARAGRAPH.into(),
                "* Leading/trailing percent signs are not required (though are accepted).".into(),
                ROFF_LINE_BREAK.into(),
                "* ".into(),
                bold(".dat"),
                " files are not required (or used).".into(),
            ],
        )
        .section("examples", &[example(
            "A proverb.\n\
            %\n\
            \"Why am I writing a manual page at 2am?\"\n\
                — the author of this page\n\
            %\n\
            Another proverb!"
        )])
        .section(
            "copyright",
            &[
                "SPDX-License-Identifier: ".into(),
                bold("MIT"),
                " OR ".into(),
                bold("Apache-2.0"),
            ],
        )
        .section("see also", &[bold("proverb"), "(6)".into()]);
    write_man_page(proverb_files, "proverb-files", 5)?;

    Ok(())
}

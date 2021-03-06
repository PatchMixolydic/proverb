# proverb
Print a random, likely uninteresting, adage.

~~Blatant ripoff of~~ Inspired by `fortune(6)`. Wraps proverbs to fit your
terminal for your reading convenience.

## Building and installing
For debugging, simply use `cargo run`.

**Important:** Since this crate relies on external data files, it does not
support being installed by `cargo install`. If you do not wish to make a release
build (`cargo build --release && strip target/release/proverb`) and install it
yourself, please use `cargo xtask install`.

`cargo xtask install` is intended to be simple to use. It takes two optional
arguments:
* `--prefix` – the installation prefix. For example,
`cargo xtask install --prefix /usr/local` would build the `proverb` binary for
installation into `/usr/local/bin/proverb`.
* `--dest-dir` – the root directory for the installation. For example,
`cargo xtask install --prefix /usr/local --dest-dir ./stage` would build the
`proverb` binary for installation into `/usr/local/bin/proverb`, but would place
it in `./stage/usr/local/bin/proverb`. By default, this is set to `/`. This
argument is primarily useful for package maintainers.

If `--prefix` is not provided, the prefix is determined
by the first non-empty value in this list (patterned after `cargo install`):
* `$PREFIX`
* `$CARGO_INSTALL_ROOT`
* `$CARGO_HOME`
* `${HOME}/.cargo`

When built manually, the `proverb` binary will use this same method to try and
determine the installation prefix.

Note that `cargo xtask install` does not currently support non-Unix-like
platforms. Any help with improving platform support is welcome!

## Data files
Data files are similar to classic `fortune` strfiles:
```
A proverb.
%
"Wow, multi-line proverbs! Where do I sign up?"
    — Nobody
%
Another proverb!
```

Proverbs are UTF-8 strings separated by a line containing only "%"
(in other words, proverbs are separated by "\n%\n").

Differences include:
* A trailing percent sign is not required (though is accepted).
* `.dat` files are not required (or used). Proverb files are most likely small
  enough to fit into memory on any computer made in the past two decades
  (ignoring thin clients and the like).

Data files can be placed in:
* `${PREFIX}/share/proverb`
* Debug mode only: `./proverb_files`
* Unix-likes only:
  * `/usr/share/proverb`
  * `/usr/local/share/proverb`
  * (`$XDG_DATA_HOME` if set, else `$HOME/.local/share`)`/proverb`
* macOS only: `$HOME/Library/Application Support`
* Windows only: `%AppData%\proverb`

## License
[MIT] OR [Apache-2.0].

**One exception** is [`./proverb_files/touhou`], which uses characters and
quotations from the Touhou Project series. I do not have enough legal knowledge
to know whether or not this would qualify as fair use. According to the
[Guidelines for Touhou Project Fan Creators] ([archived]),
it seems that this file cannot be distributed commercially. Noncommercial use
and distribution of this file, as well as commercial distribution of the rest
of this project, still appear to permitted by the aforementioned guidelines.

[MIT]: https://github.com/PatchMixolydic/proverb/blob/main/LICENSE-MIT
[Apache-2.0]: https://github.com/PatchMixolydic/proverb/blob/main/LICENSE-APACHE
[`./proverb_files/touhou`]: https://github.com/PatchMixolydic/proverb/blob/main/proverb_files/touhou
[Guidelines for Touhou Project Fan Creators]: https://touhou-project.news/guidelines_en/
[archived]: https://web.archive.org/web/20210607213949/https://touhou-project.news/guidelines_en/

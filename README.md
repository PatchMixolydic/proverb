# proverb
Print a random, likely uninteresting, adage.

~~Blatant ripoff of~~ Inspired by `fortune(6)`.

## Building and installing
For debugging, simply use `cargo run`.

**Important:** Since this crate relies on external data files, it does not
support being installed by `cargo install`. If you do not wish to make a release
build (`cargo build --release && strip target/release/proverb`) and install it
yourself, please use `install.sh` (a Windows-compatible solution is welcome!).

`install.sh` is intended to be simple to use. It takes one optional argument
representing the installation prefix (ex. `install.sh /usr/local` would install
the binary to `/usr/local/bin/proverb`). If no argument is provided, the prefix
is determined by the first non-empty value in this list (patterned after
`cargo install`):
* `$PREFIX`
* `$CARGO_INSTALL_ROOT`
* `$CARGO_HOME`
* `${HOME}/.cargo`

When built manually, the `proverb` binary will use this same method to try and
determine the installation prefix.

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

Proverbs are UTF-8 strings separated by a line containing only `%`
(in other words, proverbs are separated by `\n%\n`).

Differences include:
* A trailing percent sign is not required (though is accepted).
* `.dat` files are not required (or used). Proverb files are most likely small
  enough to fit into memory on any computer made in the past two decades
  (ignoring thin clients and the like).

Data files can be placed in:
* `${PREFIX}/share/proverb`
* Debug mode only: `./proverb`
* Unix-likes only:
  * `/usr/share/proverb`
  * `/usr/local/share/proverb`
  * (`$XDG_DATA_HOME` if set, else `$HOME/.local/share`)`/proverb`
* macOS only: `$HOME/Library/Application Support`
* Windows only: `%AppData%\proverb`

## License
[MIT] OR [Apache-2.0].

**One exception** is [`./proverb/touhou`], which uses characters and
quotations from the Touhou Project series. I do not have enough legal knowledge
to know whether or not this would qualify as fair use. According to the
[Guidelines for Touhou Project Fan Creators] ([archived]),
it seems that this file cannot be distributed commercially. Noncommercial use
and distribution of this file, as well as commercial distribution of the rest
of this project, still appear to permitted by the aforementioned guidelines.

[MIT]: https://github.com/PatchMixolydic/proverb/blob/main/LICENSE-MIT
[Apache-2.0]: https://github.com/PatchMixolydic/proverb/blob/main/LICENSE-APACHE
[`./proverb/touhou`]: https://github.com/PatchMixolydic/proverb/blob/main/proverb/touhou
[Guidelines for Touhou Project Fan Creators]: https://touhou-project.news/guidelines_en/
[archived]: https://web.archive.org/web/20210607213949/https://touhou-project.news/guidelines_en/

## Where are the offensive proverbs?
no

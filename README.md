# proverb
Print a random, likely uninteresting, adage.

~~Blatant ripoff of~~ Inspired by `fortune(6)`. Wraps proverbs to fit your
terminal for your reading convenience.

## Building and installing
For debugging, simply use `cargo run`.

**Important:** Since this crate relies on external data files, it does not
support being installed by `cargo install`. Instead, it uses
[`cargo parcel`](https://gitlab.com/rotty/cargo-parcel) for installation.

To install `proverb` to `~/.local`, simply run `cargo parcel install`.
You can change the installation destination using two optional arguments:

* `--prefix DIR` sets the prefix for `proverb`'s install directories. For
  example, `cargo parcel install --prefix /usr/local` will install the
  `proverb` binary into `/usr/local/bin/proverb`. This is probably the
  argument you want to set for a system-wide installation.
* `--dest-dir DIR` will relocate the installation to another directory.
  For instance, `cargo parcel install --prefix /usr/local --dest-dir build`
  will intall the proverb binary into `./build/usr/local/bin/proverb`. This
  is primarily useful for package maintainers.

To install `proverb` systemwide when `cargo` isn't installed for `root`, use the
following commands:
```console
$ # set --prefix to taste here
$ cargo parcel install --prefix /usr/local --dest-dir ./target/stage
$ # next two commands should be run as root
# chown -R root:root ./target/stage
# cp -ai ./target/stage/* / # be sure you've typed this correctly!
```

For more information on using `cargo parcel`, see
[their CLI guide](https://gitlab.com/rotty/cargo-parcel/-/blob/master/docs/cli-guide.md).
Note that `cargo parcel` currently does not seem to support non-Unix-like platforms.

When built manually, the `proverb` binary will try to determine the installation
prefix by the first non-empty value in this list (patterned after `cargo install`):
* `$PARCEL_INSTALL_PREFIX`
* `$PREFIX`
* `$CARGO_INSTALL_ROOT`
* `$CARGO_HOME`
* `${HOME}/.cargo`

If you wish to install `proverb` manually, you can build the binary using
`PREFIX=/your/prefix/path cargo build --release && strip target/release/proverb`.
Then, move `./target/release/proverb` into `$PREFIX/bin/` and the contents
of `./proverb_files` into `$PREFIX/share/proverb/`.

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

## Where are the offensive proverbs?
no

<small>(These proverb files *do*, however, contain swear words, which might be
inappropriate for younger audiences.)</small>

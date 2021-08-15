#!/bin/sh
set -eu

# Set $PREFIX to the first defined variable in this list:
# * the first argument
# * $PREFIX
# * $CARGO_INSTALL_ROOT
# * $CARGO_HOME
# * $HOME/.cargo
# This allows documentation/proverb files to be installed, which doesn't appear
# to be possible with `cargo install`.
export PREFIX=${1:-${PREFIX:-${CARGO_INSTALL_ROOT:-${CARGO_HOME:-"${HOME}/.cargo"}}}}

# `cargo clean` so we can bake the correct path into the binary
cargo clean
cargo build --release
strip target/release/proverb
install -m 755 -D -t ${PREFIX}/bin/ target/release/proverb
install -m 644 -D -t ${PREFIX}/share/proverb/ proverb/*

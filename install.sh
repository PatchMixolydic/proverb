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

try_elevate()  {
    if [ -n `which sudo` ]; then
        read -p "Cannot install to '$PREFIX'. Try elevating? [y/N] " response
        case $response in
            [Yy]* )
                # TODO: deduplicate
                sudo install -m 755 -D -t ${PREFIX}/bin/ target/release/proverb
                sudo install -m 644 -D -t ${PREFIX}/share/proverb/ proverb/*
                exit 0;;

            * ) exit 1;;
        esac
    else
        echo "Cannot install to '$PREFIX'. 'sudo' doesn't appear to be installed, so this script can't elevate for you."
        echo "If you have an alternative sudo-like (such as 'doas') installed, try setting an alias temporarily."
        echo "Otherwise, run the following commands as a user with write permissions for '$PREFIX':"
        echo
        echo "install -m 755 -D -t ${PREFIX}/bin/ target/release/proverb"
        echo "install -m 644 -D -t ${PREFIX}/share/proverb/ proverb/*"
        exit 1
    fi
}

try_run() {
    if ! $@ ; then
        try_elevate $@
    fi
}

# `cargo clean` so we can bake the correct path into the binary
cargo clean --release
cargo build --release
strip target/release/proverb
try_run install -m 755 -D -t ${PREFIX}/bin/ target/release/proverb
try_run install -m 644 -D -t ${PREFIX}/share/proverb/ proverb/*

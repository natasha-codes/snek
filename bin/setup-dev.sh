#! /usr/bin/env bash

# Symlink pre-commit hooks
root=$(git rev-parse --show-toplevel)
pushd "$root/.git/hooks" || exit
ln -s ../../.githooks/pre-commit pre-commit
popd || exit

# Install [clippy](https://github.com/rust-lang/rust-clippy)
rustup component add clippy

# [shellcheck](https://github.com/koalaman/shellcheck)
brew install shellcheck

#! /usr/bin/env bash

# Symlink pre-commit hooks
pushd ../.git/hooks || exit
ln -s ../../.githooks/pre-commit pre-commit
popd || exit

# Install [clippy](https://github.com/rust-lang/rust-clippy)
rustup component add clippy

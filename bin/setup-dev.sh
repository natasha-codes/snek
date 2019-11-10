#! /usr/bin/env sh

# Symlink pre-commit hooks
pushd ../.git/hooks
ln -s ../../.githooks/pre-commit pre-commit
popd

# Install [clippy](https://github.com/rust-lang/rust-clippy)
rustup component add clippy
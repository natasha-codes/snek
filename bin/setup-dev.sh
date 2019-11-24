#! /usr/bin/env bash

set -euo pipefail

# Symlink pre-commit hooks
root=$(git rev-parse --show-toplevel)
pushd "$root/.git/hooks"
ln -s ../../.githooks/pre-commit pre-commit
popd

# Install [clippy](https://github.com/rust-lang/rust-clippy), which only works on nightly
rustup toolchain add nightly
rustup component add clippy --toolchain nightly

# [shellcheck](https://github.com/koalaman/shellcheck)
brew install shellcheck

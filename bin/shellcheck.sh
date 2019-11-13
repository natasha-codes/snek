#! /usr/bin/env bash

set -euo pipefail

root="$(git rev-parse --show-toplevel)"
pushd "$root"

{
  # `.sh` extension
  git ls-files '*.sh'
  # `#! usr/bin/env bash` shebang
  git grep -l '^\(#! */usr/bin/env bash\)$'
  # remove duplicates from .sh + shebang
} | sort | uniq | xargs shellcheck || exit 1

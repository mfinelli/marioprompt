#!/usr/bin/env bash

set -e

# links the dev build to ~/bin for local dev/testing
# usage: ./dev.bash

if [[ $# -ne 0 ]]; then
  echo >&2 "usage: $(basename "$0")"
  exit 1
fi

if [[ ! -d ~/bin ]]; then
  echo >&2 "error: ~/bin doesn't exist!"
  exit 1
fi

if [[ -e ~/bin/mp ]]; then
  echo >&2 "error: ~/bin/mp already exists!"
  exit 1
fi

cargo build
ln -s "$(pwd)/target/debug/mp" ~/bin

exit 0

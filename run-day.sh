#!/usr/bin/env bash

#/ Run a specified Advent of Code day.
#/
#/ Arguments
#/ ---------
#/
#/ <DayNumber> - The day number, used to choose the target to run.

# Bash options for strict error checking
set -o errexit -o errtrace -o pipefail -o nounset

# Check arguments
if [[ $# -lt 1 ]]; then
    echo "ERROR: Required <DayNumber> is missing" >&2
    exit 1
fi

# Run target
NAME="$(printf "day-%02d" "$1")"
if [[ ! -d "$NAME" ]]; then
    echo "ERROR: '$NAME' doesn't exist"
    exit 1
fi

cargo run --bin "$NAME" <"$NAME/input.txt"

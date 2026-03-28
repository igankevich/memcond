#!/bin/sh

main() {
    set -x
    workdir="$(mktemp -d)"
    trap cleanup EXIT
    for cargo_toml in compile-time-tests/*/Cargo.toml; do
        dir="$(dirname "$cargo_toml")"
        cd "$dir" || exit 1
        should_fail cargo build
    done
}

should_fail() {
    "$@" >"$workdir"/output 2>&1
    ret=$?
    if test "$ret" = 0; then
        cat "$workdir"/output >&2
        return 1
    fi
    return 0
}

cleanup() {
    rm -rf "$workdir"
}

main

#!/bin/bash

set -e

case "${1:-r}" in
  r) mode=rust ;;
  c) mode=c ;;
  *) echo "usage: $0 [r|c]" >&2; exit 1 ;;
esac

run_benchmarks() {
  local repo_root rust_lib

  repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  cd "${repo_root}"

  make -s -C expat/tests/benchmark clean

  if [ "$mode" = "rust" ]; then
    rust_lib="${repo_root}/target/release/libexpat.a"

    cargo build -q --release

    if [ ! -f "${rust_lib}" ]; then
      echo "Rust static library not found at ${rust_lib}" >&2
      return 1
    fi

    make -s -C expat run-benchmark benchmark_LDADD="${rust_lib}"
  else
    make -s -C expat run-benchmark
  fi
}

echo -e "running benchmarks using libexpat written in \033[1;34m$mode\033[0m"
run_benchmarks

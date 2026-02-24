#!/bin/bash

set -e

case "${1:-r}" in
  r) mode=rust ;;
  c) mode=c ;;
  *) echo "usage: $0 [r|c]" >&2; exit 1 ;;
esac

run_perl_xml_parser_tests() {
  local repo_root xml_parser_dir xml_parser_ref expat_include_dir rust_lib_dir rust_lib

  repo_root="$(pwd)"
  xml_parser_dir="${repo_root}/target/xml-parser"
  xml_parser_ref="${XML_PARSER_REF:-2.47}"
  expat_include_dir="${repo_root}/expat/lib"
  rust_lib_dir="${repo_root}/target/debug/perl-link"
  rust_lib="${repo_root}/target/debug/liblibexpat.a"

  rm -rf "${xml_parser_dir}"
  git clone --depth 1 --branch "${xml_parser_ref}" https://github.com/cpan-authors/XML-Parser "${xml_parser_dir}"

  mkdir -p "${rust_lib_dir}"
  ln -sf "${rust_lib}" "${rust_lib_dir}/libexpat.a"

  (
    cd "${xml_parser_dir}"
    perl Makefile.PL EXPATLIBPATH="${rust_lib_dir}" EXPATINCPATH="${expat_include_dir}"
    make -j"$(nproc)"
    make test
  )
}

build_expat_runtests() {
  local mode="$1"

  rm -f expat/tests/runtests

  if [ "$mode" = "rust" ]; then
    local ldadd='../../target/debug/liblibexpat.a'
    cargo build --features expat_test_shims
    make -C expat/tests runtests runtests_LDADD="$ldadd"
  else
    make -C expat/tests runtests
  fi

  expat/run.sh expat/tests/runtests
}

echo -e "running built-in tests using libexpat written in \033[1;34m$mode\033[0m"
build_expat_runtests "$mode"

if [ "$mode" = "rust" ]; then
  echo -e "running Perl XML::Parser integration tests using libexpat written in \033[1;34m$mode\033[0m"
  run_perl_xml_parser_tests
fi

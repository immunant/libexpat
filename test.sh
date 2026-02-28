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

run_rust_xmlwf_regression_tests() {
  local repo_root xmlwf_bin xmlwf_dir

  repo_root="$(pwd)"
  xmlwf_bin="${repo_root}/target/debug/xmlwf"
  xmlwf_dir="${repo_root}/target/xmlwf-regression"

  cargo build --bin xmlwf --features xml-testing

  rm -rf "${xmlwf_dir}"
  mkdir -p "${xmlwf_dir}/wellformed" "${xmlwf_dir}/notwellformed"

  cat > "${xmlwf_dir}/wellformed/simple.xml" <<'XML'
<root/>
XML
  cat > "${xmlwf_dir}/wellformed/namespaces.xml" <<'XML'
<root xmlns:ns="urn:test"><ns:item attr="value">text</ns:item></root>
XML
  cat > "${xmlwf_dir}/wellformed/cdata.xml" <<'XML'
<root><![CDATA[some <escaped> text]]></root>
XML

  cat > "${xmlwf_dir}/notwellformed/unclosed.xml" <<'XML'
<root>
XML
  cat > "${xmlwf_dir}/notwellformed/mismatch.xml" <<'XML'
<root><item></root>
XML
  cat > "${xmlwf_dir}/notwellformed/undefined_entity.xml" <<'XML'
<root>&missing;</root>
XML

  for file in "${xmlwf_dir}/wellformed/"*.xml; do
    "${xmlwf_bin}" -p "${file}" >/dev/null
  done

  for file in "${xmlwf_dir}/notwellformed/"*.xml; do
    if "${xmlwf_bin}" -p "${file}" >/dev/null 2>&1; then
      echo "xmlwf expected parse failure for ${file}" >&2
      return 1
    fi
  done
}

build_expat_runtests() {
  local mode="$1"

  rm -f expat/tests/runtests expat/tests/runtests_cxx

  if [ "$mode" = "rust" ]; then
    local ldadd='../../target/debug/liblibexpat.a'
    cargo build --features xml-testing
    make -C expat/tests \
      runtests \
      runtests_cxx \
      runtests_LDADD="$ldadd" \
      runtests_cxx_LDADD="$ldadd"
  else
    make -C expat/tests runtests runtests_cxx
  fi

  expat/run.sh expat/tests/runtests
  expat/run.sh expat/tests/runtests_cxx
}

echo -e "running built-in tests using libexpat written in \033[1;34m$mode\033[0m"
build_expat_runtests "$mode"

if [ "$mode" = "rust" ]; then
  echo -e "running Rust xmlwf regression tests using libexpat written in \033[1;34m$mode\033[0m"
  run_rust_xmlwf_regression_tests

  echo -e "running Perl XML::Parser integration tests using libexpat written in \033[1;34m$mode\033[0m"
  run_perl_xml_parser_tests
fi

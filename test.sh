#!/bin/bash

set -e

case "${1:-r}" in
  r) mode=rust ;;
  c) mode=c ;;
  *) echo "usage: $0 [r|c]" >&2; exit 1 ;;
esac


rm expat/tests/runtests || true

if [ "$mode" = "rust" ]; then
  cargo build
  LDADD='../../target/debug/liblibexpat.a'
  make -C expat/tests runtests runtests_LDADD="$LDADD"
else
  make -C expat/tests runtests
fi

echo -e "testing libexpat written in \033[1;34m$mode\033[0m"
expat/run.sh expat/tests/runtests

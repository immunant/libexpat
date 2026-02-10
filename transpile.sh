#!/bin/bash

set -ex

for tool in cmake make cargo c2rust rustfmt; do
    command -v "$tool" >/dev/null || { echo "error: '$tool' is not in PATH" >&2; exit 1; }
done

# TODO: figure out how to build the C tests without building the C++ tests
cmake_options=(
    -DEXPAT_BUILD_TESTS=OFF
    -DEXPAT_BUILD_EXAMPLES=OFF
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
)
(cd expat && \
    ./buildconf.sh && \
    mkdir -p build && cd build && \
    CFLAGS="${CFLAGS:+$CFLAGS }-DXML_TESTING" \
    CC=clang LD=clang cmake "${cmake_options[@]}" .. && \
    make all \
)

# remove -Wstrict-aliasing=3 from compile_commands.json, as it causes warnings
sed -i 's/ -Wstrict-aliasing=3//g' "$PWD/expat/build/compile_commands.json"

c2rust transpile --emit-build-files --emit-c-decl-map \
    --binary xmlwf --output-dir $PWD --overwrite-existing \
    $PWD/expat/build/compile_commands.json

cargo fix --bin "xmlwf" --allow-dirty --allow-staged

# TODO: run c2rust refactor to remove unsafe blocks, unnecessary casts, etc.
# TODO: run c2rust postprocess to recover comments, format the code, etc.

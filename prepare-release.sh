#!/usr/bin/env bash

set -e

OUT_DIR=ci-release/git-mob
mkdir -p $OUT_DIR/{bin,man}

for binary in git-{mob,solo,{add,edit,delete}-coauthor}; do
    gzip --to-stdout target/release/$binary.1 > $OUT_DIR/man/$binary.gz
    cp target/release/$binary $OUT_DIR/bin/$binary
done

#!/usr/bin/env bash

set -e

mkdir -p target/man
for page in git-{mob,solo,{add,edit,delete}-coauthor}; do
    mdsh --work_dir . -o - -i docs/$page.md | \
        mandown - "$(echo $page | tr '[:lower:]' '[:upper:]')" 1 | \
        gzip > target/man/$page.1.gz
done

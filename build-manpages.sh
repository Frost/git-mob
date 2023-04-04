#!/usr/bin/env bash

set -e

mkdir -p target/man
for page in git-{mob,solo,{add,edit,delete}-coauthor}.1; do
    gzip --to-stdout target/release/$page > target/man/$page.gz
done

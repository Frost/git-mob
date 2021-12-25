#!/usr/bin/env bash

mkdir -p target/man
for page in git-{mob,solo,{add,edit,delete}-coauthor}; do
    mdsh --work_dir . -o - -i docs/$page.md | mandown - ${page^^} 1 | gzip > target/man/$page.1.gz
done

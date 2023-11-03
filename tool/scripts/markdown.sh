#!/bin/bash

# Uses https://github.com/cooperrunyan/mdp to turn the md files
# into html in target/md/

files=(
    "README.md"
    "asm/README.md"
    "asm/src/builtin/core/README.md"
)

for file in ${files[@]}; do
    base="$(basename $file)"
    dir="./target/md/$(dirname $file)"
    output="$dir/${base/.md/.html}"
    mkdir -p "$dir"
    mdp "$file" "$output"
done

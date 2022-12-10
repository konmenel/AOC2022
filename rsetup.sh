#!/bin/bash

usage() {
    echo "Usage: $0 [daynumber]"
}

is_positive_int() {
    [[ $1 -le 0 ]]  2> /dev/null && return 1 || return 0
}

if [[ $# -ne 1 ]]; then
    echo "Incorrect number of arguments"
    usage
    exit 1
fi

if [[ !($1 =~ ^[0-9]+$) ]] || [[ $1 -le 0 ]]; then
    echo "Argument is not a positive integer"
    usage
    exit 1
fi

daynumber=$1
filename=$(printf "./src/day%02d.rs" $daynumber)
cargo_str=

if test -f $filename; then
    echo "File \"$filename\" already exists"
    exit 1
fi

printf "\n[[bin]]\nname = \"day%02d\"\npath = \"src/day%02d.rs\"\n" $daynumber >> Cargo.toml

replace_str=$(printf "let day: u32 = %d" $daynumber)
sed "s/let day: u32 = XX/$replace_str/g" ./template.rs > $filename


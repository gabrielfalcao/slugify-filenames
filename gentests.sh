#!/usr/bin/env bash

IFS="
"
script_path="$(cd "$(dirname ${BASH_SOURCE[0]})" && pwd)"
tests_path="${script_path}/tests"
fixtures_path="${script_path}/tests/fixtures"
set -e
cargo build

variations() {
    declare -a items=( " " "@" "#" "\x1b[1;38;5;101m" )
    for _ in $(seq 2); do
        items+=$(random_filename $(random_number_nonzero))
    done
    for v in ${items[@]}; do
        echo "$v"
    done
}

random_number_nonzero() {
    no=0
    while [ $no -eq 0 ]; do
        no=$(( $(2>/dev/random dd if=/dev/random bs=1 count=1 | xxd -p | xargs printf "0x%s" | xargs printf "%d") + 0 ))
    done
    echo -n $no
}

random_number_greater_than() {
    min=$(( ${1} + 0 ))
    if [ $min -eq 0 ]; then
        min=3
    fi
    no=0
    while [ $no -lt $min ]; do
        no=$(( $(2>/dev/random dd if=/dev/random bs=1 count=1 | xxd -p | xargs printf "0x%s" | xargs printf "%d") + 0 ))
    done
    echo -n $no
}

random_hex() {
    2>/dev/random dd if=/dev/random bs=4 count=$(random_number_nonzero) | xxd -p

}
random_filename() {
    number=$(( ${1} + 0 ))
    if [ $number -eq 0 ]; then
        number=3
    fi
    if [ $(( $number % 3 )) -eq 0 ] || [ $(( $number % 7 )) -eq 0 ]; then
        2>/dev/random dd if=/dev/random bs=1 count=$(random_number_greater_than 3) | base64 -w0 | sed 's/^\(.\{1,200\}\).*/\1/g'
    else
        2>/dev/random dd if=/dev/random bs=1 count=$(random_number_greater_than 3) | xxd -p | sed 's/^\(.\{1,200\}\).*/\1/g'
    fi
}
multiplex_variations() {
    index=$(( ${1} + 0 ))
    shift
    base_path=${1/%\//}
    for variation in $(variations); do
        echo "${base_path}/path ${variation}${index}"
        echo "${base_path}/name${variation}${index}"
        echo "${base_path}/${variation}path${index}"
        echo "${base_path}/${index}name${variation}"
        echo "${base_path}/${variation}${index}"
        echo "${base_path}/${index}${variation}"
    done
}

create_dir() {
    index=$(( ${1} + 0 ))
    shift
    path="$@"
    mkdir -p "$(dirname "$path")"
    mkdir -p "${path}"
    1>&2 echo "${index} => ${path}"
}
create_file() {
    index=$(( ${1} + 0 ))
    shift
    path="$@"
    mkdir -p "$(dirname "$path")"
    random_hex > "${path}"
    1>&2 echo "${index} => ${path}"
}

rm -rf "${fixtures_path}"
for c in $(seq 1 $(( $count + 1 ))); do
    for path in $(multiplex_variations $c "${fixtures_path}/folder"); do
        create_dir "$c" "$path"
        create_file "$c" "${path}/file ${c}"
    done
    for path in $(multiplex_variations $c "${fixtures_path}/file"); do
        create_file "$c" "${path}"
    done
done

cargo run -- "${fixtures_path}"

cargo install --path .

(cd "${fixtures_path}" && slugify-filenames .)

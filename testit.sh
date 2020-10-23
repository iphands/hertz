#!/bin/bash

{
    killall chrome
    killall xosview
    killall conky
    killall slack
    rm ./data/${1}.out
} 2>/dev/null

set -e
cargo build --release


for var in `seq 0 15`
do
    let mask=10**$var
    # echo "var: $var"
    # echo "mas: $mask"
    hex_mask=`printf '%x\n' "$((2#$mask))"`
    echo -n "Testing cpu $var: "
    sleep 1
    taskset 0x${hex_mask} ./target/release/hertz "burnP5" >> ./data/${1}.cpu${var}.out
    echo DONE
done

echo $1
cat data/${1}.cpu*out | sort -n | tail -n 50

#!/bin/bash
cd ./data
for SPD in `ls *out | sed -s 's/.cpu.*.out//g' | sort -u`
do
    echo $SPD > /tmp/all_$SPD.out
    for var in `seq 0 15`
    do
        tail -n 38 ./$SPD.cpu$var.out
    done >> /tmp/all_$SPD.out
done

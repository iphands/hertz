#!/bin/bash

i=0
sum=`cat /proc/cpuinfo | md5sum`
while true
do
    new_sum=`cat /proc/cpuinfo | md5sum`
    if [[ "$sum" != "$new_sum" ]]
    then
        break
    fi
    i=$((i+1))
done

echo $i

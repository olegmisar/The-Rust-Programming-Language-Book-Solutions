#!/bin/bash

for dir in ./*/
do
    (
        cd $dir
        cargo test
    )
done

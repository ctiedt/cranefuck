#!/bin/sh

cargo run --release -- $1
ld -o a /lib/crt1.o a.out -lc
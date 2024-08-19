#!/bin/sh

cargo run --release -- "$1"
if [ -f /lib/crt1.o ]; then
    ld -o a /lib/crt1.o a.out -lc
elif [ -f /usr/lib32/crt1.o ]; then
    ld -o a /usr/lib32/crt1.o a.out -lc
else
    echo "No crt1.o found"
    exit 1
fi

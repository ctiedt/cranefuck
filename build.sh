#!/bin/sh

cargo run --release -- "$1"
if [ -f /lib/crt1.o ]; then
    LIB_PATH=/lib
    DYN_LINKER=ld-linux-x86-64.so.2
elif [ -f /lib64/crt1.o ]; then
    LIB_PATH=/lib64
    DYN_LINKER=ld-linux-x86-64.so.2
elif [ -f /usr/lib32/crt1.o ]; then
    LIB_PATH=/usr/lib32
    DYN_LINKER=ld-linux.so.2
elif [ -f /usr/lib64/crt1.o ]; then
    LIB_PATH=/usr/lib64
    DYN_LINKER=ld-linux-x86-64.so.2
else
    echo "No crt1.o found"
    exit 1
fi

ld -o a -dynamic-linker $LIB_PATH/$DYN_LINKER $LIB_PATH/crt1.o $LIB_PATH/crti.o -lc a.out $LIB_PATH/crtn.o

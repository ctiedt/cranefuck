#!/usr/bin/python
# A simple tool to generate a program to print a brainfuck string
import sys

def print_bf(s: str):
    special = False
    for char in s:
        if char == "\\":
            special = True
            continue
        if special:
            if char == "n":
                rep = ord("\n")
        else:
            rep = ord(char)
        print("+" * rep, end="")
        print(".>")
        special = False

if __name__ == "__main__":
    print_bf(sys.argv[1])
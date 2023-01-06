import sys

def print_bf(s: str):
    for char in s:
        rep = ord(char)
        print("+" * rep, end="")
        print(".>")

if __name__ == "__main__":
    print_bf(sys.argv[1])
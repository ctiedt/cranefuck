# A brainfuck compiler using cranelift

This is a compiler for the brainfuck programming language based on the
[cranelift](https://docs.rs/cranelift) framework.

## Examples

This project includes various examples that should all compile and run (please
open an issue if any of them doesn't work for you!).

You can also use the `print_bf.py` file to generate brainfuck code to print a
string.

## Linking

This project only provides the compiler. To get an executable for your platform,
you will need to link the object file built by the compiler. Scripts are
included to link with `ld` on Linux and `link.exe` on Windows.

## Building on Windows

To compile brainfuck code on windows, you will need MSVC build tools and
`ucrt.lib`. You can install these via the Visual Studio Installer and will need
to put the correct path for `ucrt.lib` into the `link.exe` invocation in
`build.ps1`. Please note that I am not currently using any Windows systems, so
compiling and linking on Windows may fail if Microsoft changes anything about
the process.

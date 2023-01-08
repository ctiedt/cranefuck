# A brainfuck compiler using cranelift

This is a (still relatively barebones) compiler for the brainfuck programming language.
It is currently not usable for all brainfuck use cases yet.

## Building on Windows

To compile brainfuck code on windows, you will need MSVC build tools and `ucrt.lib`.
You can install these via the Visual Studio Installer and will need
to put the correct path for `ucrt.lib` into the `link.exe` invocation
in `build.ps1`.
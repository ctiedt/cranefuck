param([String]$source)

cargo run --release -- $source
link.exe -out:a.exe -entry:main a.obj "C:\Program Files (x86)\Windows Kits\10\Lib\10.0.10240.0\ucrt\x64\ucrt.lib"
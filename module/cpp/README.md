# C++ Example of a module

This command builds the module

`clang --target=wasm32 -nostdlib -Wl,--export-all -Wl,--no-entry -Wl,--export-dynamic -Os main.cc -o main.wasm`
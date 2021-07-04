#ifndef WASM_H
#define WASM_H

#ifndef __GNUC__
#define __attribute__(x) /*NOTHING*/
#endif

#define WASM_EXPORT __attribute__((visibility("default"))) extern "C"
#define WASM_IMPORT(module, name) __attribute__((import_module(module), import_name(name), visibility("hidden"))) extern "C"

#endif

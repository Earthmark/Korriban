#ifndef FIELD_H
#define FIELD_H

#include "wasm.h"

#include <stdint.h>

template<typename T>
struct V3 {
    T x;
    T y;
    T z;
};

WASM_IMPORT("field", "get_i32") void get_i32(int index, int32_t& value);
WASM_IMPORT("field", "get_f32") void get_f32(int index, float& value);
WASM_IMPORT("field", "get_v3_f32") void get_v3_f32(int index, V3<float>& value);

#endif

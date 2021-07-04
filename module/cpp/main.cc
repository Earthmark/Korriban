#include "wasm.h"
#include "field.h"

WASM_EXPORT float exec() {
    V3<float> f;
    get_v3_f32(0, f);
    return f.x * 100 + f.y * 10 + f.z;
}
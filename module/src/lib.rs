use korriban_elem::field::*;

#[no_mangle]
pub extern "C" fn do_thing() -> f32 {
    let vec = get_v3_f32(0);
    let n = vec.x * 10.0 + vec.y * 5.0 + vec.z;
    n
}
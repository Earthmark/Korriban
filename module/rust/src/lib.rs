use korriban_elem::field::*;

#[no_mangle]
pub extern "C" fn exec() -> f32 {
    let vec = get_f32(0);
    let n = vec * 100.0;
    n
}

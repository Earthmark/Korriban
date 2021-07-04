mod internal {
    macro_rules! def {
        ($name:ident, $typ:ty) => {
            pub fn $name(i: i32, v: &mut $typ);
        };
    }
    
    #[link(wasm_import_module = "field")]
    extern "C" {
        def!(get_i32, i32);
        def!(get_f32, f32);
        def!(get_v3_f32, glam::Vec3);
    }
}

macro_rules! def {
    ($name:ident, $typ:ty) => {
        pub fn $name(i: i32) -> $typ {
            let mut v = <$typ>::default();
            unsafe {
                internal::$name(i, &mut v);
            }
            v
        }
    };
}

def!(get_i32, i32);
def!(get_f32, f32);
def!(get_v3_f32, glam::Vec3);

use wasmer::{
    imports, Function, ImportObject, LazyInit, Memory, Store, ValueType, WasmPtr, WasmerEnv,
};

use glam::Vec3;

pub trait SingleFieldProvider<Value: Copy + ValueType + Default>: WasmerEnv + Clone {
    fn get(&self, index: u32) -> Value;
    fn set(&self, index: u32, value: Value);
}

pub trait FieldProvider: SingleFieldProvider<i32> + SingleFieldProvider<f32> {}

#[derive(WasmerEnv, Clone)]
struct Env<TFieldProvider>
where
    TFieldProvider: FieldProvider,
{
    #[wasmer(export)]
    memory: LazyInit<Memory>,
    props: TFieldProvider,
}

trait WrappedValueType: ValueType {}

#[derive(Copy, Clone, Default)]
#[repr(C)]
struct V3W {
    v: Vec3,
}

unsafe impl ValueType for V3W {}

pub fn make_exports(store: &Store, props: impl FieldProvider + 'static) -> ImportObject {
    let env = Env {
        memory: LazyInit::default(),
        props,
    };
    imports! {
        "field" => {
            "get_i32" => Function::new_native_with_env(store, env.clone(), Env::get_i32),
            "get_f32" => Function::new_native_with_env(store, env.clone(), Env::get_f32),

            "set_i32" => Function::new_native_with_env(store, env.clone(), Env::set_i32),
            "set_f32" => Function::new_native_with_env(store, env.clone(), Env::set_f32),
        }
    }
}

macro_rules! bind_field_extractor {
    ($get_name:ident, $set_name:ident, $type:ty) => {
        fn $get_name(state: &Env<TFieldProvider>, index: u32, dest: WasmPtr<$type>) {
            if let Some(mem) = state.memory.get_ref() {
                if let Some(dest) = dest.deref(mem) {
                    dest.set(state.props.get(index));
                }
            }
        }

        fn $set_name(state: &Env<TFieldProvider>, index: u32, src: WasmPtr<$type>) {
            if let Some(mem) = state.memory.get_ref() {
                if let Some(src) = src.deref(mem) {
                    state.props.set(index, src.get());
                }
            }
        }
    };
}

impl<TFieldProvider: FieldProvider> Env<TFieldProvider> {
    bind_field_extractor!(get_i32, set_i32, i32);
    bind_field_extractor!(get_f32, set_f32, f32);
}

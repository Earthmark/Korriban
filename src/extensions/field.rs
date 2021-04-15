use wasmer::{imports, ImportObject, WasmPtr, ValueType, Function, Store, WasmerEnv, Memory, LazyInit};

pub trait FieldProvider : WasmerEnv + Clone {
    fn get<Value: Copy + ValueType + Default>(&self, index: u32) -> Value;
    fn set<Value: Copy + ValueType>(&self, index: u32, value: Value);
}

#[derive(WasmerEnv, Clone)]
struct Env<TFieldProvider> where TFieldProvider : FieldProvider {
    #[wasmer(export)]
    pub memory: LazyInit<Memory>,
    pub props: TFieldProvider,
}

pub fn make_exports(store: &Store, props: impl FieldProvider + 'static) -> ImportObject {
    let env = Env{
        memory: LazyInit::default(),
        props,
    };
    imports! {
        "field" => {
            "get_i32" => Function::new_native_with_env(store, env.clone(), Env::field_get::<i32>),
            "get_u32" => Function::new_native_with_env(store, env.clone(), Env::field_get::<u32>),
            "get_i64" => Function::new_native_with_env(store, env.clone(), Env::field_get::<i64>),
            "get_u64" => Function::new_native_with_env(store, env.clone(), Env::field_get::<u64>),

            "set_i32" => Function::new_native_with_env(store, env.clone(), Env::field_set::<i32>),
            "set_u32" => Function::new_native_with_env(store, env.clone(), Env::field_set::<u32>),
            "set_i64" => Function::new_native_with_env(store, env.clone(), Env::field_set::<i64>),
            "set_u64" => Function::new_native_with_env(store, env.clone(), Env::field_set::<u64>),
        }
    }
}

impl<TFieldProvider : FieldProvider> Env<TFieldProvider> {
    fn field_get<Value: Copy + ValueType + Default>
    (state: &Env<TFieldProvider>, index: u32, dest: WasmPtr<Value>) {
        if let Some(mem) = state.memory.get_ref() {
            if let Some(dest) = dest.deref(mem) {
                dest.set(state.props.get(index));
            }
        }
    }
    
    fn field_set<Value: Copy + ValueType>
    (state: &Env<TFieldProvider>, index: u32, src: WasmPtr<Value>) {
        if let Some(mem) = state.memory.get_ref() {
            if let Some(src) = src.deref(mem) {
                state.props.set(index, src.get());
            }
        }
    }
}

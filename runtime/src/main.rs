mod component;
mod extensions;
mod field;
mod prop;
mod space;

use wasmer::{
    wat2wasm, Instance, LazyInit, Memory, Module, NativeFunc, Store, ValueType, WasmerEnv,
};

use crate::prop::{Cache, PropSet};
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[derive(WasmerEnv, Clone)]
struct State {
    props: Arc<RwLock<PropSet>>,
}

impl extensions::field::SingleFieldProvider<i32> for State {
    fn get(&self, index: u32) -> i32 {
        if let Ok(props) = self.props.read() {
            if let Some(target) = props.get(index as usize) {
                return *target;
            }
        }
        i32::default()
    }

    fn set(&self, index: u32, value: i32) {
        if let Ok(mut props) = self.props.write() {
            if let Some(target) = props.get_mut(index as usize) {
                *target = value;
            }
        }
    }
}

impl extensions::field::SingleFieldProvider<f32> for State {
    fn get(&self, index: u32) -> f32 {
        if let Ok(props) = self.props.read() {
            if let Some(target) = props.get(index as usize) {
                return *target;
            }
        }
        f32::default()
    }

    fn set(&self, index: u32, value: f32) {
        if let Ok(mut props) = self.props.write() {
            if let Some(target) = props.get_mut(index as usize) {
                *target = value;
            }
        }
    }
}

impl extensions::field::FieldProvider for State {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = wat2wasm(include_bytes!("../module.wat"))?;

    let store = Store::default();

    let module = Module::new(&store, wasm_bytes)?;

    let state = State {
        props: Arc::new(RwLock::new(PropSet::new())),
    };
    let import_obj = extensions::field::make_exports(&store, state.clone());

    if let Ok(mut props) = state.props.write() {
        props.allocate::<f32>(0.1);
    }

    let instance = Instance::new(&module, &import_obj)?;

    let sum: NativeFunc<(), f32> = instance.exports.get_native_function("exec")?;
    let start = Instant::now();
    for x in 0..1000 {
        if let Ok(mut props) = state.props.write() {
            if let Some(field) = props.get_mut::<f32>(0) {
                *field = x as f32;
            }
        }

        let results = sum.call()?;
    }
    let dest = Instant::now().duration_since(start);
    println!("Executed call 1000 times in {:?}", dest);

    Ok(())
}

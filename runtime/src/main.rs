mod prop;
mod extensions;
mod component;
mod services;

use wasmer::{wat2wasm, LazyInit, Instance, Module, Store, NativeFunc, ValueType, Memory, WasmerEnv};

use std::time::Instant;
use std::sync::Arc;
use crate::prop::PropSet;

#[derive(WasmerEnv, Clone)]
struct State {
  #[wasmer(export)]
  memory: LazyInit<Memory>,
  props: Arc<PropSet>,
}

#[derive(WasmerEnv, Clone)]
struct Tmp {
}

impl extensions::field::FieldProvider for Tmp {
  fn get<Value: Copy + ValueType + Default>(&self, index: u32) -> Value {
    Value::default()
  }
  fn set<Value: Copy + ValueType>(&self, index: u32, value: Value) {
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
  let wasm_bytes = wat2wasm(include_bytes!("../../module/target/wasm32-unknown-unknown/release/korriban_module.wasm"))?;
  
  let store = Store::default();

  let module = Module::new(&store, wasm_bytes)?;

  let import_obj = extensions::field::make_exports(&store, Tmp{});

  let instance = Instance::new(&module, &import_obj)?;

  let sum: NativeFunc<(), f32> = instance.exports.get_native_function("do_thing")?;
  let start = Instant::now();
  for x in 0..1000 {
    let results = sum.call()?;
  }
  let dest = Instant::now().duration_since(start);
  println!("Executed call 1000 times in {:?}", dest);

  Ok(())
}

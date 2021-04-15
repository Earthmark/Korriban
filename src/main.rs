mod prop;
mod extensions;
mod library;
mod component;

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
  let wasm_bytes = wat2wasm(include_bytes!("module.wat"))?;
  
  let store = Store::default();

  let module = Module::new(&store, wasm_bytes)?;

  let import_obj = extensions::field::make_exports(&store, Tmp{});

  let instance = Instance::new(&module, &import_obj)?;

  instance.exports.get_native_function::<(), ()>("init")?.call()?;

  let sum: NativeFunc<(i32, i32), i32> = instance.exports.get_native_function("sum")?;
  let start = Instant::now();
  for x in 0..1000 {
    let results = sum.call(x, 1)?;
  }
  let dest = Instant::now().duration_since(start);
  println!("Executed call 1000 times in {:?}", dest);

  Ok(())
}

pub mod prop;

use cgmath::Vector2;
use prop::{PropSet};

use wasmer::Func;
use wasmer_runtime::{imports, instantiate};

static WASM: &'static [u8] = include_bytes!("test.wasm");

fn main() {
    let mut props = PropSet::new();
    let alloc = props.allocate(Vector2::<u8> { x: 1, y: 2 });
    props.get_mut(&alloc).unwrap().x = 3;
    let val = props.get(&alloc).unwrap();
    println!("{}, {}", val.x, val.y);

    let import_object = imports! {};

    let instance = instantiate(WASM, &import_object).unwrap();
    let add_one: Func<i32, i32> = instance.exports.get("add_one").unwrap();

    let value = add_one.call(42).unwrap();

    println!("{}", value);
}

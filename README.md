# Korriban
A testbench for sequencing webassembly components, as well as example APIs for those components to interact.

Everything in this project is a test of different possible styles, none of them are final or plan to be used long term. These are just to play with.

## Project Structure

* `interface` contains module APIs to be used by a component.
* `module` contains an example wasm module.
* `runtime` contains the runner that can execute and interlink components exported from a module.

## To build

1. Install [rust](https://www.rust-lang.org/)
2. Run `rustup target add wasm32-unknown-unknown` to install the wasm target.
3. While in the `module` folder compile with `cargo build --release`.
4. While in the `runtime` run the project with `cargo run --release`.

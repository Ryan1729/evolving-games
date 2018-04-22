# Evolving Games

A currently WIP project with the aim of generating/evolving turn-based single-player games.

## Generator

randomly generates some `.rs` files and places them in the proper place in the player.

### Building

The usual `cargo run`. Pass something to the binary to use what you passed as the random seed.

## Player

Surrounds then generated files and when compiled produces a running game.

### Building (using Rust's native WebAssembly backend)

1. Install newest nightly Rust:

       $ curl https://sh.rustup.rs -sSf | sh

2. Install WebAssembly target:

       $ rustup target add wasm32-unknown-unknown

3. Install [cargo-web]:

       $ cargo install -f cargo-web

4. Build it:

       $ cargo web start --target wasm32-unknown-unknown --release

5. Visit `http://localhost:8000` with your browser.

[cargo-web]: https://github.com/koute/cargo-web

### Building for other backends

Replace `--target-webasm` with `--target wasm32-unknown-emscripten` or `--target asmjs-unknown-emscripten`
if you want to build it using another backend. You will also have to install the
corresponding targets with `rustup` - `wasm32-unknown-emscripten` and `asmjs-unknown-emscripten`
respectively.

___

Dual-licensed under Apache and MIT.
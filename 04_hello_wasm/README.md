# README
Demonstrates how to get a basic webassembly component building.

Following the guide [here](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

```sh
cargo install wasm-pack
```

## How it was created 
```sh
# create a new project
cargo new --lib hello-wasm          
```

## Running
```sh
cd  hello-wasm
wasm-pack build --target web

# open the vscode liveserver on index.html
```

# Resoures
* General developers guide [here](https://webassembly.org/getting-started/developers-guide/)



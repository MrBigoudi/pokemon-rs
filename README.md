# pokemon-rs
A pokemon game in rust

## Howto

To run the code in desktop mode
```sh
cargo run
```

To run the code in debug mode
```sh
RUST_LOG=debug cargo run
```

To run the code in the web
```sh
./run_wasm.sh -h
./run_wasm.sh -c
```

To just compile the code in WASM
```sh
wasm-pack build --target web  
```
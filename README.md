# Mycelium

An RPC client for substrate base chain.
This project is based from https://github.com/scs/substrate-api-client

Changes as follows:
- [X] Use of async instead of threads
- [X] Compilable to Web assembly
- [X] Minimize the use of macro in composing extrinsics
- [X] Use of RPC via http

## Usage
1. Checkout and run the [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template)

```shell
    git clone --depth=1 https://github.com/substrate-developer-hub/substrate-node-template
    cd substrate-node-template
    cargo run --release -- --dev
```

2. Run the examples
```
cargo run --example do_something
```

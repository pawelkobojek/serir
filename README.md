# Serir - SimplE Redis In Rust
## What's this?
This is a simplified (only simple GET and SET is suppoted) Redis implemented in Rust. It aims to be a drop-in replacement assuming only simple GET and SET commands are used.

This is an educational project for practicing Rust.
## How to run it?
No packages are distributed at this moment, therefore you must clone this repository and build it by yourself: 
1. `cargo build --release`.
2. `target/release/serir [--port <port_number> --num-workers <num_workers>] `. Default port is 6379.
3. Connect with `redis-cli` and try running `GET`s and `SET`s.
4. Alternatively, run `redis-benchmark -t get,set`.


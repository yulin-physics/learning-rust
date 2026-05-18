# How about some Rust from C?

One of the joys of Rust is how well it plays with others - in both directions. Maybe you have some
awesome Rust that you want to use inside a C (or C++, or anything else that speaks the C ABI) program?

Let's start by making a Rust project:

```bash
cargo new ex03_rust_from_c --lib
```

> Note that we're making a library.

## Edit Cargo.toml to Emit a C Dynamic Library

Let's edit `Cargo.toml`:

```toml
[package]
name = "ex03_rust_from_c"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"] # Will create a .so on Linux, .dylib on Mac, .dll on Windows

[dependencies]
```
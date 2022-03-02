## [Run a code snippet in Rust](hello_world)

Make the file:

```
touch hello.rs
```

Compile code:

```
rustc hello.rs
```

Run compiled binary:

```
./hello
```

## [Build a Project in Rust](sample_code)

1. Initialise project with cargo
   To initialise in a folder:

```
cargo new [folder-name]
```

To initialise in current folder:

```
cargo init
```

2. Compile and run

```
cargo run
```

The executable is located in `./target/debug`

To build only you run the following command:

```
cargo build
```

To build for production

```
cargo build --release
```

## Recommended Visual Studio Code Extensions

- Rust Analyzer
- Better TOML

### TOML

### <a href="https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm">WebAssembly</a>

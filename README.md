## [Run a Code Snippet in Rust](hello_world)

Make the file:

```
touch [file-name].rs
```

Compile the file:

```
rustc [file-name].rs
```

Run compiled binary:

```
./[file-name]
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

    To build only run the following command:

    ```
    cargo build
    ```

    To build for production:

    ```
    cargo build --release
    ```
<a href="https://www.youtube.com/watch?v=zF34dRivLOw"> Rust Crash Course</a>

<a href="https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html">Building a Multithread Webserver</a>

Use Compiler Driven Development:
```
cargo check
```
See Generated Documentation:
```
cargo doc --open 
```
## Recommended Visual Studio Code Extensions

- Rust Analyzer
- Better TOML

### TOML

### <a href="https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm">WebAssembly</a>

fork/join model and the single-threaded async I/O model

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

1.  Create a package

    To initialise in a folder (`--vcs=none` flag overrides cargo default of initialising a new Git repository along with a .gitignore file):

    ```
    cargo new --vcs=none [folder-name]
    ```

    To initialise in current folder:

    ```
    cargo init
    ```

    - `src/main.rs` is the crate root of a binary crate
    - `src/lib.rs` is the crate root of a library crate (to create a new library, add '--lib' flag to 'cargo new')

    both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

2.  Compile and run

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

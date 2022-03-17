# Rust Code Snippets Collection

## [Run a file in Rust](01_getting_started)

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

## Build a Project in Rust

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

Use Compiler Driven Development:

```
cargo check
```

See Generated Documentation:

```
cargo doc --open
```

## [Run Tests](11_automated_tests)

`cargo test` compiles your code in test mode and runs the resulting test binary. Tests are run in parallel by default:

```
cargo test
```

Run tests consecutively using `--test-threads` flag:

```
cargo test -- --test-threads=1
```

Show printed values from the tested functions regardless of success or failure:

```
cargo test -- --show-output
```

Run `#[ignore]` tests:

```
cargo test -- --ignored
```

Run all tests including `#[ignore]`:

```
cargo test -- --include-ignored
```

Run a particular integration test crate in `tests/`:

```
cargo test --test test_file_name
```

## Recommended Visual Studio Code Extensions

- Rust Analyzer
- Better TOML

## Learning Resources

- <a href="https://doc.rust-lang.org/book/title-page.html"> The Rust Programming Language Book</a>
- <a href="https://www.youtube.com/watch?v=zF34dRivLOw"> Rust Crash Course</a>
- <a href="https://doc.rust-lang.org/nomicon/intro.html"> The Rustonomicon </a>
- <a href="https://doc.rust-lang.org/reference/introduction.html"> The Rust Reference </a>
- <a href="https://veykril.github.io/tlborm/"> The Little Book of Rust Macros</a>

### TOML

### <a href="https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm">WebAssembly</a>

fork/join model and the single-threaded async I/O model

Halting Problem

Deadlcoks

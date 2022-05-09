# Rust Code Snippets Collection

Empowering everyone to build reliable and efficient software

- [Run a File in Rust](#run-a-file-in-rust)
- [Build a Project in Rust](#build-a-project-in-rust)
- [Run Tests in Rust](#run-tests-in-rust)
- [Benchmarking in Rust](#benchmarking-in-rust)
- [Generating Crate Documentation Locally](#generating-documentation-locally)
- [Recommended VS Code Extensions](#recommended-visual-studio-code-extensions)
- [Learning Resources](#learning-resources)

## Why Rust?

Multiple concurrent Rust toolchains can be installed and managed via <a href="https://rustup.rs/">rustup</a>. Rust installations come with Cargo, a command line tool to manage dependencies, run tests, generate documentation, and more.
Rust was built with safety, speed, and efficiency (productivity and control).

- Compile time memory checks, guuarantees memory safety with no run time costs through ownership and borrowing (variables immutable by default)

- Catching bugs at compile time and ridiculously helpful compiler errors
- Best of two worlds from systems programming and functional/higher-order programming: Low level control and abstraction when required
  Zero-cost abstractions (Fluent Builder Pattern)

- Control over memory access, memory layout and specific CPU instructions, for example, semantics for creating integers:

```
// An integer on the stack
let a = 10;
// Boxed integer: An integer on the heap
let b = Box::new(20);
// A boxed integer wrapped within a reference counter
let c = Rc::new(Box::new(30));
// An integer protected by a mutual exlusion lock, wrapped in an atomic reference counter
let d = Arc::new(Mutex::new(40));
```

- Reliable language: Code written will always compile with a future Rust compiler

Rust is slower at compiling code than its peer languages, not free from logic errors.

---

### Command Line Tool

`rustup` manages Rust toolchains and move between versions of the compiler.
`cargo` manages projects.

## [Run a File in Rust](01_getting_started)

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

## [Run Tests in Rust](11_automated_tests)

`cargo test` compiles your code in test mode and runs the resulting test binary. Tests are run in parallel by default:

```
cargo test
```

Run tests consecutively using `--test-threads` flag:

```
cargo test -- --test-threads=1
```

Any arguments appearing after -- are sent through to the result of the build.

Show printed values from the tested functions regardless of success or failure:

```
cargo test -- --show-output
```

or

```
cargo test -- --nocapture
```

Rust test programs hide the stdout of successful tests in order for the test output to be tidy.

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

## [Benchmarking in Rust](benchmarking)

Install [Criterion](https://github.com/bheisler/criterion.rs) as dependency

Read about [Cargo Modules](https://betterprogramming.pub/explaining-rusts-modules-420d38eed6c5)

Read about [cfg marcos](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html)

For a single benchmark file:

```
cargo bench
```

For multiple benchmark files:

```
cargo bench --bench [benchmark name]
```

## Generating Documentation Locally

### Use `rustdoc` to render docs for a single source file

```
rustdoc file.rs
```

This creates a `doc/` directory with subdirectory containing index.html.

### Use `cargo` to render docs for a crate and its dependencies

Generating HTML documentation:

```
cargo doc
```

This crates index.html in `/target/doc/`. To open doc in web browser:

```
cargo doc --open
```

To inspect the output directory:

```
tree -d -L 1 target/doc/
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
- <a href="https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm">WebAssembly</a>
- <a href="https://github.com/rust-lang/rustlings">Rustlings - Small exercises</a>
- <a href="https://doc.rust-lang.org/edition-guide/introduction.html">The Edition Guide</a>

[Memory Safety Explained](memory/README.md)

To improve the throughput of a web server:

- Thread Pool
- Fork/Join model
- Single-threaded async I/O model

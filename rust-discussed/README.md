# rust-discussed

A place for everything covered in the Rust Club sessions.

## Introduction to Rust

Rust was built with safety, speed, and efficiency in mind. Some benefits of using Rust:

- Compile time memory checks: Rust ensures memory safety through ownership and borrowing without garbage collection
- Catching bugs at compile time with a user-friendly compiler
- Low-level control
- Zero-cost abstractions
- Easier concurrency
- Good documentation
- Integrated package manager

Rust has been rated as Stack Overflow's most-loved programming language since 2016, and is being used increasingly by tech companies including Google, Microsoft and Facebook.

## Installing Rust

On macOS, run the following command in the terminal:

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

You may need to enter your password.

This will install Rust via `rustup`, a command line tool that is used to manage Rust and its associated tools.

More detailed installation instructions can be found here: https://doc.rust-lang.org/stable/book/ch01-01-installation.html

The rust-analyzer extension for Visual Studio Code provides good support for Rust.

## Running Files in Rust

Rust uses the file extension .rs. To compile a file, we run `rustc [filename].rs`, and to run the resulting binary, we use `./[filename]`.

As an example, to run the file `hello_world.rs`, we would compile with `rustc hello_world.rs` and run using `./hello_world`. By convention, we use underscores and lowercase for filenames in Rust.

## Using Cargo to Build Projects

Cargo is a useful tool that handles various tasks like building your code and downloading the libraries that your code depends on. To initialise Cargo in a new folder as a Git repository, run `cargo new [folder-name]`. You can add the flag `--vcs=none` (as in `cargo new --vcs=none [folder-name]`) to avoid creating the Git repository. Alternatively, to initialise Cargo in an existing folder, use `cargo init`.

This creates a project with a `Cargo.toml` file which contains metadata for the project and keeps track of the dependencies used. It will also create a `main.rs` file in the `src` folder, which is the generally the entry point of a program.

A crate in Rust is a compilation unit. Note that a crate can be compiled into a binary or into a library. Libraries have no entry point, but instead provide functionality for other crates to use. To create a library, add the `--lib` flag to `cargo new`.

To build the project, use `cargo build`. Cargo stores the result of the build in the `./target/debug directory`. If you want to compile and run the project, use `cargo run`.

Additionally, you can use `cargo build --release` to build for production, `cargo check` to check that the code compiles without building, and `cargo doc --open` to see generated documentation.

You can find more information on Cargo here: https://doc.rust-lang.org/stable/cargo/

## Resources

- <a href="https://doc.rust-lang.org/book/">The Rust Programming Language</a>: an introductory book about Rust
- <a href="https://doc.rust-lang.org/stable/rust-by-example/">Rust By Example</a>: examples illustrating various Rust concepts
- <a href="https://doc.rust-lang.org/stable/cargo/">The Cargo Book</a>: learn more about Cargo
- <a href="https://github.com/rust-lang/rustlings">Rustlings</a>: small exercises for getting used to reading and writing Rust code
- <a href="https://code.visualstudio.com/docs/languages/rust">Rust in Visual Studio Code</a>
- <a href="https://exercism.org/tracks/rust"> Rust Exercism</a>
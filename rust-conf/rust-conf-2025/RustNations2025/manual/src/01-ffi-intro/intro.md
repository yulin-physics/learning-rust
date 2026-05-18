# Getting Started with FFI

FFI stands for "Foreign Function Interface". While that sounds rather alarming, it just means "stuff that wasn't written in Rust".

The "C ABI" --- Application Binary Interface, we're playing acronym soup --- is the lingua franca of modern operating systems. It's sometimes the only well-defined binary interface for calling functions and exchanging data. Unfortunately, you can't just drop a Rust library straight into a C++, Go, C#, etc. project and expect it to do much. The languages are different, and need a common language in order to communicate.

FFI is that commonality.

Rust is great at FFI, it was one of the original design goals. There's no performance penalty (like CGo, C# Marshaling, etc.), but you do lose some of the richness of the type system.

## How to Avoid Doing FFI

> If your goal is not do FFI, this may have been the wrong workshop!

You could:
* Wrap up the code you need in another language in an executable and run it with `std::process::Command`.
    * Now every call requires that you setup its inputs.
    * Every call requires that the OS load the program, allocate it, etc.
    * Now you have to read the input.
    * And worst of all - you aren't writing Rust!
* You could put it on a microservice
    * Now you pay for a network call every time you need it.
    * You *still* have to wrap the input/output, and
    * You're *still* not writing Rust.

## Safety

You're going to type `unsafe` a lot more than you're (hopefully) used to! There's even been proposals for a "safe unsafe" flag!

So... let's write some horribly unsafe code!

![](../images/crab-leroy.webp)
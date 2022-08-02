## Stack and Heap

CPU has a queue of programs to be run and hands them to the OS.The OS allocates each program a fixed RAM, including the stack. If the program needs more memory at runtime, the OS goes to the heap (reserved memory on RAM for dynamic use) and allocates the memory. Memory addresses in a program are virtual, and are mapped to physical addresses in memory by the OS.

Compare `String` and `&str` in Rust:

`String`

- Stored in the heap, takes up more bytes than the string itself because it is the String object
- Reassigning variable wipes data on heap, and keep the same memory address
- Heap exists on the RAM.

`&str`

- Stored on the stack, takes up exactly the bytes of the string
- Reassigning variable gets a new memory address, because stack size is fixed and cannot be changed
- Stack is always in RAM. There is a stack pointer that is kept in a register in CPU that points to the top of stack, i.e., the address of the location at the top of stack.

Heap and stack grow towards one another as they get larger. Hence, if you use too much stack memory, the stack will actually collide with the address space of the heap, resulting in a **stack overflow**. In reality, stack and heap may not exist near one another on physical RAM.

## Memory Safety

RWLock pattern & Iterator invalidation.

The simplest way to prevent mutating shared state at compile time is to either disallow sharing or disallow mutating.

Actor-based languages generally solve the problem by disallowing sharing. (You have to send things back and forth across channels, so no two actors can have a reference to the same memory at the same time.)

Functional languages generally solve the problem by disallowing mutation. ("Modifying" a variable is actually creating a new one with the same programmer-visible name, but the compiler omits that if it would be equivalent.)

Rust solves the problem by disallowing your choice of sharing or mutation. (The language only allows multiple references to the same memory if none of them are mutable.)

Rust does not allow borrowing a reference and a mut reference at the same time, or mutating data while iterating over the data (invariants). Rust has both references (`&T` & `&mut T`) and raw pointers (`*const T` & `*mut T`). References must uphold certain invariants, but that means you can use them safely. Raw pointers don't have such guarantees themself, which means using them requires unsafe. There is also std::ptr module, which contains functions that operate on raw pointers.

### References

<a href="https://www.reddit.com/r/rust/comments/7d9pkg/why_does_rust_not_allow_borrow_references_and_a/">Why does Rust not allow borrow references and a mut reference at the same time?</a>

<a href="https://www.reddit.com/r/rust/comments/px2hp1/what_is_the_difference_between_a_borrow_and_a/">What is the difference between a borrow and a pointer?</a>

<a href="https://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/">The Problem With Single-threaded Shared Mutability</a>

<a href="https://peterlyons.com/problog/2017/12/rust-converting-bytes-chars-and-strings/">Rust converting bytes chars and strings</a>

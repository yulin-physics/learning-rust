# Rust in Action

Primitive types in Rust implement Copy trait, whereas all other types have move semantics.

## Lifetime Annotations

Lifetimes provide specific information to the Rust compiler about data that lives outside a function (functions that use references). Make sure input data lives as long as the function.

Lifetime elision - lifetime parameters don't need to be provided usually.

## Ownership

Two main ways to shift ownership

- through assignment
- through a function barrier

Resolve ownership issues:

- Use references where full ownership is not required
- Duplicate the value by Copy and Clone
- Refactoring code to reduce number of long-lived objects
- Wrap data in a type deisgned to assist with movement issues `Rc<T>` and `Arc<T>`

Reference counted pointers enable shared ownership of data. `Rc<T>` implements Clone. Every call to clone() increments an internal counter, every Drop decrements the counter. When the internal counter reaches zero, the original instance is freed.

`Rc<T>` does not allow mutation, need `Rc<RefCell<T>>` to perform interior mutability. An object with interior mutability presents an immutable facade, while internal values are being modified.

`Rc<T>` is not threadsafe. Replace `Rc<T>` with `Arc<T>` for atomic reference counter and `Rc<RefCell<T>>` with `Arc<Mutex<T>>`.

## Generics

Handle many possible input types to functions, define in function signature by capital letters.

## Trait bounds

Add constraints to generics. A trait is analogous to an interface or protocol in other domains.

## Newtype Pattern

Create alias using the type keyword, the alias will have all methods form the type:

```
type File = String
```

Create distinct types by using newtype pattern, consists of wrapping a core type within a single field struct or perhaps a tuple. Compiler will treat the types differently:

```
struct File(String)
```

## Error Handling

Modifying a known global varibale, C programmers are used to checking the value of **errno** once system calls have been returned.

```
static mut ERROR
```

Need to wrap static lifetime global vaiables in `unsafe` block when accessing.

Alternatively, make use of Rust `Result` return type. Let functions take ownership of the argument and return in Ok(T).

## Integer Overflow

Text files are just binary files that happen to follow a consistent mapping between bit strings and characters. This mapping is called an encoding.

Integer Overflow: integers are fixed size
Endianness: CPU specific, reading byte sequences from left-to-right or right-to-left. Computer's prefernce for layout of individual bits is known as its bit numbering or bit endianness. MSB can refer to bother most significant bit (bit numbering) and most significant byte (endianness).

## Floating point

Each floating point number is laid out in memory as scientific notation. A floating point value is a container with three fields:

- a sign bit
- an exponent
- a mantissa, or significand

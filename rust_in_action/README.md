# Rust in Action

## Lifetime Annotations

Lifetimes provide specific information to the Rust compiler about data that lives outside a function (functions that use references). Make sure input data lives as long as the function.

Lifetime elision - lifetime parameters don't need to be provided usually.

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

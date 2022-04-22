# String and &str

Creating text using string literals is by default, of type string slice &str (reference type).
String is capable of resizing, and appending more text.

Using string slices is more efficient, because it is a reference and no copy is made when passing the variable. However,if ownership of the string is wanted, use String (e.g. a string field in a struct)

`String` can be easily turned into `&str` by deref coercion (place `&` in front of a `String`).

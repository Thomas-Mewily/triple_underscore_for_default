A *shorter* way to write default. Provide :

- `___()` as a shorthand for `Default::default()`
-  `i32::___()` instead of `i32::default()`

Based on the [internals Rust discussion](https://internals.rust-lang.org/t/could-we-have-std-default/8756)

Also check the [Defaults crate](https://github.com/dpc/rust-default) which use `default()` instead of `Default::default()` 

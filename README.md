A *shorter* way to write default. Provide :

- `___()` as a shorthand for `Default::default()`
-  `i32::___()` instead of `i32::default()`

Based on the [internals Rust discussion](https://internals.rust-lang.org/t/could-we-have-std-default/8756)

Also check the [Defaults crate](https://github.com/dpc/rust-default) which use `default()` instead of `Default::default()` 

# Examples

```rust
let b : i32 = Default::default(); // Default Rust
let a : i32 = ___(); // Now
assert_eq!(a, b);
```

Can also be used with function :

```rust
let a = f(Default::default()); // Default Rust
let b = f(___()); // Now
assert_eq!(a, b);
```

Can also be used to initialize complex Rust struct when implementing the `Default` trait :

```rust
impl Default for ComplexeStruct {
    fn default() -> Self { 
    Self { a : ___(), b : ___(), c : ___(), vec : vec![0] }
    //  instead of
    // Self { a : Default::default(), b : Default::default(), c : Default::default(), vec : vec![0] }
    }
}
```

Uniform syntax : `MyStruct::___()` instead of `MyStruct::default()` :

```rust
type T = i32; // any type with default
assert_eq!(T::default(), T::___());
```
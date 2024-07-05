//! A *shorter* way to write default. Provide :
//! 
//! - `___()` as a shorthand for `Default::default()`
//! -  `i32::___()` instead of `i32::default()`
//! 
//! Based on the [internals Rust discussion](https://internals.rust-lang.org/t/could-we-have-std-default/8756)
//! 
//! Also check the [Defaults crate](https://github.com/dpc/rust-default) which use `default()` instead of `Default::default()`

/// `___()` is a shorthand for `Default::default()`
///
/// # Examples
/// 
/// ```
/// let b : i32 = Default::default(); // Default Rust
/// let a : i32 = ___(); // Now

/// assert_eq!(a, b);
/// ```
/// 
/// Can also be used with function :
/// 
/// ```
/// let a = f(Default::default()); // Default Rust
/// let b = f(___()); // Now
/// assert_eq!(a, b);
/// ```
/// 
/// Can also be used to initialize complexe Rust struct when implementing the `Default` trait :
/// 
/// ```
/// impl Default for ComplexeStruct {
///     fn default() -> Self { 
///     Self { a : ___(), b : ___(), c : ___(), vec : vec![0] }
///         //  instead of
///         // Self { a : Default::default(), b : Default::default(), c : Default::default(), vec : vec![0] }
///     }
/// }
/// ````
/// 
/// And also to partially initialize a struct
/// 
/// ```
/// let a = BigStruct { x : 42, y : 64, ..Default::default() };
/// let b = BigStruct { x : 42, y : 64, ..___() };
/// assert_eq!(a, b);
/// ```
pub fn ___<T:Default>() -> T {
    std::default::Default::default()
}

/// Uniform syntax : `MyStruct::___()` instead of `MyStruct::default()`
pub trait DefaultIsTripleUnderscore : Default
{
    /// The `Self::default()` method.
    /// 
    /// Returns the "default value" for a type.
    /// 
    /// ```
    /// type T = i32; // any type with default
    /// assert_eq!(T::default(), T::___());
    /// ```
    fn ___() -> Self { Self::default() }
}
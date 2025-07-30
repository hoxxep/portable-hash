//! Implementations of `PortableOrd` for various types.
//!
//! There is an implicit assumption that the standard library types will not break the order of
//! some collections, such as `slice` or `Vec`. We assume these will always be element-wise, and
//! will never change to length-first sorting for example.

#[cfg(feature = "alloc")]
mod alloc_lib;
mod core_lib;
mod primitives;
#[cfg(feature = "std")]
mod std_lib;

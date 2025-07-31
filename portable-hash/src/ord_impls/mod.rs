//! Implementations of `PortableOrd` for various types.
//!
//! There is an implicit assumption that the standard library types will not break the order of
//! some collections, such as `slice` or `Vec`. We assume these will always be element-wise, and
//! will never change to length-first sorting for example.

#[cfg(feature = "alloc")]
mod ord_alloc;
mod ord_core;
mod ord_primitives;
#[cfg(feature = "std")]
mod ord_std;

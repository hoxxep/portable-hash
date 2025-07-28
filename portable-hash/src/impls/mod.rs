//! Implementations of `PortableHash` for common types.
#[cfg(feature = "alloc")]
mod alloc_lib;
mod core_lib;
mod primitives;
#[cfg(feature = "std")]
mod std_lib;

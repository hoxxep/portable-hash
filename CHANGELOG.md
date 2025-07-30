This changelog applies to the `portable-hash` crate. Any changes to the `portable-hash-macros` crate will be rolled into this changelog, as the crate is not intended to be used independently.

# v0.4.0

- **Breaking:** Change default `write_str` behaviour to use a `write_u8(0xFF)` suffix instead of a length prefix [to match std](https://github.com/rust-lang/rust/pull/134134).
- **Breaking:** Removed the `PortableHash` implementation for `!`.
- **New:** Added `write_short(bytes: [u8; LEN])` to `PortableHasher` for hashing short fixed-length byte arrays.
- Reduced MSRV to 1.57, and used `rustversion` to gate certain `PortableHash` implementations based on compiler version.

# v0.3.0

- **Breaking:** Fixed unpadded integer slice hashing portability by removing the old transmute approach.
- **New:** Added the `PortableOrd` marker trait for hashable types. This is used when hashing collections that require stable ordering to hash portably, such as BTrees.
- Improved documentation for most types and methods.
- Bump portable-hash-macros to 0.1.1 to reduce the MSRV to 1.57.

# v0.2.0

- **Breaking:** Changed the implementation of some `PortableHash` implementations.
- **Breaking:** Moved `write_len_prefix` calling out of `impl<T> PortableHash for [T]` and into the `PortableHash::hash_slice` method. The impl calls `hash_slice`, which enables us to override the `write_len_prefix` calling on a per-type basis, enabling `write_bytes` to do so on a per-hasher basis.
- **Breaking:** Changed primitive integer types to call `write_bytes` when hashing slices.
- **New:** Added a `PortableHash` derive macro.
- **New:** Added many `PortableHash` implementations for std, alloc, core, and primitive types.

# v0.1.0

- **New:** A very basic draft of the `PortableHash` and `PortableHasher` traits.

This changelog applies to the `portable-hash` crate. Any changes to the `portable-hash-macros` crate will be rolled into this changelog, as the crate is not intended to be used independently.

# v0.3.0

- **Breaking:** Fixed unpadded integer slice hashing portability by removing the old transmute approach.
- **New:** Added the `PortableOrd` marker trait for hashable types. This is used when hashing collections that require stable ordering to hash portably, such as BTrees.
- **New:** Improved documentation for most types and methods.

# v0.2.0

- **Breaking:** Changed the implementation of some `PortableHash` implementations.
- **Breaking:** Moved `write_len_prefix` calling out of `impl<T> PortableHash for [T]` and into the `PortableHash::hash_slice` method. The impl calls `hash_slice`, which enables us to override the `write_len_prefix` calling on a per-type basis, enabling `write_bytes` to do so on a per-hasher basis.
- **Breaking:** Changed primitive integer types to call `write_bytes` when hashing slices.
- **New:** Added a `PortableHash` derive macro.
- **New:** Added many `PortableHash` implementations for std, alloc, core, and primitive types.

# v0.1.0

- **New:** A very basic draft of the `PortableHash` and `PortableHasher` traits.

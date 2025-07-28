# Sha Hasher

Sha hashing library that implements `PortableHasher`. This allows easy portable hashing of rust types using `derive(PortableHash)` or by implementing `PortableHash` on your types.

This crate is currently written as an example `PortableHasher` implementation for tests, and discussing whether cryptographic hashers could be implemented through the same `PortableHasher` trait.

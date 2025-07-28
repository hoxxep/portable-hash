# Sha Hasher

Sha hashing library for [portable-hash](https://github.com/hoxxep/portable-hash).

`Sha256Hasher` implements a SHA-256 `PortableHasher`. This allows easy portable hashing of arbitrary rust types using `derive(PortableHash)`, or by manually implementing `PortableHash` on your types.

This crate is currently written as an example `PortableHasher` implementation for tests, and discussing whether cryptographic hashers could be implemented through the same `PortableHasher` trait.

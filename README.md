# Portable Hashing Traits for Rust

Disclaimer: not ready for production, yet!

Introducing `PortableHash` and `PortableHasher`: a set of traits to make portable hashing easy and stable across different platforms and versions of Rust.

The standard library `Hash` and `Hasher` traits are not suitable for portable hashing across different platforms and versions of Rust. The hashing of primitive types, standard library types, implementation of `derive(Hash)`, and the default behaviour of `Hasher` methods may all change between platforms and compiler versions. This crate is intended to provide an equally easy to use alternative that is truly stable and portable across platforms and versions of Rust.

Do not use this crate in production yet as it's still under development. Please wait for the 1.0 release to stabilise the API and hash output. The `PortableHash` and `PortableHasher` traits deviate from the standard library in various ways that still need to be reviewed and documented, and are subject to change. Subscribe to notifications on the [stabilisation issue](https://github.com/hoxxep/portable-hash/issues/1) to be notified of the 1.0 release. Issues and contributions are very welcome.

## TODO
- [ ] Documentation for the APIs.
- [ ] Documentation for how to implement portable hashing correctly.
- [ ] Create a `derive(PortableHash)` macro.
- [ ] Decide on, and/or fully implement, `write_bytes`
- [ ] Decide on digest/hasher-specific output types. Should the default `finish` instead offer a custom Output type? Use a better name than "digest".
- [ ] Decide on `!` implementation.
- [ ] Decide on ptr implementations.
- [ ] Decide on `write_len_prefix` name change.
- [ ] Decide on `write_str` default implementation change to a length prefix.
- [ ] Tests and example implementations, including rapidhash, Sha256, BLAKE3, and SipHasher.
- [ ] Final comment period.
- [ ] Stabilise with 1.0.

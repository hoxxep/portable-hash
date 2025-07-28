# Portable Hashing Traits for Rust

Disclaimer: not ready for production, yet!

Introducing `PortableHash` and `PortableHasher`: a set of traits for portable and stable hashing across different platforms and compiler versions. Stable, portable hashing made easy!

## Using PortableHash

To use `PortableHash`, simply derive or implement it on your types, and choose a `PortableHasher` implementation that suits your needs.

```rust
use portable_hash::PortableHash;
use rapidhash::v3::RapidBuildHasher;

#[derive(PortableHash, Default)]
struct MyType {
    field1: u32,
    field2: String,
}

let mut hasher = RapidHasher::default();
let object = MyType::default();
object.portable_hash(&mut hasher);
let hash = hasher.finish();
```

Hashers that implement `PortableHasher`:
- [rapidhash](https://crates.io/crates/rapidhash): A fast, non-cryptographic, minimally DoS resistant hasher.
- TBC: sha, blake, siphash, seahash etc. hashers.

## Implementing PortableHasher for Hash Library Authors

Implementing `PortableHasher` is very similar to implementing the standard library `Hasher` trait, with some additional requirements.

Your crate must provide the following guarantees when implementing `PortableHasher`:
- The hash output must be stable across all minor versions of your crate.
- Integer types are hashed consistently across all platforms, explicitly choosing little-endian or big-endian encoding.

## What's wrong with the `std::hash` traits?
The standard library `Hash` and `Hasher` traits are not suitable for portable hashing across different platforms and versions of Rust. The hashing of primitive types, standard library types, implementation of `derive(Hash)`, and the default behaviour of `Hasher` methods may all change between platforms and compiler versions. This crate is intended to provide an equally easy to use alternative that is truly stable and portable across platforms and versions of Rust.

<details>
<summary><strong>What is required to use <code>std::hash</code> traits in a stable and portable way?</strong></summary>

The default behaviour of hashing any primitive type, standard library type, and the default `Hash` and `Hasher` implementations are all subject to change between compiler versions.

`Hash` is responsible for breaking down a type into primitive types to feed a `Hasher`, while `Hasher` is responsible for consuming those bytes and producing a hash output.

A `Hasher` author must:
- Ensure that integers are hashed consistently on all platforms, always choosing little-endian or big-endian.
- Override the default `write_*` methods to ensure that compiler versions changing the default behaviour won't affect this `Hasher`'s output.
- Ensure stability of the hash output between minor crate versions.

And end users must:
- Ensure their chosen `Hasher` is portable, and promises to be stable between rust and crate versions.
- Explicitly not use `derive(Hash)` and implement `Hash::hash` on their hashed types manually using `Hasher::write_*` methods.
- Avoid using `Hash::hash` on types they haven't manually implemented, including primitive types like `str` and tuples.
- Avoid `Hasher::write_*` methods with default implementations (particularly the upcoming `write_str`), which requires reading the Hasher implementation source code to check.
- Avoid using `write_usize` and `write_isize` unless it is portably hashed across platforms by the `Hasher`.
- Iterate manually over any tuples and collections.
- Be informed on how to construct a hash to avoid reordering or length-extension attacks etc, if required for their use case.

This is so fraught with accidental footguns, `PortableHash` and `PortableHasher` have been provided to allow end users to simply `derive(PortableHash)` and choose any `PortableHasher` without worrying about the above pitfalls.

</details>

## Is portable-hash ready for production?
Do not use this crate in production yet as it's still under development. Please wait for the 1.0 release to stabilise the API and hash output. The `PortableHash` and `PortableHasher` traits deviate from the standard library in various ways that still need to be reviewed and documented, and are subject to change. Subscribe to notifications on the [stabilisation issue](https://github.com/hoxxep/portable-hash/issues/1) to be notified of the 1.0 release. Issues and contributions are very welcome.

## TODO
- [ ] Documentation for the APIs.
- [ ] Documentation for how to implement portable hashing correctly.
- [ ] Create a `derive(PortableHash)` macro.
- [ ] Match the ordering of the `Hasher` trait methods.
- [ ] Decide on, and/or fully implement, `write_bytes`
- [ ] Decide on removing `write_usize` and `write_isize` methods.
- [ ] Decide on digest/hasher-specific output types. Should the default `finish` instead offer a custom Output type? Use a better name than "digest".
- [ ] Decide on `!` implementation.
- [ ] Decide on ptr implementations.
- [ ] Decide on `write_len_prefix` name change.
- [ ] Decide on `write_str` default implementation change to a length prefix.
- [ ] Decide on renaming `BuildPortableHasher` to `PortableBuildHasher`?
- [ ] Tests and example implementations, including rapidhash, Sha256, BLAKE3, and SipHasher.
- [ ] Final comment period.
- [ ] Stabilise with 1.0.

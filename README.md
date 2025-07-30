# Portable Hashing Traits for Rust

Note: not ready for production, yet!

Introducing `PortableHash` and `PortableHasher`: a set of traits for portable and stable hashing across different platforms and compiler versions. Stable, portable hashing made easy! This crate does not offer a hasher implementation, but provides the traits and macros to link data types implementing `PortableHash` with hashers implementing `PortableHasher`.

**Sponsored by [Upon](https://uponvault.com?utm_source=github&utm_campaign=portable-hash)**, inheritance vaults for your digital life. Ensure your family can access your devices, accounts, and assets when the unexpected happens.

## Using `PortableHash` in libs and apps

To use `PortableHash`, simply `#[derive(PortableHash)]` or implement it manually on your types, and choose a `PortableHasher` implementation that suits your needs.

By implementing `PortableHash` on library types, **you promise to guarantee** that the type hashing logic is stable across:
- **All platforms.** Avoid hashing non-portable types such as `OsString`, `OsStr`, or `Path` have platform-specific encodings and representations.
- **All rust compiler versions.** Avoid mixing `std::hash::Hash` or other non-stable hashing traits to produce a `PortableHash` output.
- **All minor versions of your crate.** Fields in your types may be reordered, added, or changed, but the `PortableHash::portable_hash` must always hash the same fields in the same order for all crate minor versions.
  - Any breaking changes to the hash output of any type should require a major version bump of your crate, and documentation of the breaking change in your changelog.
  - Be careful with `#[derive(PortableHash)]`. Changing the order of fields in structs or enums will change the hash output. We recommend writing unit tests that hash each of your types against hardcoded hash outputs to check for stability. Fields can be _renamed_ safely, but cannot be re-ordered or change type. Please implement `PortableHash` manually to maintain stability if you need to change the order of fields.

<details>
<summary><strong>Examples of hashable, but not portable types.</strong></summary>

`OsString`, `OsStr`, and `Path` are examples of types that vary between platforms. The string encodings of these types can differ based on the operating system, making them unsuitable for portable hashing. They can safely derive `std::hash::Hash` for in-memory hashmaps, but `PortableHash` is explicitly _not_ implemented on these types.

</details>

```rust
use portable_hash::{PortableHash, PortableHasher, PortableHasherDigest};
use sha_hasher::Sha256Hasher;

#[derive(PortableHash, Default)]
struct MyType {
    a: u32,
    b: String,
}

let object = MyType { a: 42, b: "Hello".to_string() };

let mut hasher = Sha256Hasher::default();
object.portable_hash(&mut hasher);
assert_eq!(hasher.finish(), 5333351996764360352, "u64 output");
assert_eq!(hasher.digest(), [
    160, 142, 66, 61, 98, 223, 3, 74, 108, 15, 1, 253, 229, 169, 86, 215,
    117, 111, 201, 32, 16, 24, 16, 174, 206, 67, 25, 224, 226, 174, 4, 168
], "hasher-specific output type");
```

Hashers that implement `PortableHasher`:
- [sha-hasher](https://github.com/hoxxep/portable-hasher): The portable-hash test hasher, a stable SHA-256 hasher that implements `PortableHasher`.
- [rapidhash](https://crates.io/crates/rapidhash) (under development): A fast, non-cryptographic, portable, minimally DoS resistant hasher.
- TBC: sha, blake, siphash, seahash etc. hashers.

## Implementing `PortableHasher` for hash library authors

Implementing `PortableHasher` is very similar to implementing the standard library `Hasher` trait, with some additional requirements.

Your crate must provide the following guarantees when implementing `PortableHasher`:
- The hash output must be stable across all minor versions of your crate.
- Integer types are hashed consistently across all platforms, explicitly choosing little-endian or big-endian encoding.
- Use `default-features = false` when adding `portable-hash` as a dependency, so end users can opt to disable the `std` feature.

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

## TODO before stabilisation
- [x] Basic `PortableHash` and `PortableHasher` traits.
- [x] Implement `PortableHash` on many primitive and standard library types.
- [ ] Documentation for the APIs.
- [ ] Documentation for how to implement portable hashing correctly.
- [ ] Should the `PortableHasher` trait methods be renamed? Is there a risk of accidentally calling the wrong implementation of `write` if `Hasher` and `PortableHasher` are implemented at the same time? Should be a "no" when implementing `PortableHash::hash`.
- [x] Create a `derive(PortableHash)` macro.
- [ ] Review the `derive(PortableHash)` macro to produce stable enum hashing. Re-ordering currently changes the hash output, while renaming is safe.
- [x] Create a `PortableOrd` marker trait for collections that require stable ordering to hash portably, such as BTrees.
- [x] Compare to [anyhash](https://crates.io/crates/anyhash). It does not promise stability or hash types in a DoS-resistant way, but will follow the `HasherWrite` and rustc's `ExtendedHasher` trait idea for custom hasher outputs.
- [x] Match the ordering of the `Hasher` trait methods.
- [x] Decide on, and/or fully implement, `write_bytes`.
- [x] ~Decide on removing `write_usize` and `write_isize` methods.~ These types are not portable by default, we will expect them to always be `write_u64`, but it seems more flexible to leave that decision to the `PortableHasher` implementation.
- [ ] Decide on digest/hasher-specific output types.
  - [ ] Should the default `finish` instead offer a custom Output type?
  - [ ] Use a better name for custom outputs than "digest".
  - [ ] Should cryptographic hashes implement `PortableHasher`? Is the `sha-hasher` a reasonable thing to publish?
- [x] Decide on `!` implementation. Remove the nightly feature, `!` can be implemented if required by a user.
- [x] Decide on ptr implementations, or remove hashing pointers. Removed as non portable.
- [x] Decide on `write_len_prefix` name change (differs from `write_length_prefix` in the std library). Stick with `write_len_prefix` as the FCP might change it, we can also deprecate it later if necessary.
- [ ] Decide on `write_str` default implementation change to use a length prefix.
- [ ] Decide on renaming `BuildPortableHasher` to `PortableBuildHasher`?
- [ ] Decide on `std` being a default feature or not.
  - [x] Document that Hasher libraries should use `default-features = false` so users can choose what to include.
- [ ] Review many of the primitive and enum `PortableHash` implementations for stability and DoS resistance, double-check the manual `write_u8` enum discriminant keys.
- [ ] Tests and example implementations, including rapidhash, Sha256, BLAKE3, and SipHasher.
- [ ] Final comment period.
- [ ] Stabilise with 1.0.

## Versioning

1.0 will mark the first stable release of portable-hash. Before then, consider this trait unstable.

Major version bumps will occur for:
- Breaking API changes.
- Hash output changes in any way:
  - Changes to the `PortableHash` implementation of a basic type.
  - Changes to the default behaviour of a `PortableHasher` method.

Minor version bumps will occur for:
- API additions.
- New `PortableHash` implementations.

Users must be able to fix to a specific major version of `portable-hash`. Any library with a `portable-hash` dependency should make a major version bump of their crate if they change the major version of `portable-hash`, unless their trait offers support for multiple versions of `PortableHash`.

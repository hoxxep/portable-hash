# PortableHasher Testing

Automating **stability testing** of `PortableHasher` and `PortableHash` implementations.

This crate provides a test suite and thousands of test fixtures for standard types. The fixtures are hashed by the chosen `PortableHasher` implementation, the has outputs are serialized to a `fixtures.csv` file, and subsequent runs will compare the outputs against the saved results.

Users are able to add their own fixtures of `PortableHash` types, so this can be used to test the stability and portability of any combination of `PortableHasher` and `PortableHash` types.

## Example usage
```rust
use portable_hash::PortableHash;
use portable_hash_tester::{test_default_fixtures, FixtureDB};

/// Your custom type that implements `PortableHash`.
#[derive(PortableHash, Debug)]
struct MyType {
    a: u32,
}

/// Test your custom `PortableHasher` implementation and `PortableHash` types are
/// stable and portable between platforms, compiler, and crate versions.
#[test]
fn test_my_hasher() {
    // Any BuildPortableHasher implementation you choose.
    let hasher = BuildCustomHasher::default();

    // Load/store the expected hash outputs from a file in your git repository.
    let mut fixtures = FixtureDB::load(hasher, "path/to/fixtures.csv");

    // Run thousands of test suite fixtures on standard types.
    test_default_fixtures(&mut fixtures);

    // Test your own PortableHash types.
    fixtures.test_fixture("test_name", MyType { a: 42 });

    // Test your own PortableHash types that don't implement debug.
    fixtures.test_fixture_no_debug("test_name", MyType { a: 42 });

    // Log the summary stats and error if any hash outputs changed.
    fixtures.finish();
}
```

## TODO

- [ ] Finish adding all tests.
- [ ] Support or compare to `Hasher`?
- [ ] Add slice tests for all types, due to `hash_slice` being overridable on a per-type basis.
- [ ] Consider building our own stable Arbitrary trait to make generating fixtures easier?
- [ ] Decide on versioning. If new tests are breaking, should this crate perform a major version bump when new tests are released?
- [ ] Improved documentation.
- [ ] Rename `FixtureDB` to something more appropriate.

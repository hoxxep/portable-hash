# PortableHasher Testing

A trait to automate **stability testing** of `PortableHasher` implementations. This trait will automatically generate a whole suite of fixtures against the provided `PortableHasher` implementation and ensure the hash output remains stable between platform, compiler, and crate versions.

This is designed for both `PortableHasher` authors and for users who want to test the stability of their own `PortableHash` types.

## Design

A `BuildPortableHasher` object and a fixture file path get passed to the framework for testing. The user is responsible for running the tests over different seeds.

The framework then:
- Parses the existing fixture file for expected results.
- Runs the hasher implementation against every test, comparing to the previously saved results.
  - New tests will be added to the fixture file.
  - If the results differ, the test will fail.
- Generates new fixtures if none exist, or when some kind of flag is passed (saving the old as .old).

## TODO

- [ ] Finish adding all tests.
- [ ] Support or compare to `Hasher`?
- [ ] Add slice tests for all types, due to `hash_slice` being overridable on a per-type basis.
- [ ] Consider using Arbitrary for generating random types.
- [ ] Decide on versioning. If new tests are breaking, should this crate perform a major version bump when new tests are released?
- [ ] Improved documentation.
- [ ] Rename `FixtureDB` to something more appropriate.

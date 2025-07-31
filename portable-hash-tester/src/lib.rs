#![doc = include_str!("../README.md")]
// #![cfg_attr(not(feature = "std"), no_std)]

#![warn(missing_docs)]
#![deny(unused_must_use)]

mod tests;
mod fixture;
mod rng;

use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::thread;
use portable_hash::{BuildPortableHasher, PortableHash};
use crate::fixture::{Fixture, FixtureState};

pub use rng::*;

/// Instantiate the portable-hash-tester crate against a specific [`BuildPortableHasher`]
/// implementation and fixture file path. This will run the default test suite.
///
/// Please see [`FixtureDB`] for how to also run your own fixtures against the provided hasher.
///
/// # Example
/// ```ignore
/// use portable_hash_tester::test_portable_hasher;
///
/// #[test]
/// fn test_my_hasher() {
///    test_portable_hasher(CustomHasher::default(), "path/to/fixtures.csv");
/// }
/// ```
pub fn test_portable_hasher(
    hasher: impl BuildPortableHasher,
    fixture: impl AsRef<Path>,
) {
    let mut fixtures = FixtureDB::load(hasher, fixture);
    // fixtures.update_fixtures(true);
    test_default_fixtures(&mut fixtures);
    fixtures.finish();
}

/// Run the default fixtures against the provided hasher.
pub fn test_default_fixtures(fixtures: &mut FixtureDB<impl BuildPortableHasher>) {
    tests::test_primitives::test_primitives(fixtures);
    tests::test_core::test_core(fixtures);
}

/// Our custom test and fixture harness.
///
/// This tracks the state of each fixture, both expected and actual hashes, and is responsible
/// for logging and failing the test.
///
/// TODO(stabilisation): also test custom output types.
///
/// TODO(stabilisation): rename this to `TestHarness` or something more appropriate?
///
/// TODO: equivalent test against Hasher and BuildHasher traits too, for users porting from std.
///   Some hashes will likely be different from std... Check enum discriminants, etc.
///
/// # Example
/// ```ignore
/// use portable_hash_tester::{test_default_fixtures, test_fixture, FixtureDB};
///
/// /// Test your custom `PortableHasher` implementation or custom `PortableHash` types are
/// /// stable and portable between platforms, compiler, and crate versions.
/// #[test]
/// fn test_my_hasher() {
///     // Load the fixture database from a file in your git repository.
///     // NOTE: the hasher must use a constant seed for the tests to be stable.
///     let mut fixtures = FixtureDB::load(CustomHasher::default(), "path/to/fixtures.csv");
///
///     // Run the default fixtures and a custom fixture
///     test_default_fixtures(&mut fixtures);
///
///     // Or test your own PortableHash types this way
///     fixtures.test_fixture("test_name", "your custom object to be hashed");
///
///     // And test your own PortableHash types without the `Debug` trait, if necessary.
///     fixtures.test_fixture_no_debug("test_other", "your custom object to be hashed");
///
///     // Log the summary stats and check all tests passed.
///     fixtures.finish();
/// }
/// ```
#[derive(Default)]
pub struct FixtureDB<H: BuildPortableHasher> {
    hasher: H,
    path: PathBuf,
    fixtures: HashMap<String, Fixture>,
    updating: bool,
    finished: bool,
}

impl<H: BuildPortableHasher> FixtureDB<H> {
    /// Load the fixture database from a specified file path and instantiate the test harness.
    pub fn load(hasher: H, path: impl AsRef<Path>) -> Self {
        // Load fixtures from the specified path
        // Parse the file and populate the `fixtures` map
        // Return an error if loading fails

        let fixtures = load_fixture_file(path.as_ref());

        Self {
            hasher,
            path: path.as_ref().to_path_buf(),
            fixtures,
            updating: false,
            finished: false,
        }
    }

    /// Update the persisted fixture file with new fixtures.
    ///
    /// The default behaviour is to _not_ update the fixtures file.
    ///
    /// The fixtures database will be updated automatically if either:
    /// - A `--update-fixtures` flag is passed to the test runner.
    /// - A `UPDATE_HASHER_FIXTURES` environment variable is set to `true`.
    ///
    /// This method overrides the above defaults, and hard-codes the behaviour to always be true or
    /// false for a specific test run.
    pub fn update_fixtures(&mut self, updating: bool) {
        self.updating = updating;
    }

    /// Log the hash output for a fixture.
    fn log_hash_result(&mut self, name: &str, actual_hash: u64) -> &Fixture {
        assert!(!self.finished, "You are testing fixtures after `FixtureDB::finish()` has been called.");

        let fixture = self.fixtures
            .entry(name.to_string())
            .or_insert_with(|| Fixture::new(name.to_string()));

        fixture.log_hash_result(actual_hash);
        fixture
    }

    /// Test a fixture against the provided hasher.
    ///
    /// Test names must be unique, and will panic if a duplicate name is used.
    pub fn test_fixture<O: PortableHash + Debug>(&mut self, test_name: impl AsRef<str>, object: O) {
        let test_name = test_name.as_ref();

        // hash the object using the provided hasher
        let object_type = std::any::type_name::<O>();
        let object_fmt = format!("{:?}", object);
        let actual_hash = self.hasher.hash_one(object);

        // store the hash in the fixture database
        let fixture = self.log_hash_result(test_name, actual_hash);

        // logging for the fixture state
        match fixture.state() {
            FixtureState::New => {
                println!(
                    "new:  '{}' hash (actual) 0x{:016x} for {} object: {}",
                    test_name, actual_hash, object_type, object_fmt
                );
            }
            FixtureState::Failed => {
                let expected_hash = fixture.expected_hash
                    .map_or_else(
                        || "None".to_string(),
                        |i| format!("0x{:016x}", i),
                    );
                println!(
                    "fail: '{}' hash (expected) 0x{:016x} != {} (actual) for {} object: {}",
                    test_name, actual_hash, expected_hash, object_type, object_fmt
                );
            }
            FixtureState::Skipped => {
                let expected_hash = fixture.expected_hash
                    .map_or_else(
                        || "None".to_string(),
                        |i| format!("0x{:016x}", i),
                    );
                println!(
                    "skip: '{}' hash (expected) {} for {} object: {}",
                    test_name, expected_hash, object_type, object_fmt
                );
            }
            _ => {},  // we don't care about Passed or Skipped states in this context
        }
    }

    /// Test a fixture that doesn't implement `Debug`.
    ///
    /// Equivalent to [`test_fixture`] but without the `Debug` formatting of the object.
    pub fn test_fixture_no_debug<O: PortableHash>(&mut self, test_name: impl AsRef<str>, object: O) {
        let test_name = test_name.as_ref();

        // hash the object using the provided hasher
        let object_type = std::any::type_name::<O>();
        let actual_hash = self.hasher.hash_one(object);

        // store the hash in the fixture database
        let fixture = self.log_hash_result(test_name, actual_hash);

        // logging for the fixture state
        match fixture.state() {
            FixtureState::New => {
                println!("new:  '{}' hash: {} for {}", test_name, actual_hash, object_type);
            }
            FixtureState::Failed => {
                let expected_hash = fixture.expected_hash.map(|i| i.to_string()).unwrap_or_else(|| "None".to_string());
                println!("fail: '{}' hash (expected) {} != {} (actual) for {}", test_name, actual_hash, expected_hash, object_type);
            }
            _ => {},  // we don't care about Passed or Skipped states in this context
        }
    }

    /// Returns a summary of the fixture states.
    ///
    /// True the test should fail to alert the user, with a summary message for all fixture states.
    ///
    /// ```txt
    /// 120 passed, 3 new, 1 failed, 2 skipped for fixtures 'path/to/fixtures.csv'
    /// ```
    ///
    /// The summary will fail the test if:
    /// - There are any fixtures that are `Failed`.
    /// - There are any fixtures that are `New`, which should be added to the fixture file.
    ///   - TODO(stabilisation): review this decision, as adding new fixtures in releases will break everyone's tests... CI option?
    pub fn finish(&mut self) {
        let mut new = 0;
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;

        for fixture in self.fixtures.values() {
            match fixture.state() {
                FixtureState::New => new += 1,
                FixtureState::Passed => passed += 1,
                FixtureState::Failed => failed += 1,
                FixtureState::Skipped => skipped += 1,
            }
        }

        let msg = format!(
            "{} passed, {} new, {} failed, {} skipped for fixtures '{}'",
            new, passed, failed, skipped, self.path.display()
        );
        let fail_test = !(failed == 0 && new == 0);

        println!("\n{}", msg);

        // if we're updating, write the fixture database to the specified file path
        if self.updating {
            save_fixture_file(self.path.as_path(), &self.fixtures);
        }

        // fail if any value does not match the expected hash
        self.finished = true;
        if fail_test {
            panic!("Some fixtures failed. See the output above for details.");
        }
    }
}

impl<H: BuildPortableHasher> Drop for FixtureDB<H> {
    #[inline]
    fn drop(&mut self) {
        if !thread::panicking() {
            assert!(self.finished, "You failed to reach `FixtureDB::finish()` at the end of your test.");
        }
    }
}

fn load_fixture_file(path: impl AsRef<Path>) -> HashMap<String, Fixture> {
    let path = path.as_ref();
    if !path.exists() {
        println!("Starting from scratch, fixture file not found at: {}", path.display());
        return HashMap::new();
    }

    // read the CSV file and parse the fixtures
    let mut fixtures = HashMap::new();
    let file = std::fs::File::open(path).expect("Failed to open fixture file");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in reader.records() {
        let record = result.expect("Failed to read record from fixture file");

        if record.len() < 2 {
            continue; // skip invalid records
        }

        let name = record.get(0).expect("Expected fixture name").to_string();
        let expected_hash = record.get(1).and_then(|s| {
            // we print u64 hashes as big-endian hex strings
            let mut bytes = [0u8; 8];
            hex::decode_to_slice(s, &mut bytes).ok()?;
            Some(u64::from_be_bytes(bytes))
        });

        let fixture = Fixture {
            name,
            expected_hash,
            actual_hash: None,
        };

        fixtures.insert(fixture.name.clone(), fixture);
    }

    fixtures
}

fn save_fixture_file(path: impl AsRef<Path>, fixtures: &HashMap<String, Fixture>) {
    let path = path.as_ref();

    // rename the old file to .old as a backup
    if path.exists() {
        let backup_path = path.with_extension(".csv.old");
        if backup_path.exists() {
            std::fs::remove_file(&backup_path).expect("Failed to remove old fixture file");
        }
        std::fs::rename(path, backup_path).expect("Failed to rename fixture file to .old");
    }

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(path)
        .expect("Failed to create CSV writer for fixture file");

    let headers = ["name", "expected_hash_u64"];
    writer.write_record(&headers).expect("Failed to write headers to fixture file");

    let mut fixtures_vec = fixtures.values()
        .filter(|fixture| fixture.actual_hash.is_some())  // remove skipped tests
        .collect::<Vec<_>>();

    fixtures_vec.sort();

    for fixture in fixtures_vec {
        // the actual hash becomes the expected hash, because we're updating the fixtures
        // encode the expected hash as a big-endian hex string
        let new_expected_hash = fixture.actual_hash
            .map(|h| hex::encode_upper(h.to_be_bytes()))
            .expect("Actual hash should be set for fixture before writing");

        writer.write_record(&[&fixture.name, &new_expected_hash])
            .expect("Failed to write record to fixture file");
    }

    writer.flush().expect("Failed to flush CSV writer");
}

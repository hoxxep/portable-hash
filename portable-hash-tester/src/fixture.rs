pub(crate) enum FixtureState {
    New,
    Passed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Fixture {
    pub name: String,
    pub expected_hash: Option<u64>,
    pub actual_hash: Option<u64>,
}

impl Fixture {
    pub fn new(name: String) -> Self {
        Self {
            name,
            expected_hash: None,
            actual_hash: None,
        }
    }

    pub fn state(&self) -> FixtureState {
        match (self.expected_hash, self.actual_hash) {
            (Some(l), Some(r)) if l == r => FixtureState::Passed,
            (Some(_), Some(_)) => FixtureState::Failed,
            (None, Some(_)) => FixtureState::New,
            (Some(_), None) => FixtureState::Skipped,
            (None, None) => FixtureState::Skipped,  // TODO: shouldn't really happen?
        }
    }

    pub fn log_hash_result(&mut self, actual_hash: u64) {
        assert!(self.actual_hash.is_none(), "Duplicate test name '{}'. Hash result already logged.", self.name);
        self.actual_hash = Some(actual_hash);
    }
}

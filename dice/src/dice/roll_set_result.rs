use super::{RollResult, RollSet};

pub struct RollSetResult {
    results: Vec<RollResult>,
    total: usize,
}

impl RollSetResult {
    pub fn results(&self) -> &[RollResult] {
        &self.results
    }

    pub fn total(&self) -> &usize {
        &self.total
    }
}

impl From<&RollSet> for RollSetResult {
    fn from(set: &RollSet) -> Self {
        let results: Vec<RollResult> = set.rolls().iter().map(RollResult::from).collect();

        let total = results.iter().map(|r| r.total()).sum();

        Self { results, total }
    }
}

impl std::fmt::Display for RollSetResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for result in self.results.iter() {
            write!(f, "{}", result)?;
        }

        write!(f, "Total: {}", self.total)
    }
}

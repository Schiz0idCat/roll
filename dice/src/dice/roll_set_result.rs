use super::{RollResult, RollSet};

/// Represents the result of executing a [`RollSet`].
///
/// Contains the individual [`RollResult`] values
/// and the aggregated total of all rolls.
pub struct RollSetResult {
    /// Each roll result.
    results: Vec<RollResult>,

    /// Aggregated total of all contained roll results.
    total: usize,
}

impl RollSetResult {
    /// Returns each roll result.
    pub fn results(&self) -> &[RollResult] {
        &self.results
    }

    /// Returns the aggregated total of all roll results.
    pub fn total(&self) -> usize {
        self.total
    }
}

impl From<&RollSet> for RollSetResult {
    /// Makes the roll and stores the result
    fn from(set: &RollSet) -> Self {
        let results: Vec<RollResult> = set.rolls().iter().map(RollResult::from).collect();

        let total = results.iter().map(RollResult::total).sum();

        Self { results, total }
    }
}

impl std::fmt::Display for RollSetResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for result in self.results.iter() {
            writeln!(f, "{}", result)?;
        }

        write!(f, "\nTotal: {}", self.total)
    }
}

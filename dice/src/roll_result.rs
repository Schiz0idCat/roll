use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct RollResult {
    rolls: Vec<usize>,
    total: usize,
}

impl RollResult {
    pub fn new(rolls: Vec<usize>) -> Self {
        Self {
            rolls: rolls.clone(),
            total: rolls.iter().sum(),
        }
    }

    pub fn rolls(&self) -> Vec<usize> {
        self.rolls.clone()
    }

    pub fn total(&self) -> usize {
        self.total
    }
}

impl Ord for RollResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}

impl PartialOrd for RollResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

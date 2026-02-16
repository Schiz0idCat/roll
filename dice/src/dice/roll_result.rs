use crate::{Roll, RollType};

use std::cmp::Ordering;

use rand::random_range;

#[derive(PartialEq, Eq)]
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

impl From<Roll> for RollResult {
    fn from(roll: Roll) -> Self {
        match roll.roll_type() {
            RollType::Normal => {
                let rolls: Vec<usize> = (0..roll.amount())
                    .map(|_| random_range(1..=roll.die().sides()))
                    .collect();

                Self {
                    rolls: rolls.clone(),
                    total: rolls.iter().sum(),
                }
            }
            RollType::Advantage => {
                let rolls: Vec<usize> = (0..2)
                    .map(|_| random_range(1..=roll.die().sides()))
                    .collect();

                let total = *rolls.iter().max().unwrap();
                Self { rolls, total }
            }
            RollType::Disadvantage => {
                let rolls: Vec<usize> = (0..2)
                    .map(|_| random_range(1..=roll.die().sides()))
                    .collect();

                let total = *rolls.iter().min().unwrap();
                Self { rolls, total }
            }
        }
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

impl std::fmt::Display for RollResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rolls: {:?}", self.rolls())?;
        write!(f, "Total: {}", self.total())
    }
}

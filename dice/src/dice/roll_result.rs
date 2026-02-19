use crate::{Die, Roll, RollType};

use rand::random_range;

pub struct RollResult {
    rolls: Vec<usize>,
    die: Die,
    total: usize,
}

impl RollResult {
    pub fn rolls(&self) -> &[usize] {
        &self.rolls
    }

    pub fn die(&self) -> &Die {
        &self.die
    }

    pub fn total(&self) -> &usize {
        &self.total
    }
}

impl From<&Roll> for RollResult {
    fn from(roll: &Roll) -> Self {
        match roll.roll_type() {
            RollType::Normal => {
                let rolls: Vec<usize> = (0..*roll.amount())
                    .map(|_| random_range(1..=roll.die().sides()))
                    .collect();

                let total = rolls.iter().sum();

                Self {
                    rolls,
                    die: *roll.die(),
                    total,
                }
            }
            RollType::Advantage => {
                let rolls: Vec<usize> = (0..2)
                    .map(|_| random_range(1..=roll.die().sides()))
                    .collect();

                let total = *rolls.iter().max().unwrap();

                Self {
                    rolls,
                    die: *roll.die(),
                    total,
                }
            }
            RollType::Disadvantage => {
                let rolls: Vec<usize> = (0..2)
                    .map(|_| random_range(1..=roll.die().sides()))
                    .collect();

                let total = *rolls.iter().min().unwrap();

                Self {
                    rolls,
                    die: *roll.die(),
                    total,
                }
            }
        }
    }
}

impl std::fmt::Display for RollResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rolls().len() > 1 {
            writeln!(f, "Rolls: {:?}", self.rolls())?;
        }
        write!(f, "Total: {}", self.total())
    }
}

use crate::{Die, Roll, RollType};

use rand::random_range;

/// Represents the result of executing a [`Roll`].
///
/// Stores the individual dice outcomes, the die used,
/// and the computed total according to the [`RollType`].
pub struct RollResult {
    /// Result of each roll.
    rolls: Vec<usize>,

    /// Rolled die.
    die: Die,

    /// Total sum of all dice.
    total: usize,
}

impl RollResult {
    /// Returns the result of each roll.
    pub fn rolls(&self) -> &[usize] {
        &self.rolls
    }

    /// Returns the rolled die.
    pub fn die(&self) -> Die {
        self.die
    }

    /// Returns the computed total of the roll.
    ///
    /// For normal rolls this is the sum of all dice.
    /// For advantage or disadvantage rolls, this is
    /// respectively the highest or lowest value.
    pub fn total(&self) -> &usize {
        &self.total
    }
}

impl From<&Roll> for RollResult {
    /// Executes the given [`Roll`] and produces a `RollResult`.
    fn from(roll: &Roll) -> Self {
        let die = roll.die();
        let roll_type = roll.roll_type();

        let n = match roll_type {
            RollType::Normal => roll.amount(),
            RollType::Advantage | RollType::Disadvantage => 2,
        };

        let rolls: Vec<usize> = (0..n).map(|_| random_range(1..=die.sides())).collect();

        let total = match roll_type {
            RollType::Normal => rolls.iter().sum(),
            RollType::Advantage => *rolls.iter().max().unwrap(),
            RollType::Disadvantage => *rolls.iter().min().unwrap(),
        };

        Self { rolls, die, total }
    }
}

impl std::fmt::Display for RollResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: {:?} = {}",
            self.rolls().len(),
            self.die(),
            self.rolls(),
            self.total()
        )
    }
}

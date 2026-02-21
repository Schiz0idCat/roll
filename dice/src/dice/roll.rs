use super::parser::RollParser;
use super::{Die, RollResult, RollType, Rollable};

use std::convert::Infallible;
use std::str::FromStr;

/// Represents a dice roll configuration.
///
/// A `Roll` describes how many dice are rolled, which die is used,
/// and whether the roll is normal, with advantage, or with disadvantage.
pub struct Roll {
    /// Dice amount to roll.
    amount: usize,

    /// Die type to roll.
    die: Die,

    /// Normal, advantage or disadvantage roll.
    roll_type: RollType,

    /// Bonus to the result roll
    modifier: isize,
}

impl Roll {
    /// Returns roll data of a normal roll
    pub fn new(amount: usize, die: Die, modifier: isize) -> Self {
        Self {
            amount,
            die,
            roll_type: RollType::Normal,
            modifier,
        }
    }

    /// Creates a roll with a specific [`RollType`].
    ///
    /// The amount is implicitly set to `1`, since advantage and disadvantage
    /// rolls in D&D apply to a single die.
    pub fn new_with_type(die: Die, roll_type: RollType, modifier: isize) -> Self {
        Self {
            amount: 1,
            die,
            roll_type,
            modifier,
        }
    }

    /// Returns the amount of rolls.
    pub fn amount(&self) -> usize {
        self.amount
    }

    /// Returns the die to roll.
    pub fn die(&self) -> Die {
        self.die
    }

    /// Returns the roll type.
    pub fn roll_type(&self) -> &RollType {
        &self.roll_type
    }

    /// Returns the modifier.
    pub fn modifier(&self) -> isize {
        self.modifier
    }
}

impl Rollable for Roll {
    type Output = RollResult;

    /// Executes the roll and returns a [`RollResult`].
    fn roll(&self) -> Self::Output {
        RollResult::from(self)
    }
}

impl From<RollParser> for Roll {
    fn from(parser: RollParser) -> Self {
        let amount = parser.amount;
        let die = parser.die;
        let roll_type = match (parser.extra.advantage, parser.extra.disadvantage) {
            (true, false) => RollType::Advantage,
            (false, true) => RollType::Disadvantage,
            _ => RollType::Normal,
        };
        let modifier = parser.extra.modifier;

        match roll_type {
            RollType::Normal => Self::new(amount, die, modifier),
            _ => Self::new_with_type(die, roll_type, modifier),
        }
    }
}

impl FromStr for Roll {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = RollParser::from_str(s).unwrap();

        Ok(Roll::from(parser))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_n_returns_correct_length() {
        let n = 5;
        let result = Roll::new(n, Die::D8, 0).roll();

        assert_eq!(result.rolls().len(), n);
    }

    #[test]
    fn roll_n_values_are_in_range() {
        let n = 100;
        let result = Roll::new(n, Die::D10, 0).roll();

        for value in result.rolls() {
            assert!(*value >= 1);
            assert!(*value <= Die::D10.sides());
        }
    }

    #[test]
    fn roll_advantage() {
        let result = Roll::new_with_type(Die::D20, RollType::Advantage, 0).roll();

        assert!(result.rolls()[0] >= 1 && result.rolls()[0] <= 20);
        assert!(result.rolls()[1] >= 1 && result.rolls()[1] <= 20);
        assert!(*result.total() >= 1 && *result.total() <= 20);
        assert!(*result.total() == *result.rolls().iter().max().unwrap());
    }

    #[test]
    fn roll_disadvantage() {
        let result = Roll::new_with_type(Die::D20, RollType::Disadvantage, 0).roll();

        assert!(result.rolls()[0] >= 1 && result.rolls()[0] <= 20);
        assert!(result.rolls()[1] >= 1 && result.rolls()[1] <= 20);
        assert!(*result.total() >= 1 && *result.total() <= 20);
        assert!(*result.total() == *result.rolls().iter().min().unwrap());
    }
}

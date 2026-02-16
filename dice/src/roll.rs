use super::Die;
use super::RollResult;

use rand::random_range;

pub struct Roll;

impl Roll {
    pub fn roll(die: Die) -> RollResult {
        Self::roll_n(1, die)
    }

    pub fn roll_n(n: usize, die: Die) -> RollResult {
        let rolls: Vec<usize> = (0..n).map(|_| random_range(1..=die.sides())).collect();

        RollResult::new(rolls)
    }

    pub fn roll_advantage(die: Die) -> RollResult {
        let rolls = [Self::roll(die), Self::roll(die)];

        rolls.into_iter().max().unwrap()
    }

    pub fn roll_disadvantage(die: Die) -> RollResult {
        let rolls = [Self::roll(die), Self::roll(die)];

        rolls.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_returns_single_result() {
        let result = Roll::roll(Die::D6);

        assert_eq!(result.rolls().len(), 1);
        assert_eq!(result.total(), result.rolls()[0]);
        assert!(result.rolls()[0] >= 1);
        assert!(result.rolls()[0] <= Die::D6.sides());
    }

    #[test]
    fn roll_n_returns_correct_length() {
        let n = 5;
        let result = Roll::roll_n(n, Die::D8);

        assert_eq!(result.rolls().len(), n);
    }

    #[test]
    fn roll_n_values_are_in_range() {
        let n = 100;
        let result = Roll::roll_n(n, Die::D10);

        for value in &result.rolls() {
            assert!(*value >= 1);
            assert!(*value <= Die::D10.sides());
        }
    }
}

use super::{Die, RollResult, RollType, Rollable};

#[derive(Clone)]
pub struct Roll {
    amount: usize,
    die: Die,
    roll_type: RollType,
}

impl Roll {
    pub fn new(amount: usize, die: Die) -> Self {
        Self {
            amount,
            die,
            roll_type: RollType::Normal,
        }
    }

    pub fn new_single(die: Die) -> Self {
        Self {
            amount: 1,
            die,
            roll_type: RollType::Normal,
        }
    }

    pub fn new_with_type(die: Die, roll_type: RollType) -> Self {
        Self {
            amount: 1,
            die,
            roll_type,
        }
    }

    pub fn amount(&self) -> usize {
        self.amount
    }

    pub fn die(&self) -> Die {
        self.die
    }

    pub fn roll_type(&self) -> RollType {
        self.roll_type
    }
}

impl Rollable for Roll {
    type Output = RollResult;

    fn roll(&self) -> Self::Output {
        RollResult::from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_returns_single_result() {
        let result = Roll::new_single(Die::D6).roll();

        assert_eq!(result.rolls().len(), 1);
        assert_eq!(result.total(), result.rolls()[0]);
        assert!(result.rolls()[0] >= 1);
        assert!(result.rolls()[0] <= Die::D6.sides());
    }

    #[test]
    fn roll_n_returns_correct_length() {
        let n = 5;
        let result = Roll::new(n, Die::D8).roll();

        assert_eq!(result.rolls().len(), n);
    }

    #[test]
    fn roll_n_values_are_in_range() {
        let n = 100;
        let result = Roll::new(n, Die::D10).roll();

        for value in &result.rolls() {
            assert!(*value >= 1);
            assert!(*value <= Die::D10.sides());
        }
    }

    #[test]
    fn roll_advantage() {
        let result = Roll::new_with_type(Die::D20, RollType::Advantage).roll();

        assert!(result.rolls()[0] >= 1 && result.rolls()[0] <= 20);
        assert!(result.rolls()[1] >= 1 && result.rolls()[1] <= 20);
        assert!(result.total() >= 1 && result.total() <= 20);
        assert!(result.total() == *result.rolls().iter().max().unwrap());
    }

    #[test]
    fn roll_disadvantage() {
        let result = Roll::new_with_type(Die::D20, RollType::Disadvantage).roll();

        assert!(result.rolls()[0] >= 1 && result.rolls()[0] <= 20);
        assert!(result.rolls()[1] >= 1 && result.rolls()[1] <= 20);
        assert!(result.total() >= 1 && result.total() <= 20);
        assert!(result.total() == *result.rolls().iter().min().unwrap());
    }
}

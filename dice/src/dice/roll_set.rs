use super::{Roll, RollSetResult, Rollable};

/// A collection of [`Roll`] values.
///
/// `RollSet` allows grouping multiple independent roll configurations,
/// potentially with different dice types, into a single executable unit.
pub struct RollSet(Vec<Roll>);

impl RollSet {
    /// Creates a new `RollSet` from a collection of rolls.
    ///
    /// # Example
    ///
    /// ```
    /// use dice::{RollSet, Roll, Die};
    ///
    /// let set = RollSet::new(vec![
    ///     Roll::new(2, Die::D6),
    ///     Roll::new(1, Die::D20),
    /// ]);
    ///
    /// let result = set.roll();
    /// ```
    pub fn new(rolls: Vec<Roll>) -> Self {
        Self(rolls)
    }

    /// Returns each [`Roll`].
    pub fn rolls(&self) -> &[Roll] {
        &self.0
    }
}

impl Rollable for RollSet {
    type Output = RollSetResult;

    fn roll(&self) -> Self::Output {
        RollSetResult::from(self)
    }
}

use super::{Roll, RollOutput, RollSet, Rollable};

/// A polymorphic dice roll expression.
///
/// `RollExpr` allows treating both [`Roll`] and [`RollSet`] as a single
/// rollable entity. When executed, it produces a [`RollOutput`],
/// which may contain either a [`RollResult`](super::RollResult)
/// or a [`RollSetResult`](super::RollSetResult).
///
/// # Example
///
/// ```
/// use dice::{RollExpr, Roll, Die, Rollable};
///
/// let expr = RollExpr::Single(Roll::new(1, Die::D6, 0));
/// let result = expr.roll();
/// ```
pub enum RollExpr {
    Single(Roll),
    Set(RollSet),
}

impl Rollable for RollExpr {
    type Output = RollOutput;

    fn roll(&self) -> Self::Output {
        match self {
            RollExpr::Single(r) => RollOutput::Single(r.roll()),
            RollExpr::Set(s) => RollOutput::Set(s.roll()),
        }
    }
}

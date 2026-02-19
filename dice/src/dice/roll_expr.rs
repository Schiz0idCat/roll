use super::{Roll, RollOutput, RollSet, Rollable};

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

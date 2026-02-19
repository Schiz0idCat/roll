use super::{Roll, RollSetResult, Rollable};

pub struct RollSet(Vec<Roll>);

impl RollSet {
    pub fn new(rolls: Vec<Roll>) -> Self {
        Self(rolls)
    }

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

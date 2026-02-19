use super::{Roll, RollSetResult, Rollable};

#[derive(Clone)]
pub struct RollSet(Vec<Roll>);

impl RollSet {
    pub fn new(rolls: Vec<Roll>) -> Self {
        Self(rolls)
    }

    pub fn rolls(&self) -> Vec<Roll> {
        self.0.clone()
    }
}

impl Rollable for RollSet {
    type Output = RollSetResult;

    fn roll(&self) -> Self::Output {
        RollSetResult::from(self)
    }
}

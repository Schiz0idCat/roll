use crate::errors::CliError;
use dice::{Die, Roll, RollExpr, RollSet, RollType};

use clap::Parser;

use std::str::FromStr;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'a', long = "advantage", conflicts_with = "disadvantage")]
    advantage: bool,

    #[arg(short = 'd', long = "disadvantage", conflicts_with = "advantage")]
    disadvantage: bool,

    /// Notation NdM (e.g.: 3d6, d20, 12, 3d6 2d8 1d12)
    dice: Vec<String>,
}

impl Cli {
    pub fn try_parse(&self) -> Result<RollExpr, CliError> {
        if self.dice.is_empty() {
            if self.advantage || self.disadvantage {
                return Ok(RollExpr::Single(Roll::new_with_type(
                    Die::D20,
                    self.roll_type(),
                    0,
                )));
            }

            return Err(CliError::ParseDie);
        }

        let mut rolls = Vec::with_capacity(self.dice.len());

        for dice in &self.dice {
            rolls.push(Roll::from_str(dice).unwrap());
        }

        if rolls.len() == 1 {
            Ok(RollExpr::Single(rolls.pop().unwrap()))
        } else {
            Ok(RollExpr::Set(RollSet::new(rolls)))
        }
    }

    fn roll_type(&self) -> RollType {
        match (self.advantage, self.disadvantage) {
            (true, false) => RollType::Advantage,
            (false, true) => RollType::Disadvantage,
            _ => RollType::Normal,
        }
    }
}

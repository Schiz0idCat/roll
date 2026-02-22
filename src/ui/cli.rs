use crate::errors::CliError;
use dice::{Die, Roll, RollExpr, RollSet, RollType};

use clap::Parser;

use std::str::FromStr;

#[derive(Parser)]
pub struct Cli {
    #[arg(
        short = 'a',
        long = "advantage",
        conflicts_with_all = ["disadvantage", "dice"]
    )]
    advantage: bool,

    #[arg(
        short = 'd',
        long = "disadvantage",
        conflicts_with_all = ["advantage", "dice"]
    )]
    disadvantage: bool,

    /// Notation NdM (e.g.: 3d6, d20, 12, 3d6 2d8 1d12)
    dice: Vec<String>,
}

impl Cli {
    pub fn try_parse(&self) -> Result<RollExpr, CliError> {
        let roll_type = self.roll_type();

        if self.dice.is_empty() {
            if roll_type != RollType::Normal {
                return Ok(RollExpr::Single(Roll::new_with_type(
                    Die::D20,
                    roll_type,
                    0,
                )));
            }

            return Err(CliError::NoDie);
        }

        let mut rolls = Vec::with_capacity(self.dice.len());

        for dice in &self.dice {
            rolls.push(Roll::from_str(dice)?);
        }

        let rolls = RollSet::new(rolls);

        if rolls.rolls().len() == 1 {
            Ok(RollExpr::Single(rolls.rolls()[0].clone()))
        } else {
            Ok(RollExpr::Set(rolls))
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

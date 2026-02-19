use crate::errors::CliError;
use dice::{Die, Roll, RollExpr, RollSet, RollType};

use clap::Parser;

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
                )));
            }

            return Err(CliError::ParseDie);
        }

        let mut rolls = Vec::with_capacity(self.dice.len());

        for dice in &self.dice {
            rolls.push(self.parse_roll(dice)?);
        }

        if rolls.len() == 1 {
            Ok(RollExpr::Single(rolls.pop().unwrap()))
        } else {
            Ok(RollExpr::Set(RollSet::new(rolls)))
        }
    }

    fn parse_with_d(&self, dice: &str) -> Result<Roll, CliError> {
        let (amount, die) = dice.split_once('d').ok_or(CliError::ParseDie)?;

        let amount: usize = amount.parse().unwrap_or(1);
        let die = Die::try_from(die.parse::<usize>()?)?;

        if amount > 2 && (self.advantage || self.disadvantage) {
            return Err(CliError::InvalidAdvantageMultiplicity);
        }

        if amount == 1 {
            return Ok(Roll::new_with_type(die, self.roll_type()));
        }

        Ok(Roll::new(amount, die))
    }

    fn parse_die(&self, dice: &str) -> Result<Die, CliError> {
        let n: usize = dice.parse()?;

        Ok(Die::try_from(n)?)
    }

    fn parse_roll(&self, dice: &str) -> Result<Roll, CliError> {
        if dice.contains('d') {
            self.parse_with_d(dice)
        } else {
            Ok(Roll::new_with_type(self.parse_die(dice)?, self.roll_type()))
        }
    }

    fn roll_type(&self) -> RollType {
        if self.advantage {
            RollType::Advantage
        } else if self.disadvantage {
            RollType::Disadvantage
        } else {
            RollType::Normal
        }
    }
}

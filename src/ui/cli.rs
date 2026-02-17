use crate::errors::CliError;
use dice::{Die, Roll, RollType};

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'a', long = "advantage", conflicts_with = "disadvantage")]
    advantage: bool,

    #[arg(short = 'd', long = "disadvantage", conflicts_with = "advantage")]
    disadvantage: bool,

    /// Notación NdM (e.g.: 3d6, 1d20)
    dice: Option<String>,
}

impl Cli {
    pub fn try_parse(&self) -> Result<Roll, CliError> {
        match self.dice.as_deref() {
            None => {
                if self.advantage || self.disadvantage {
                    Ok(Roll::new_with_type(Die::D20, self.roll_type()))
                } else {
                    Err(CliError::ParseDie)
                }
            }
            Some(dice) if dice.contains('d') => self.parse_with_d(dice),
            Some(dice) => Ok(Roll::new_with_type(self.parse_die(dice)?, self.roll_type())),
        }
    }

    fn parse_with_d(&self, dice: &str) -> Result<Roll, CliError> {
        let (amount, die) = dice.split_once('d').ok_or(CliError::ParseDie)?;

        let amount: usize = amount.parse().unwrap_or(1);
        let die = Die::try_from(die.parse::<usize>()?)?;

        if amount != 1 && (self.advantage || self.disadvantage) {
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

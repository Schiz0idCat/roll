use super::super::Die;
use super::errors::RollParserError;
use super::{Components, Extra, Split};

use std::str::FromStr;

pub struct RollParser {
    pub die: Die,
    pub amount: usize,
    pub extra: Extra,
}

impl FromStr for RollParser {
    type Err = RollParserError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(RollParserError::EmptyDie);
        }

        let die = input.trim().to_lowercase();
        let splited = die.parse::<Split>().unwrap();

        let die = match splited.sides {
            Some(s) => s.parse::<Die>()?,
            None => return Err(RollParserError::NoSides),
        };

        let amount = match splited.amount {
            Some(a) => a.parse::<usize>()?,
            None => 1,
        };

        let components = match splited.components {
            Some(s) => s,
            None => String::from(""),
        };
        let components = Components::from_str(&components)?;
        let extra = Extra::try_from(components)?;

        if amount > 2 && (extra.advantage || extra.disadvantage) {
            return Err(RollParserError::InvalidAdvantageMultiplicity);
        }

        Ok(Self { die, amount, extra })
    }
}

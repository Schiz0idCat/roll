use super::{Die, Roll, RollType};
use crate::errors::RollParserError;

pub struct RollParser {
    pub die: Die,
    pub amount: usize,
    pub extras: Extras,
}

struct Splited {
    pub amount: Option<String>,
    pub sides: Option<String>,
    pub extras: Option<String>,
}

struct Extras {
    pub advantage: bool,
    pub disadvantage: bool,
    pub modifier: isize,
}

enum Component {
    Advantage,
    Disadvantage,
    Modifier(isize),
}

impl RollParser {
    pub fn try_parse(&self, die: &str) -> Result<Roll, RollParserError> {
        if die.is_empty() {
            return Err(RollParserError::EmptyDie);
        }

        let die = die.trim().to_lowercase();
        let splited = self.split(&die);

        let die = match splited.sides {
            Some(s) => s.parse::<Die>()?,
            None => return Err(RollParserError::NoSides),
        };

        let amount = match splited.amount {
            Some(a) => a.parse::<usize>()?,
            None => 1,
        };

        let roll = RollParser { die, amount };

        if let Some(extra) = splited.extras {
            self.parse_extra(&extra)?;
        }

        // self.parse_roll(&die)
    }

    fn split(&self, input: &str) -> Splited {
        // amount of rolls + chars
        let (amount, rest) = match input.split_once('d') {
            Some((left, right)) => ((!left.is_empty()).then(|| left.to_string()), right),
            None => (None, input),
        };

        // rest of the chars
        let mut chars = rest.chars();

        // take the sides
        let digits: String = chars.by_ref().take_while(|c| c.is_ascii_digit()).collect();
        let sides = (!digits.is_empty()).then_some(digits);

        // extra parameters
        let extras_str: String = chars.collect();
        let extras = (!extras_str.is_empty()).then_some(extras_str);

        Splited {
            amount,
            sides,
            extras,
        }
    }

    fn parse_extra(mut input: &str) -> Result<Vec<Component>, RollParserError> {
        let mut components = Vec::new();

        while !input.is_empty() {
            if let Some(rest) = input.strip_prefix("adv") {
                components.push(Component::Advantage);
                input = rest;
                continue;
            }

            if let Some(rest) = input.strip_prefix("dis") {
                components.push(Component::Disadvantage);
                input = rest;
                continue;
            }

            if input.starts_with('+') || input.starts_with('-') {
                let digits = input[1..]
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .count();

                if digits == 0 {
                    return Err(RollParserError::InvalidModifier);
                }

                let number: isize = input[..1 + digits]
                    .parse()
                    .map_err(|_| RollParserError::InvalidModifier)?;

                components.push(Component::Modifier(number));
                input = &input[1 + digits..];
                continue;
            }

            return Err(RollParserError::InvalidExtra);
        }

        Ok(components)
    }

    // fn parse_roll(&self, dice: &str) -> Result<Roll, RollParserError> {
    //     if dice.contains('d') {
    //         self.parse_with_d(dice)
    //     } else {
    //         Ok(Roll::new_with_type(self.parse_die(dice)?, self.roll_type()))
    //     }
    // }
    //
    // fn parse_with_d(&self, dice: &str) -> Result<Roll, RollParserError> {
    //     let (amount, die) = dice.split_once('d').ok_or(RollParserError::ParseDie)?;
    //
    //     let amount: usize = amount.parse().unwrap_or(1);
    //     let die = Die::try_from(die.parse::<usize>()?)?;
    //
    //     if amount > 2 && (self.advantage || self.disadvantage) {
    //         return Err(CliError::InvalidAdvantageMultiplicity);
    //     }
    //
    //     if amount == 1 {
    //         return Ok(Roll::new_with_type(die, self.roll_type()));
    //     }
    //
    //     Ok(Roll::new(amount, die))
    // }
    //
    // fn parse_die(&self, dice: &str) -> Result<Die, CliError> {
    //     let n: usize = dice.parse()?;
    //
    //     Ok(Die::try_from(n)?)
    // }
    //
    // fn roll_type(&self) -> RollType {
    //     if self.advantage {
    //         RollType::Advantage
    //     } else if self.disadvantage {
    //         RollType::Disadvantage
    //     } else {
    //         RollType::Normal
    //     }
    // }
}

use super::errors::ComponentError;

use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub enum Component {
    Advantage,
    Disadvantage,
    Modifier(isize),
}

impl FromStr for Component {
    type Err = ComponentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "adv" => Ok(Component::Advantage),
            "dis" => Ok(Component::Disadvantage),
            _ if s.starts_with('+') || s.starts_with('-') => {
                let int: isize = s.parse()?;

                Ok(Component::Modifier(int))
            }
            _ => Err(ComponentError::InvalidComponent(s.to_string())),
        }
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Component::Advantage => write!(f, "adv"),
            Component::Disadvantage => write!(f, "dis"),
            Component::Modifier(n) => write!(f, "{:+}", n),
        }
    }
}

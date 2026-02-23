use super::Component;
use super::errors::ComponentError;

use std::str::FromStr;

pub struct Components(Vec<Component>);

impl Components {
    fn parse_component(input: &str) -> Result<Option<(Component, &str)>, ComponentError> {
        if let Some(res) = Self::parse_advantage(input) {
            return Ok(Some(res));
        }

        if let Some(res) = Self::parse_disadvantage(input) {
            return Ok(Some(res));
        }

        if let Some(res) = Self::parse_modifier(input)? {
            return Ok(Some(res));
        }

        Ok(None)
    }

    fn parse_advantage(input: &str) -> Option<(Component, &str)> {
        if let Some(rest) = input.strip_prefix(&Component::Advantage.to_string()) {
            return Some((Component::Advantage, rest));
        }

        None
    }

    fn parse_disadvantage(input: &str) -> Option<(Component, &str)> {
        if let Some(rest) = input.strip_prefix(&Component::Disadvantage.to_string()) {
            return Some((Component::Disadvantage, rest));
        }

        None
    }

    fn parse_modifier(input: &str) -> Result<Option<(Component, &str)>, ComponentError> {
        if !(input.starts_with('+') || input.starts_with('-')) {
            return Ok(None);
        }

        let digits = input[1..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .count();

        if digits == 0 {
            return Err(ComponentError::InvalidModifier);
        }

        let token = &input[..1 + digits];
        let component = token.parse()?;
        let rest = &input[1 + digits..];

        Ok(Some((component, rest)))
    }
}

impl FromStr for Components {
    type Err = ComponentError;

    fn from_str(mut input: &str) -> Result<Self, Self::Err> {
        let mut components = Vec::new();

        while !input.is_empty() {
            match Self::parse_component(input)? {
                Some((component, rest)) => {
                    components.push(component);
                    input = rest;
                }
                None => {
                    return Err(ComponentError::InvalidComponent(input.to_string()));
                }
            }
        }

        Ok(Components(components))
    }
}

impl IntoIterator for Components {
    type Item = Component;
    type IntoIter = std::vec::IntoIter<Component>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

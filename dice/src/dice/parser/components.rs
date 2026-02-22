use super::Component;
use super::errors::ComponentError;

use std::str::FromStr;

pub struct Components(Vec<Component>);

impl FromStr for Components {
    type Err = ComponentError;

    fn from_str(mut input: &str) -> Result<Self, Self::Err> {
        let mut components = Vec::new();

        while !input.is_empty() {
            // advantage
            if let Some(rest) = input.strip_prefix("adv") {
                components.push("adv".parse()?);
                input = rest;
                continue;
            }

            // disadvantage
            if let Some(rest) = input.strip_prefix("dis") {
                components.push("dis".parse()?);
                input = rest;
                continue;
            }

            // modifier (+/-int)
            if input.starts_with('+') || input.starts_with('-') {
                let digits = input[1..]
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .count();

                if digits == 0 {
                    return Err(ComponentError::InvalidModifier.into());
                }

                let token = &input[..1 + digits];
                components.push(token.parse()?);
                input = &input[1 + digits..];
                continue;
            }

            return Err(ComponentError::InvalidComponent(input.to_string()).into());
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

use super::Component;
use super::errors::ComponentError;

use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
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

impl Deref for Components {
    type Target = [Component];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Components {
    type Item = Component;
    type IntoIter = std::vec::IntoIter<Component>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_advantage() {
        let adv = "advfarsdl";
        let (component, rest) = Components::parse_advantage(adv).unwrap();

        assert_eq!(component, Component::Advantage);
        assert_eq!(rest, "farsdl");

        let adv = "farsdladv";

        assert!(Components::parse_advantage(adv).is_none());
    }

    #[test]
    fn parse_disadvantage() {
        let dis = "disfarsdl";
        let (component, rest) = Components::parse_disadvantage(dis).unwrap();

        assert_eq!(component, Component::Disadvantage);
        assert_eq!(rest, "farsdl");

        let dis = "farsdisfasdf";

        assert!(Components::parse_disadvantage(dis).is_none());
    }

    #[test]
    fn parse_modifier() {
        let modifier = "+7asdf";
        let (component, rest) = Components::parse_modifier(modifier).unwrap().unwrap();

        assert_eq!(component, Component::Modifier(7));
        assert_eq!(rest, "asdf");

        let dis = "fars-4fasdf";

        assert!(Components::parse_disadvantage(dis).is_none());
    }

    #[test]
    fn parse_component() {
        let components = "advdis-4+7";
        let (adv, rest) = Components::parse_component(components).unwrap().unwrap();

        assert_eq!(adv, Component::Advantage);
        assert_eq!(rest, "dis-4+7");

        let components = rest;
        let (dis, rest) = Components::parse_component(components).unwrap().unwrap();

        assert_eq!(dis, Component::Disadvantage);
        assert_eq!(rest, "-4+7");

        let components = rest;
        let (minus_four, rest) = Components::parse_component(components).unwrap().unwrap();

        assert_eq!(minus_four, Component::Modifier(-4));
        assert_eq!(rest, "+7");

        let components = rest;
        let (plus_seven, rest) = Components::parse_component(components).unwrap().unwrap();

        assert_eq!(plus_seven, Component::Modifier(7));
        assert_eq!(rest, "");
    }

    #[test]
    fn from_str() {
        let components = Components::from_str("dis+7adv-2").unwrap();

        assert_eq!(
            *components,
            vec![
                Component::Disadvantage,
                Component::Modifier(7),
                Component::Advantage,
                Component::Modifier(-2)
            ]
        );

        let components = Components::from_str("dis+7adv-").unwrap_err();

        assert_eq!(components, ComponentError::InvalidModifier)
    }
}

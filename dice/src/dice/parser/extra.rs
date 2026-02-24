use super::errors::ComponentError;
use super::{Component, Components};

#[derive(Debug)]
pub struct Extra {
    advantage: bool,
    disadvantage: bool,
    modifier: isize,
}

impl Extra {
    pub fn advantage(&self) -> bool {
        self.advantage
    }

    pub fn disadvantage(&self) -> bool {
        self.disadvantage
    }

    pub fn modifier(&self) -> isize {
        self.modifier
    }
}

impl TryFrom<Components> for Extra {
    type Error = ComponentError;

    fn try_from(components: Components) -> Result<Self, Self::Error> {
        let mut advantage = false;
        let mut disadvantage = false;
        let mut modifier: isize = 0;

        for component in components {
            match component {
                Component::Advantage => {
                    if disadvantage {
                        return Err(ComponentError::ConflictingComponents(
                            Component::Advantage,
                            Component::Disadvantage,
                        ));
                    }

                    advantage = true;
                }
                Component::Disadvantage => {
                    if advantage {
                        return Err(ComponentError::ConflictingComponents(
                            Component::Disadvantage,
                            Component::Advantage,
                        ));
                    }

                    disadvantage = true;
                }
                Component::Modifier(int) => modifier += int,
            }
        }

        Ok(Self {
            advantage,
            disadvantage,
            modifier,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn read_components() {
        let components = Components::from_str("advadv+5-2").unwrap();
        let extra = Extra::try_from(components).unwrap();

        assert_eq!(extra.advantage(), true);
        assert_eq!(extra.disadvantage(), false);
        assert_eq!(extra.modifier(), 3);

        let components = Components::from_str("disadv-2").unwrap();
        let extra = Extra::try_from(components).unwrap_err();

        assert_eq!(
            extra,
            ComponentError::ConflictingComponents(Component::Advantage, Component::Disadvantage)
        )
    }
}

use super::errors::ComponentError;
use super::{Component, Components};

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

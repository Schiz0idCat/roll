use crate::errors::DieError;

#[derive(Clone, Copy)]
pub enum Die {
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100,
}

impl Die {
    pub fn sides(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<usize> for Die {
    type Error = DieError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Die::D4),
            6 => Ok(Die::D6),
            8 => Ok(Die::D8),
            10 => Ok(Die::D10),
            12 => Ok(Die::D12),
            20 => Ok(Die::D20),
            100 => Ok(Die::D100),
            _ => Err(DieError::DieNotRecognized),
        }
    }
}

impl std::fmt::Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "D{}", *self as usize)
    }
}

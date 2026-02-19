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
    pub fn sides(self) -> usize {
        self as usize
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
        match self {
            Die::D4 => write!(f, "D4"),
            Die::D6 => write!(f, "D6"),
            Die::D8 => write!(f, "D8"),
            Die::D10 => write!(f, "D10"),
            Die::D12 => write!(f, "D12"),
            Die::D20 => write!(f, "D20"),
            Die::D100 => write!(f, "D100"),
        }
    }
}

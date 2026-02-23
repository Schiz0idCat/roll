use std::convert::Infallible;
use std::str::FromStr;

pub struct Split {
    amount: Option<String>,
    sides: Option<String>,
    components: Option<String>,
}

impl Split {
    fn new(amount: Option<String>, sides: Option<String>, components: Option<String>) -> Self {
        Self {
            amount,
            sides,
            components,
        }
    }

    pub fn amount(&self) -> &Option<String> {
        &self.amount
    }

    pub fn sides(&self) -> &Option<String> {
        &self.sides
    }

    pub fn components(&self) -> &Option<String> {
        &self.components
    }
}

impl FromStr for Split {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // amount of rolls + chars
        let (amount, rest) = match s.split_once('d') {
            Some((left, right)) => ((!left.is_empty()).then(|| left.to_string()), right),
            None => (None, s),
        };

        // take the sides
        let digits_end = rest
            .char_indices()
            .position(|(_, c)| !c.is_ascii_digit())
            .unwrap_or(rest.len());

        let (digits, components) = rest.split_at(digits_end);

        let sides = (!digits.is_empty()).then_some(digits.to_string());

        // extra parameters
        let components = (!components.is_empty()).then_some(components.to_string());

        Ok(Split::new(amount, sides, components))
    }
}

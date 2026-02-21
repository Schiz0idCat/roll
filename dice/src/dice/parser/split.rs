use std::convert::Infallible;
use std::str::FromStr;

pub struct Split {
    pub amount: Option<String>,
    pub sides: Option<String>,
    pub components: Option<String>,
}

impl Split {
    fn new(amount: Option<String>, sides: Option<String>, components: Option<String>) -> Self {
        Self {
            amount,
            sides,
            components,
        }
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

        // rest of the chars
        let mut chars = rest.chars();

        // take the sides
        let digits: String = chars.by_ref().take_while(|c| c.is_ascii_digit()).collect();
        let sides = (!digits.is_empty()).then_some(digits);

        // extra parameters
        let components: String = chars.collect();
        let components = (!components.is_empty()).then_some(components);

        Ok(Split::new(amount, sides, components))
    }
}

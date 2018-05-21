use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Category(String);

impl FromStr for Category {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Validation
        Ok(Category(s.to_string()))
    }
}

#[derive(Clone, Debug, Fail)]
pub enum ParseError {
    #[fail(display = "unknown category: {}", _0)]
    UnknownCategory(String),
}

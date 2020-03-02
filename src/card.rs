use std::fmt;

#[derive(PartialEq)]
pub enum Arrow {
    Empty,
    Single,
    Double,
}

impl fmt::Display for Arrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arrow::Empty => write!(f, " "),
            Arrow::Single => write!(f, "*"),
            Arrow::Double => write!(f, "$"),
        }
    }
}

impl Default for Arrow {
    fn default() -> Self { Arrow::Empty }
}

#[derive(Default)]
pub struct Card {
    // Arrows starting at north, going clockwise.
    pub arrows: [Arrow; 4],
    pub good: bool,
}

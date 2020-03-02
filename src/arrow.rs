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

use crate::card::Card;

#[derive(PartialEq)]
pub enum ActionType {
    // Relative to the row/col pair, integer used for indexing into array!
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Place = -1,

}

impl ActionType {
    pub fn opposite(&self) -> ActionType {
        match &self {
            ActionType::North => ActionType::South,
            ActionType::South => ActionType::North,
            ActionType::East => ActionType::West,
            ActionType::West => ActionType::East,
            ActionType::Place => ActionType::Place,
        }
    }
}

pub struct Action {
    pub row: usize,
    pub col: usize,
    pub action_type: ActionType,
    pub card: Card,
}


use crate::card::Card;

#[derive(PartialEq)]
pub enum TileType {
    Passable,
    NotPassable,
    Gem,
}

impl Default for TileType {
    fn default() -> Self { TileType::NotPassable }
}

#[derive(Default)]
pub struct Tile {
    pub tile_type: TileType,
    pub card: Option<Card>,
}


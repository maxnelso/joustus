use colored::*;
use std::fmt;
use std::fs;

use crate::action::Action;
use crate::action::ActionType;
use crate::card::Arrow;
use crate::card::Card;
use crate::tile::Tile;
use crate::tile::TileType;

#[derive(Default)]
pub struct Grid {
     pub tiles: [[Tile; 4]; 4],
}

impl Grid {
    pub fn new(path: String) -> Grid {
        let mut tiles: [[Tile; 4]; 4] = Default::default();
        let contents = fs::read_to_string(path).expect("couldn't read file");
        let split = contents.split("\n");
        let vec = split.collect::<Vec<&str>>();
        for i in 0..4 {
            for j in 0..4 {
                match vec[i].chars().nth(j).unwrap() {
                    'E' => tiles[i][j].tile_type = TileType::Passable,
                    'G' => tiles[i][j].tile_type = TileType::Gem,
                    _ => (),
                }
            }
        }
        return Grid{tiles: tiles};
    }

    fn is_valid_move(&self, action: Action) -> bool {
        let tile = &self.tiles[action.row][action.col];
        if action.action_type == ActionType::Place {
            if tile.tile_type == TileType::Gem {
                return false;
            }
            if tile.tile_type == TileType::NotPassable {
                return false;
            }
            // Spot already taken
            match &tile.card {
                Some(_card) => return false,
                None => return true,
            }
        }

        // Can't push what's not there!
        match &tile.card {
            Some(_card) => (),
            None => return false,
        }

        let opposite_dir = action.action_type.opposite() as usize;
        if action.card.arrows[opposite_dir] == Arrow::Empty {
            return false;
        }

        return true;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out: String = "".to_owned();
        for row in self.tiles.iter() {
            fn color_string(s: String, tile: &Tile) -> String {
                match &tile.card {
                    Some(card) => {
                        if card.good {
                            return s.blue().bold().to_string();
                        }
                        return s.red().bold().to_string();
                    },
                    None => return s,
                }
            }

            out.push_str("=========================\n");
            out.push_str("|");
            // North
            for tile in row {
                match tile.tile_type {
                    TileType::NotPassable => {
                        out.push_str("xxxxx|");
                        continue;
                    },
                    _ => (),
                }

                out.push_str("  ");
                match &tile.card {
                    Some(card) => out.push_str(&color_string(card.arrows[0].to_string(), tile)),
                    None => out.push_str(" "),
                }
                out.push_str("  |");
            }
            out.push_str("\n");
            out.push_str("|");
            // West/East
            for tile in row {
                match tile.tile_type {
                    TileType::NotPassable => {
                        out.push_str("xxxxx|");
                        continue;
                    },
                    _ => (),
                }
                out.push_str(" ");
                match &tile.card {
                    Some(card) => out.push_str(&color_string(card.arrows[3].to_string(), tile)),
                    None => out.push_str(" "),
                }
                out.push_str(" ");
                match &tile.card {
                    Some(card) => out.push_str(&color_string(card.arrows[1].to_string(), tile)),
                    None => out.push_str(" "),
                }
                out.push_str(" |");
            }
            out.push_str("\n");
            out.push_str("|");
            // South
            for tile in row {
                match tile.tile_type {
                    TileType::NotPassable => {
                        out.push_str("xxxxx|");
                        continue;
                    },
                    _ => (),
                }
                out.push_str("  ");
                match &tile.card {
                    Some(card) => out.push_str(&color_string(card.arrows[2].to_string(), tile)),
                    None => out.push_str(" "),
                }
                out.push_str("  |");
            }
            out.push_str("\n");

        }
        out.push_str("=========================\n");
        return write!(f, "{}", out);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_valid_move_gem() {
        let mut grid: Grid = Default::default();
        grid.tiles[0][0].tile_type = TileType::Gem;
        assert_eq!(grid.is_valid_move(Action{
            row: 0,
            col: 0,
            action_type: ActionType::Place,
            card: Default::default(),
        }), false);
    }

    #[test]
    fn test_is_valid_move_has_arrow() {
        let mut grid: Grid = Default::default();
        let placed_card: Card = Default::default();
        grid.tiles[0][0].card = Some(placed_card);
        let mut card: Card = Default::default();
        card.arrows[0] = Arrow::Single;
        assert_eq!(grid.is_valid_move(Action{
            row: 0,
            col: 0,
            action_type: ActionType::South,
            card: card,
        }), true);
    }

    #[test]
    fn test_is_valid_move_doesnt_have_arrow() {
        let mut grid: Grid = Default::default();
        let placed_card: Card = Default::default();
        grid.tiles[0][0].card = Some(placed_card);
        let mut card: Card = Default::default();
        card.arrows[0] = Arrow::Single;
        assert_eq!(grid.is_valid_move(Action{
            row: 0,
            col: 0,
            action_type: ActionType::East,
            card: Default::default(),
        }), false);
    }

}

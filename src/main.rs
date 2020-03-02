mod action;
mod grid;
mod card;
mod tile;

use crate::card::Arrow;
use crate::card::Card;
use crate::grid::Grid;

fn main() {
    let mut grid = Grid::new("data/A.grid".to_string());
    let mut c1: Card = Default::default();
    c1.arrows[0] = Arrow::Single;
    let mut c2: Card = Default::default();
    c2.arrows[0] = Arrow::Single;
    c2.arrows[1] = Arrow::Single;
    c2.arrows[2] = Arrow::Double;
    c2.arrows[3] = Arrow::Double;
    c2.good = true;
    grid.tiles[0][1].card = Some(c1);
    grid.tiles[1][0].card = Some(c2);
    println!("{}", grid);
}

use std::fmt;
use std::fs;

enum Arrow {
    Empty,
    Single,
    Double,
}

struct Card {
    // Arrows starting at north, going clockwise.
    arrows: [Arrow; 4],
}

struct Tile {
    gem: bool,
    card: Option<Card>,
}

struct Grid {
    tiles: [[Tile; 4]; 4],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "supsup")
    }
}

fn read_grid(path: String) -> Grid {
    let contents = fs::read_to_string(path).expect("couldn't read file");
    let tiles: [[Tile; 4]; 4];
}

fn main() {
    println!("Hello supsup!");
}

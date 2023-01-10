use std::fmt::Display;

use bevy::prelude::Component;

#[derive(Clone, Debug, Default)]
pub struct Map {
    pub start: usize,
    pub width: usize,
    pub heigth: usize,
    pub tiles: Vec<Vec<MapTile>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Map size: {} {}", self.width, self.heigth)?;

        for row in &self.tiles {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile {
                        MapTile::Player1 => '@',
                        MapTile::Player2 => '$',
                        _ => '.',
                    }
                )?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Default, Component, Copy, Clone, Debug, PartialEq, Eq)]
pub enum MapTile {
    Player1, // a & @
    Player2, // s & $
    Empty,   // .

    #[default]
    None,
}

impl MapTile {
    pub fn from_char(c: char) -> Self {
        match c {
            'a' | '@' => Self::Player1,
            's' | '$' => Self::Player2,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

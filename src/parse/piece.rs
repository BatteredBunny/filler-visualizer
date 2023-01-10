use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct Piece {
    pub start: usize,
    pub width: usize,
    pub heigth: usize,
    pub tiles: Vec<Vec<PieceTile>>,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Piece size: {} {}", self.width, self.heigth)?;

        for row in &self.tiles {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile {
                        PieceTile::Piece => "O",
                        PieceTile::Empty => ".",
                    }
                )?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PieceTile {
    Piece, // O
    Empty, // .
}

impl PieceTile {
    pub fn from_char(c: char) -> Self {
        match c {
            'O' => Self::Piece,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

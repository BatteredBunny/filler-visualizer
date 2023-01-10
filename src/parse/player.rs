use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub num: usize,
    pub path: String,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player {} [{}]", self.num, self.path)
    }
}

use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct Answer {
    pub player: String,
    pub answer: Option<(usize, usize)>,
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Answer ({}): ", self.player)?;

        match self.answer {
            Some(ans) => write!(f, "{ans:?}")?,
            None => write!(f, "Invalid answer")?,
        };

        Ok(())
    }
}

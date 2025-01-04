use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq)]
pub enum Direction {
    Left,
    Right
}

pub struct TapeTransition {
    pub write: Option<char>,
    pub direction: Direction,
    pub disp_string: String, // this is kind of a lol but whatever
}

impl Display for TapeTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.disp_string)
    }
}

impl Debug for TapeTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

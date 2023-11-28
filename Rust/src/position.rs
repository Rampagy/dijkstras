use std::fmt;
use std::hash::{Hasher};

//#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub state: u64,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

impl Hasher for Position {
    fn write(&mut self, _bytes: &[u8]) {
        /* szudziks function */
        if self.x >= self.y
        {
            self.state = self.x as u64 * self.x as u64 + self.x as u64 + self.y as u64;
        }
        else
        {
            self.state = self.x as u64 + self.y as u64 * self.y as u64;
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}
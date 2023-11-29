use std::fmt;
//use std::hash::{Hasher};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,

    _state: u64,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

impl Position {
    // Constructor will pass in x and y, default state to 0
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y, _state: 0 }
    }
}

/* TODO: Implement custom hash function */

/*
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
*/
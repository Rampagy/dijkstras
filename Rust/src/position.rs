use std::fmt;
use std::hash::{BuildHasher, Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}


impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}


impl Position {
    // Constructor will pass in x and y, default state to 0
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn get_surrounding_positions(&self) -> [Position; 4] {
        return [
            Position::new(self.x + 0, self.y - 1),  // north
            Position::new(self.x + 1, self.y + 0),  // east
            Position::new(self.x + 0, self.y + 1),  // south
            Position::new(self.x - 1, self.y + 0),  // west
        ]
    }
}


impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


impl Eq for Position {}


/* https://stackoverflow.com/questions/77588838/how-to-create-a-custom-hash-function-in-rust */
/* https://www.reddit.com/r/rust/comments/184xnxo/hey_rustaceans_got_a_question_ask_here_482023/kbmj1xb/ */
impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        assert!(self.x >= 0);
        assert!(self.y >= 0);

        let x: u64 = self.x as u64;
        let y: u64 = self.y as u64;

        /* szudzik's pairing function */
        let hash_val: u64 = if x >= y {
            x * x + x + y
        } else {
            x + y * y
        };

        state.write_u64(hash_val);
    }
}

pub struct PositionBuildHasher;

impl BuildHasher for PositionBuildHasher {
    type Hasher = PositionHasher;

    fn build_hasher(&self) -> Self::Hasher {
        PositionHasher(0)
    }
}

pub struct PositionHasher(u64);

impl Hasher for PositionHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0 = u64::from_ne_bytes(bytes.try_into().expect("Can only write 8 byte values"));
    }

    fn finish(&self) -> u64 {
        self.0
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}
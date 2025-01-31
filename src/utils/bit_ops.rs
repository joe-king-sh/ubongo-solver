use std::ops::{BitAnd, BitXor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(u64);

pub fn set_bit(board: &mut BitBoard, x: usize, y: usize, width: usize, value: bool) {
    let index = y * width + x;
    if value {
        board.0 |= 1 << index;
    } else {
        board.0 &= !(1 << index);
    }
}

pub fn get_bit(board: &BitBoard, x: usize, y: usize, width: usize) -> bool {
    let index = y * width + x;
    (board.0 >> index) & 1 == 1
}

impl BitBoard {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        get_bit(self, x, y, 5)
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        set_bit(self, x, y, 5, value)
    }

    pub fn not_any(&self) -> bool {
        self.0 == 0
    }

    pub fn fill(&mut self, value: bool) {
        if value {
            self.0 = u64::MAX;
        } else {
            self.0 = 0;
        }
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

use std::ops::{BitAnd, BitXor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard {
    bits: u64,
}

pub fn set_bit(board: &mut BitBoard, x: usize, y: usize, width: usize, value: bool) {
    let index = y * width + x;
    if value {
        board.bits |= 1 << index;
    } else {
        board.bits &= !(1 << index);
    }
}

pub fn get_bit(board: &BitBoard, x: usize, y: usize, width: usize) -> bool {
    let index = y * width + x;
    (board.bits >> index) & 1 == 1
}

impl BitBoard {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let idx = y * 8 + x;
        (self.bits >> idx) & 1 == 1
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let idx = y * 8 + x;
        if value {
            self.bits |= 1 << idx;
        } else {
            self.bits &= !(1 << idx);
        }
    }

    pub fn not_any(&self) -> bool {
        self.bits == 0
    }

    pub fn fill(&mut self, value: bool) {
        self.bits = if value { u64::MAX } else { 0 };
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits,
        }
    }
}

use crate::{
    piece::Piece,
    utils::{bit_ops::BitBoard, types::Position},
};

#[derive(Debug, Clone)]
pub struct Board {
    width: usize,
    height: usize,
    state: BitBoard,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            state: BitBoard::new(),
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_state(&self) -> BitBoard {
        self.state
    }

    pub fn can_place_piece(&self, piece: &Piece, pos: Position) -> bool {
        let (x, y) = pos;
        let piece_board = piece.get_board();

        // Check if the piece fits within the board boundaries
        if x + piece.get_width() > self.width || y + piece.get_height() > self.height {
            return false;
        }

        // Check if the piece overlaps with any existing pieces
        for dy in 0..piece.get_height() {
            for dx in 0..piece.get_width() {
                if piece_board.get(dx, dy) && self.state.get(x + dx, y + dy) {
                    return false;
                }
            }
        }

        true
    }

    pub fn place_piece(&mut self, piece: &Piece, pos: Position) {
        let (x, y) = pos;
        let piece_board = piece.get_board();
        for dy in 0..piece.get_height() {
            for dx in 0..piece.get_width() {
                if piece_board.get(dx, dy) {
                    self.state.set(x + dx, y + dy, true);
                }
            }
        }
    }

    pub fn remove_piece(&mut self, piece: &Piece, pos: Position) {
        let (x, y) = pos;
        let piece_board = piece.get_board();
        for dy in 0..piece.get_height() {
            for dx in 0..piece.get_width() {
                if piece_board.get(dx, dy) {
                    self.state.set(x + dx, y + dy, false);
                }
            }
        }
    }

    pub fn display(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.state.get(x, y) {
                    result.push('■');
                } else {
                    result.push('□');
                }
            }
            result.push('\n');
        }
        result
    }

    pub fn is_position_empty(&self, pos: Position) -> bool {
        let (x, y) = pos;
        if x >= self.width || y >= self.height {
            return false;
        }
        !self.state.get(x, y)
    }

    pub fn is_empty(&self) -> bool {
        self.state.not_any()
    }

    pub fn clear(&mut self) {
        self.state.fill(false);
    }
}

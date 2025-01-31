use crate::utils::bit_ops::BitBoard;

#[derive(Debug, Clone)]
pub struct Piece {
    pub board: BitBoard,
    pub width: usize,
    pub height: usize,
    pub name: String,
}

impl Piece {
    pub fn new(shape: &[&str], name: &str) -> Self {
        let height = shape.len();
        let width = shape[0].len();
        let mut board = BitBoard::new();

        for (y, row) in shape.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '1' {
                    board.set(x, y, true);
                }
            }
        }

        Self {
            board,
            width,
            height,
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_board(&self) -> &BitBoard {
        &self.board
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_all_variants(&self) -> Vec<Piece> {
        let mut variants = Vec::with_capacity(8);
        let mut current = self.clone();

        // Original
        variants.push(current.clone());

        // Rotations
        for _ in 0..3 {
            current = current.rotate();
            variants.push(current.clone());
        }

        // Flipped
        current = self.flip();
        variants.push(current.clone());

        // Flipped rotations
        for _ in 0..3 {
            current = current.rotate();
            variants.push(current.clone());
        }

        variants
    }

    fn rotate(&self) -> Piece {
        let new_width = self.height;
        let new_height = self.width;
        let mut new_board = BitBoard::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.board.get(x, y) {
                    new_board.set(new_width - 1 - y, x, true);
                }
            }
        }

        Piece {
            board: new_board,
            width: new_width,
            height: new_height,
            name: self.name.clone(),
        }
    }

    fn flip(&self) -> Piece {
        let mut new_board = BitBoard::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.board.get(x, y) {
                    new_board.set(self.width - 1 - x, y, true);
                }
            }
        }

        Piece {
            board: new_board,
            width: self.width,
            height: self.height,
            name: self.name.clone(),
        }
    }

    pub fn area(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.board.get(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn create_all_pieces() -> Vec<Piece> {
    vec![
        Piece::new(&["1110", "1100"], "ZIPPO"),
        Piece::new(&["1100", "0110"], "Z"),
        Piece::new(&["0100", "1110"], "T"),
        Piece::new(&["1000", "1110"], "L"),
        Piece::new(&["1100", "0111"], "LONG_Z"),
        Piece::new(&["0100", "1111"], "LONG_T"),
        Piece::new(&["1110", "0000"], "SHORT_BAR"),
    ]
}

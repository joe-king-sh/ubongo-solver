use crate::{
    piece::Piece,
    utils::{bit_ops::BitBoard, types::Position},
};

/// ピースの配置情報を保持する構造体
#[derive(Debug, Clone)]
pub struct PlacedPiece {
    pub piece: Piece,
    pub variant: Piece, // 回転・裏返しを適用した後のピース
    pub position: Position,
}

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

    /// ボード上の1のビットの数（面積）を返します
    pub fn area(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.state.get(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    /// 各ピースを異なる記号で表示します
    pub fn display_with_pieces(&self, placed_pieces: &[PlacedPiece]) -> String {
        let symbols = ['■', '★', '●', '◆', '▲', '♦', '♠', '♥'];
        let mut board_symbols = vec![vec!['□'; self.width]; self.height];
        let mut piece_indices = vec![vec![None; self.width]; self.height];

        // 各ピースの位置を記録
        for (i, placed) in placed_pieces.iter().enumerate() {
            let piece_board = placed.variant.get_board();
            let (x, y) = placed.position;

            for dy in 0..placed.variant.get_height() {
                for dx in 0..placed.variant.get_width() {
                    if piece_board.get(dx, dy) {
                        piece_indices[y + dy][x + dx] = Some(i);
                    }
                }
            }
        }

        // 記録された位置に基づいて記号を配置
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(i) = piece_indices[y][x] {
                    board_symbols[y][x] = symbols[i % symbols.len()];
                }
            }
        }

        // 文字列に変換
        let mut result = String::new();
        for row in board_symbols {
            for symbol in row {
                result.push(symbol);
            }
            result.push('\n');
        }
        result
    }
}

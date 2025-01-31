use crate::{board::Board, piece::Piece, utils::types::Position};

#[derive(Debug)]
pub struct Solver {
    target_board: Board,
    pieces: Vec<Piece>,
    used_pieces: Vec<(Piece, Position)>,
    current_board: Board,
}

impl Solver {
    pub fn new(target_board: Board, pieces: Vec<Piece>) -> Self {
        let width = target_board.get_width();
        let height = target_board.get_height();
        Self {
            target_board,
            pieces,
            used_pieces: Vec::new(),
            current_board: Board::new(width, height),
        }
    }

    pub fn solve(&mut self) -> Result<bool, String> {
        if self.used_pieces.len() == self.pieces.len() {
            if self.current_board.get_state() == self.target_board.get_state() {
                return Ok(true);
            }
            return Ok(false);
        }

        let pieces = self.pieces.clone();
        for piece in pieces {
            if self.is_piece_used(&piece) {
                continue;
            }

            println!("ピース {} を試行中...", piece.get_name());
            for (variant_idx, variant) in piece.get_all_variants().iter().enumerate() {
                println!("  バリエーション {} を試行中...", variant_idx + 1);
                for y in 0..self.target_board.get_height() {
                    for x in 0..self.target_board.get_width() {
                        let pos = (x, y);
                        if self.can_place_piece_at_target(variant, pos) {
                            println!("    位置 ({}, {}) に配置を試行", x, y);
                            self.place_piece(variant, pos);
                            self.used_pieces.push((piece.clone(), pos));

                            if let Ok(true) = self.solve() {
                                return Ok(true);
                            }

                            self.remove_piece(variant, pos);
                            self.used_pieces.pop();
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    fn can_place_piece_at_target(&self, piece: &Piece, pos: Position) -> bool {
        let mut temp_board = self.current_board.clone();
        
        // 1. ピースがボードの範囲内に収まるか確認
        if !temp_board.can_place_piece(piece, pos) {
            return false;
        }
        
        // 2. ピースを配置
        temp_board.place_piece(piece, pos);
        
        // 3. 新しく配置されたピースのビットを取得
        let piece_bits = temp_board.get_state() ^ self.current_board.get_state();
        
        // 4. ピースが目標ボードの1の位置に収まっているか確認
        // piece_bitsの1のビットが全てtarget_boardの1のビットの位置にあることを確認
        if (piece_bits & self.target_board.get_state()) != piece_bits {
            return false;
        }
        
        // 5. 他のピースと重なっていないか確認
        if !(piece_bits & self.current_board.get_state()).not_any() {
            return false;
        }
        
        true
    }

    fn place_piece(&mut self, piece: &Piece, pos: Position) {
        self.current_board.place_piece(piece, pos);
    }

    fn remove_piece(&mut self, piece: &Piece, pos: Position) {
        self.current_board.remove_piece(piece, pos);
    }

    fn is_piece_used(&self, piece: &Piece) -> bool {
        self.used_pieces
            .iter()
            .any(|(p, _)| p.get_name() == piece.get_name())
    }

    pub fn get_solution(&self) -> &[(Piece, Position)] {
        &self.used_pieces
    }

    pub fn display_solution(&self) -> String {
        if self.used_pieces.is_empty() {
            return "解が見つかりませんでした。".to_string();
        }

        let mut result = String::new();
        result.push_str("解が見つかりました！\n\n");

        result.push_str("使用したピース：\n");

        for (i, (piece, pos)) in self.used_pieces.iter().enumerate() {
            result.push_str(&format!(
                "{}. {} at ({}, {})\n",
                i + 1,
                piece.get_name(),
                pos.0,
                pos.1
            ));
        }

        result.push_str("\n最終的な配置：\n");
        result.push_str(&self.current_board.display());
        result
    }
}

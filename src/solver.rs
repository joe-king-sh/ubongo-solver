use crate::{
    board::{Board, PlacedPiece},
    piece::Piece,
    utils::types::Position,
};

#[derive(Debug)]
pub struct Solver {
    target_board: Board,
    pieces: Vec<Piece>,
    used_pieces: Vec<PlacedPiece>,
    current_board: Board,
    solutions: Vec<Vec<PlacedPiece>>, // 全ての解を保存
    verbose: bool,                    // デバッグ出力を制御するフラグ
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
            solutions: Vec::new(),
            verbose: false,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn solve(&mut self) -> bool {
        if self.used_pieces.len() == self.pieces.len() {
            if self.current_board.get_state() == self.target_board.get_state() {
                // 解が見つかったら保存して探索を続ける
                self.solutions.push(self.used_pieces.clone());
                return false; // 探索を続けるためにfalseを返す
            }
            return false;
        }

        let pieces = self.pieces.clone();
        for piece in pieces {
            if self.is_piece_used(&piece) {
                continue;
            }

            if self.verbose {
                println!("ピース {} を試行中...", piece.get_name());
            }
            for (variant_idx, variant) in piece.get_all_variants().iter().enumerate() {
                if self.verbose {
                    println!("  バリエーション {} を試行中...", variant_idx + 1);
                }
                for y in 0..self.target_board.get_height() {
                    for x in 0..self.target_board.get_width() {
                        let pos = (x, y);
                        if self.can_place_piece_at_target(variant, pos) {
                            if self.verbose {
                                println!("    位置 ({}, {}) に配置を試行", x, y);
                            }
                            self.place_piece(variant, pos);
                            self.used_pieces.push(PlacedPiece {
                                piece: piece.clone(),
                                variant: variant.clone(),
                                position: pos,
                            });

                            self.solve(); // 結果に関わらず探索を続ける

                            self.remove_piece(variant, pos);
                            self.used_pieces.pop();
                        }
                    }
                }
            }
        }

        false
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
            .any(|p| p.piece.get_name() == piece.get_name())
    }

    pub fn get_solutions(&self) -> &[Vec<PlacedPiece>] {
        &self.solutions
    }

    pub fn display_all_solutions(&self) -> String {
        if self.solutions.is_empty() {
            return "解が見つかりませんでした。".to_string();
        }

        let mut result = format!("{}個の解が見つかりました！\n", self.solutions.len());

        for (solution_idx, solution) in self.solutions.iter().enumerate() {
            result.push_str(&format!("\n解 {}:\n", solution_idx + 1));
            result.push_str("使用したピース：\n");

            for (i, placed) in solution.iter().enumerate() {
                result.push_str(&format!(
                    "{}. {} at ({}, {})\n",
                    i + 1,
                    placed.piece.get_name(),
                    placed.position.0,
                    placed.position.1
                ));
            }

            result.push_str("\n配置：\n");
            let mut board = Board::new(
                self.target_board.get_width(),
                self.target_board.get_height(),
            );
            result.push_str(&board.display_with_pieces(solution));
            result.push('\n');
        }

        result
    }
}

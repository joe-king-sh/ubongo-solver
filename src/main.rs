use itertools::Itertools;
use ubongo_solver::{
    board::Board,
    piece::{create_all_pieces, Piece},
    solver::Solver,
    utils::{bit_ops::BitBoard, types::Position},
};

fn create_target_board(width: usize, height: usize, pattern: &[&str]) -> Board {
    let mut board = Board::new(width, height);

    for (y, row) in pattern.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '1' {
                board.place_piece(&Piece::new(&["1"], ""), (x, y));
            }
        }
    }
    board
}

fn main() {
    let pattern = ["10000", "11100", "11110", "11111"];

    let target = create_target_board(5, 4, &pattern);
    println!("問題：\n{}", target.display());

    let all_pieces = create_all_pieces();
    let target_area = target.area();

    // すべての3ピースの組み合わせを試す
    for pieces in all_pieces.into_iter().combinations(3) {
        let total_area: usize = pieces.iter().map(|p| p.area()).sum();
        if total_area != target_area {
            continue;
        }

        let mut solver = Solver::new(target.clone(), pieces.clone());
        match solver.solve() {
            Ok(_) => {
                if !solver.get_solutions().is_empty() {
                    println!("\n使用するピース:");
                    for piece in &pieces {
                        println!("- {} (面積: {})", piece.get_name(), piece.area());
                    }
                    println!("{}", solver.display_all_solutions());
                }
            }
            Err(e) => {
                println!("\nエラーが発生しました: {:?}", e);
            }
        }
    }

    println!("\n探索が完了しました。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_target_board() {
        let pattern = ["111", "101"];
        let board = create_target_board(3, 2, &pattern);
        assert_eq!(board.display(), "■■■\n■□■\n");
    }
}

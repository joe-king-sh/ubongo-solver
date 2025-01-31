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
    let pattern = [
        "10000", // ◾️◻︎◻︎◻︎◻︎
        "11111", // ◾️◾️★★★
        "11110", // ⚫︎◾️★★◻︎
        "11110", // ⚫︎⚫︎⚫︎⚫︎◻︎
    ];

    let target = create_target_board(5, 4, &pattern);
    println!("問題：\n{}", target.display());

    let all_pieces = create_all_pieces();
    let pieces = all_pieces
        .iter()
        .filter(|p| {
            let name = p.get_name();
            name == "L_WIDE" || name == "Z_SHAPE"
        })
        .cloned()
        .collect::<Vec<_>>();

    println!("\n使用するピース:");
    for piece in &pieces {
        println!("- {} (面積: {})", piece.get_name(), piece.area());
    }

    let mut solver = Solver::new(target, pieces);
    match solver.solve() {
        Ok(true) => {
            println!("\n{}", solver.display_solution());
        }
        Ok(false) => {
            println!("\n解が見つかりませんでした。");
        }
        Err(e) => {
            println!("\nエラーが発生しました: {:?}", e);
        }
    }
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

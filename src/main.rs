use itertools::Itertools;
use ubongo_solver::{
    board::Board,
    piece::{create_all_pieces, Piece},
    solver::Solver,
};

struct Problem {
    pattern: Vec<&'static str>,
    num_pieces: usize,
}

impl Problem {
    fn new(pattern: Vec<&'static str>, num_pieces: usize) -> Self {
        Self {
            pattern,
            num_pieces,
        }
    }
}

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

fn solve_problem(problem: &Problem) {
    let width = problem.pattern[0].len();
    let height = problem.pattern.len();
    let target = create_target_board(width, height, &problem.pattern);
    
    println!("問題：\n{}", target.display());
    println!("使用ピース数: {}", problem.num_pieces);

    let all_pieces = create_all_pieces();
    let target_area = target.area();

    for pieces in all_pieces.into_iter().combinations(problem.num_pieces) {
        let total_area: usize = pieces.iter().map(|p| p.area()).sum();
        if total_area != target_area {
            continue;
        }

        let mut solver = Solver::new(target.clone(), pieces.clone());
        solver.solve();
        if !solver.get_solutions().is_empty() {
            println!("\n使用するピース:");
            for piece in &pieces {
                println!("- {} (面積: {})", piece.get_name(), piece.area());
            }
            println!("{}", solver.display_all_solutions());
        }
    }
}

fn main() {
    let problems = vec![
        Problem {
            pattern: vec!["10000", "11100", "11110", "11111"],
            num_pieces: 3,
        },
        Problem {
            pattern: vec!["01111", "01111", "11111", "11111"],
            num_pieces: 4,
        },
    ];

    for (i, problem) in problems.iter().enumerate() {
        println!("\n問題 {}:", i + 1);
        solve_problem(problem);
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

    #[test]
    fn test_problem_creation() {
        let problem = Problem::new(vec!["111", "101"], 3);
        assert_eq!(problem.pattern, vec!["111", "101"]);
        assert_eq!(problem.num_pieces, 3);
    }
}

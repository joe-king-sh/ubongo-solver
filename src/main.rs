use itertools::Itertools;
use ubongo_solver::{
    board::{Board, PlacedPiece},
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

#[derive(Debug)]
struct Solution {
    pieces: Vec<Piece>,
    placed_pieces: Vec<PlacedPiece>,
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

fn solve_problem(problem: &Problem) -> Vec<Solution> {
    let width = problem.pattern[0].len();
    let height = problem.pattern.len();
    let target = create_target_board(width, height, &problem.pattern);
    
    println!("問題：\n{}", target.display());
    println!("使用ピース数: {}", problem.num_pieces);

    let all_pieces = create_all_pieces();
    let target_area = target.area();
    let mut all_solutions = Vec::new();

    for pieces in all_pieces.into_iter().combinations(problem.num_pieces) {
        let total_area: usize = pieces.iter().map(|p| p.area()).sum();
        if total_area != target_area {
            continue;
        }

        let mut solver = Solver::new(target.clone(), pieces.clone());
        solver.solve();
        if !solver.get_solutions().is_empty() {
            for solution in solver.get_solutions() {
                all_solutions.push(Solution {
                    pieces: pieces.clone(),
                    placed_pieces: solution.clone(),
                });
            }
        }
    }

    all_solutions
}

fn display_solutions(solutions: &[Solution], board_width: usize, board_height: usize) {
    if solutions.is_empty() {
        println!("解が見つかりませんでした。");
        return;
    }

    println!("\n合計{}個の解が見つかりました！", solutions.len());

    for (i, solution) in solutions.iter().enumerate() {
        println!("\n解 {}:", i + 1);
        println!("使用したピース：");
        for (j, placed) in solution.placed_pieces.iter().enumerate() {
            println!(
                "{}. {} at ({}, {})",
                j + 1,
                placed.piece.get_name(),
                placed.position.0,
                placed.position.1
            );
        }

        println!("\n配置：");
        let mut board = Board::new(board_width, board_height);
        println!("{}", board.display_with_pieces(&solution.placed_pieces));
    }
}

fn main() {
    let problems = vec![
        Problem {
            pattern: vec!["10000", "11100", "11110", "11111"],
            num_pieces: 3,
        },
    ];

    for (i, problem) in problems.iter().enumerate() {
        println!("\n問題 {}:", i + 1);
        let solutions = solve_problem(problem);
        let width = problem.pattern[0].len();
        let height = problem.pattern.len();
        display_solutions(&solutions, width, height);
    }

    println!("\n探索が完了しました。");
}

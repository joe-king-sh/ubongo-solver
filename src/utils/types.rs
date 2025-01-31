pub use super::bit_ops::BitBoard;

pub type Position = (usize, usize);

pub trait PositionExt {
    fn all_positions() -> Vec<Position>;
}

impl PositionExt for Position {
    fn all_positions() -> Vec<Position> {
        let mut positions = Vec::new();
        for y in 0..6 {
            for x in 0..6 {
                positions.push((x, y));
            }
        }
        positions
    }
}

#[derive(Debug)]
pub enum SolverError {
    OutOfBounds,
    Overlap,
    NoSolution,
}

impl std::fmt::Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverError::OutOfBounds => write!(f, "ピースが盤面からはみ出しています"),
            SolverError::Overlap => write!(f, "ピースが重なっています"),
            SolverError::NoSolution => write!(f, "解が見つかりませんでした"),
        }
    }
}

impl std::error::Error for SolverError {}

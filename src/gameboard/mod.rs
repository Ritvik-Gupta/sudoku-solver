pub mod backtracking_solver;

use crate::utils::Vec2D;

const BOARD_SIZE: usize = 9;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Given(u8),
    Guess(u8),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => " - ".to_owned(),
                Cell::Given(val) => format!("({})", val),
                Cell::Guess(val) => format!(" {} ", val),
            }
        )?;
        Ok(())
    }
}

impl Cell {
    fn value(&self) -> u8 {
        match self {
            Cell::Empty => 0,
            Cell::Given(num) | Cell::Guess(num) => *num,
        }
    }
}

#[derive(Default)]
pub struct GameBoard([[Cell; BOARD_SIZE]; BOARD_SIZE]);

impl std::ops::Index<Vec2D> for GameBoard {
    type Output = Cell;

    fn index(&self, idx: Vec2D) -> &Self::Output {
        &self.0[idx.0][idx.1]
    }
}

impl std::fmt::Debug for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.0.iter() {
            for cell in row.iter() {
                write!(f, "{:?} ", cell)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl GameBoard {
    fn new(board: [[u8; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        let mut gameboard = Self::default();
        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, &num) in row.iter().enumerate() {
                gameboard.0[row_idx][col_idx] = if num == 0 {
                    Cell::Empty
                } else {
                    Cell::Given(num)
                };
            }
        }
        gameboard
    }

    pub fn set_cell(&mut self, pos: Vec2D, num: u8) {
        self.0[pos.0][pos.1] = Cell::Given(num);
    }
}

#[cfg(test)]
mod test {
    use super::{GameBoard, BOARD_SIZE};
    use crate::utils::Vec2D;

    #[test]
    fn has_correct_output() {
        let mut gameboard = GameBoard::new([
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ]);

        gameboard.backtracking_solver();

        let solved_gameboard = GameBoard::new([
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ]);

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                assert_eq!(
                    gameboard[Vec2D(i, j)].value(),
                    solved_gameboard[Vec2D(i, j)].value()
                )
            }
        }
    }
}

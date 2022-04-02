use super::{Cell, GameBoard, BOARD_SIZE};
use crate::utils::Vec2D;

impl GameBoard {
    pub fn backtracking_solver(&mut self) -> bool {
        let mut first_empty_pos = None;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self[Vec2D(i, j)] == Cell::Empty {
                    first_empty_pos = Some(Vec2D(i, j));
                }
            }
        }

        match first_empty_pos {
            None => return true,
            Some(pos) => {
                for num in 1..=9 {
                    if self.is_valid_place(pos, num) {
                        self.0[pos.0][pos.1] = Cell::Guess(num);
                        if self.backtracking_solver() {
                            return true;
                        }
                        self.0[pos.0][pos.1] = Cell::Empty;
                    }
                }
            }
        }
        false
    }

    fn is_present_in_rows_or_cols(&self, Vec2D(row_idx, col_idx): Vec2D, num: u8) -> bool {
        for idx in 0..BOARD_SIZE {
            match self[Vec2D(row_idx, idx)] {
                Cell::Given(x) | Cell::Guess(x) if x == num => return true,
                _ => {}
            }
            match self[Vec2D(idx, col_idx)] {
                Cell::Given(x) | Cell::Guess(x) if x == num => return true,
                _ => {}
            }
        }
        false
    }

    fn is_present_in_box(&self, pos: Vec2D, num: u8) -> bool {
        let box_start_cell = Vec2D(3 * (pos.0 / 3), 3 * (pos.1 / 3));

        for row in 0..3 {
            for col in 0..3 {
                match self[Vec2D(box_start_cell.0 + row, box_start_cell.1 + col)] {
                    Cell::Given(x) | Cell::Guess(x) if x == num => return true,
                    _ => {}
                }
            }
        }
        false
    }

    fn is_valid_place(&self, pos: Vec2D, num: u8) -> bool {
        !self.is_present_in_rows_or_cols(pos, num) && !self.is_present_in_box(pos, num)
    }
}

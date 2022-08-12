use crate::utils::Vec2D;

pub mod wave_function_collapse;

#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Given(usize),
    Guess(usize),
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => format!("\x1b[91m{}\x1b[0m", "."),
                Cell::Given(val) => format!("\x1b[93m{}\x1b[0m", val),
                Cell::Guess(val) => format!("{}", val),
            }
        )?;
        Ok(())
    }
}

impl Cell {
    pub fn as_given(val: usize) -> Self {
        match val {
            0 => Self::Empty,
            _ => Self::Given(val),
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Given(val) | Self::Guess(val) => *val,
        }
    }
}

#[derive(Clone)]
pub struct GameBoard {
    board: Vec<Vec<Cell>>,
    pub box_size: usize,
    pub board_size: usize,
}

impl GameBoard {
    pub fn create_empty(box_size: usize) -> Self {
        let board_size = box_size * box_size;
        Self {
            board: vec![vec![Cell::Empty; board_size]; board_size],
            box_size,
            board_size,
        }
    }

    fn box_position(&self, cell_pos: Vec2D) -> Vec2D {
        (cell_pos / self.box_size) * self.box_size
    }

    fn box_cell_positions(&self, cell_pos: Vec2D) -> impl Iterator<Item = Vec2D> {
        let box_pos = self.box_position(cell_pos);
        let box_size = self.box_size;

        (0..box_size)
            .flat_map(move |x| (0..box_size).map(move |y| Vec2D::new(x, y)))
            .map(move |pos| pos + box_pos)
    }
}

impl std::ops::Index<Vec2D> for GameBoard {
    type Output = Cell;

    fn index(&self, idx: Vec2D) -> &Self::Output {
        &self.board[idx.x()][idx.y()]
    }
}

impl std::ops::IndexMut<Vec2D> for GameBoard {
    fn index_mut(&mut self, idx: Vec2D) -> &mut Self::Output {
        &mut self.board[idx.x()][idx.y()]
    }
}

impl std::fmt::Debug for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n")?;

        for (row_idx, row) in self.board.iter().enumerate() {
            if row_idx != 0 && row_idx % 3 == 0 {
                for _ in 0..11 {
                    write!(f, "\x1b[90m-\x1b[0m ")?;
                }
                write!(f, "\n")?;
            }
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx != 0 && col_idx % 3 == 0 {
                    write!(f, "\x1b[90m|\x1b[0m ")?;
                }
                write!(f, "{:?} ", cell)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

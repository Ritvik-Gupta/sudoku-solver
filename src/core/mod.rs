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
pub struct GameBoard<const BOX_SIZE: usize>(Vec<Vec<Cell>>);

impl<const BOX_SIZE: usize> GameBoard<BOX_SIZE> {
    const BOARD_SIZE: usize = BOX_SIZE * BOX_SIZE;

    pub fn create_empty() -> Self {
        Self(vec![vec![Cell::Empty; Self::BOARD_SIZE]; Self::BOARD_SIZE])
    }

    pub fn board_size(&self) -> usize {
        Self::BOARD_SIZE
    }

    fn box_position(&self, cell_pos: Vec2D) -> Vec2D {
        (cell_pos / BOX_SIZE) * BOX_SIZE
    }

    fn box_cell_positions(&self, cell_pos: Vec2D) -> impl Iterator<Item = Vec2D> {
        let box_pos = self.box_position(cell_pos);

        (0..BOX_SIZE)
            .flat_map(move |x| (0..BOX_SIZE).map(move |y| Vec2D::new(x, y)))
            .map(move |pos| pos + box_pos)
    }
}

impl<const BOX_SIZE: usize> std::ops::Index<Vec2D> for GameBoard<BOX_SIZE> {
    type Output = Cell;

    fn index(&self, idx: Vec2D) -> &Self::Output {
        &self.0[idx.x()][idx.y()]
    }
}

impl<const BOX_SIZE: usize> std::ops::IndexMut<Vec2D> for GameBoard<BOX_SIZE> {
    fn index_mut(&mut self, idx: Vec2D) -> &mut Self::Output {
        &mut self.0[idx.x()][idx.y()]
    }
}

impl<const BOX_SIZE: usize> std::fmt::Debug for GameBoard<BOX_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n")?;

        for (row_idx, row) in self.0.iter().enumerate() {
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

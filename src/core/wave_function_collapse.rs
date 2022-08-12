use super::{Cell, GameBoard};
use crate::utils::Vec2D;
use keyed_priority_queue::{Entry, KeyedPriorityQueue};

#[derive(Clone)]
pub struct CellTile(Vec<usize>);

impl std::ops::Deref for CellTile {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for CellTile {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }
}

impl Eq for CellTile {}

impl PartialOrd for CellTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.len().cmp(&self.0.len()))
    }
}

impl Ord for CellTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
pub struct WaveState {
    pub gameboard: GameBoard,
    pub entropy_queue: KeyedPriorityQueue<Vec2D, CellTile>,
}

impl WaveState {
    pub fn build(gameboard: GameBoard) -> Self {
        let mut non_collapsed_cells = KeyedPriorityQueue::new();
        for i in 0..gameboard.board_size {
            for j in 0..gameboard.board_size {
                if gameboard[Vec2D::new(i, j)] == Cell::Empty {
                    non_collapsed_cells.push(
                        Vec2D::new(i, j),
                        CellTile((1..=gameboard.board_size).collect()),
                    );
                }
            }
        }

        let mut simulation = Self {
            gameboard,
            entropy_queue: non_collapsed_cells,
        };

        for i in 0..simulation.gameboard.board_size {
            for j in 0..simulation.gameboard.board_size {
                if let Cell::Given(given_state) = simulation.gameboard[Vec2D::new(i, j)] {
                    simulation.apply_heuristics(Vec2D::new(i, j), given_state);
                }
            }
        }

        simulation
    }

    fn heuristics_on_cell(&mut self, pos: Vec2D, removing_tile: usize) {
        match self.entropy_queue.entry(pos) {
            Entry::Occupied(entry) => {
                let next_tiles = CellTile(
                    entry
                        .get_priority()
                        .iter()
                        .map(|&tile| tile)
                        .filter(|&tile| tile != removing_tile)
                        .collect(),
                );
                entry.set_priority(next_tiles);
            }
            Entry::Vacant(_) => {}
        }
    }

    fn apply_heuristics(&mut self, pos: Vec2D, given_tile: usize) {
        for i in 0..self.gameboard.board_size {
            self.heuristics_on_cell(Vec2D::new(i, pos.y()), given_tile);
            self.heuristics_on_cell(Vec2D::new(pos.x(), i), given_tile);
        }

        self.gameboard
            .box_cell_positions(pos)
            .for_each(|pos| self.heuristics_on_cell(pos, given_tile));
    }

    // fn recur(self) -> Option<Self> {
    //     loop {
    //         match self.entropy_queue.peek() {
    //             Some((_, min_entropy_tiles)) if min_entropy_tiles.is_empty() => return None,
    //             Some((&min_entropy_pos, min_entropy_tiles)) if min_entropy_tiles.len() > 1 => {
    //                 for &tile in min_entropy_tiles.0.iter() {
    //                     let mut cloned_state = self.clone();
    //                     cloned_state.apply_heuristics(min_entropy_pos, tile);
    //                 }
    //                 break;
    //             }
    //             Some((&min_entropy_pos, min_entropy_tiles)) => {}
    //             _ => {}
    //         }
    //     }
    //     Some(self)
    // }
}

pub struct WaveFunction {
    prev_frames: Vec<WaveState>,
    pub state: WaveState,
}

impl WaveFunction {
    pub fn build(gameboard: GameBoard) -> Self {
        Self {
            prev_frames: Vec::new(),
            state: WaveState::build(gameboard),
        }
    }

    // pub fn collapse_for(gameboard: GameBoard) -> Option<GameBoard> {
    //     WaveState::build(gameboard)
    //         .recur()
    //         .map(|state| state.gameboard)
    // }

    pub fn simulate_generation(&mut self) -> bool {
        match self.state.entropy_queue.peek() {
            Some((_, min_entropy_tiles)) if min_entropy_tiles.is_empty() => return false,
            Some((&min_entropy_pos, min_entropy_tiles)) => {
                if min_entropy_tiles.len() > 1 {
                    let mut cloned_state = self.state.clone();
                    cloned_state.apply_heuristics(min_entropy_pos, min_entropy_tiles[0]);
                    self.prev_frames.push(cloned_state);
                }

                let chosen_tile = min_entropy_tiles[0];
                self.state.gameboard[min_entropy_pos] = Cell::Guess(chosen_tile);

                self.state.entropy_queue.pop();
                self.state.apply_heuristics(min_entropy_pos, chosen_tile);
            }
            _ => {}
        }

        true
    }

    pub fn backtrack_prev_frame(&mut self) -> bool {
        match self.state.entropy_queue.peek() {
            Some((_, min_entropy_tiles)) if min_entropy_tiles.is_empty() => {
                self.state = self.prev_frames.pop().unwrap();
                true
            }
            _ => false,
        }
    }

    pub fn print(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;

        stdout.write(b"\x1B[2J\x1B[1;1H")?;
        stdout.write(format!("{:?}", self.state.gameboard).as_bytes())?;
        stdout.write(format!("{}\n", self.state.entropy_queue.len()).as_bytes())?;
        stdout.write(
            format!(
                "{:?}",
                self.state
                    .entropy_queue
                    .iter()
                    .map(|entry| entry.0)
                    .collect::<Vec<_>>()
            )
            .as_bytes(),
        )?;
        if let Some(prev_frame) = self.prev_frames.last() {
            stdout.write(b"\n\n")?;
            stdout.write(format!("{:?}", prev_frame.gameboard).as_bytes())?;
            stdout.write(format!("{}\n", prev_frame.entropy_queue.len()).as_bytes())?;
            stdout.write(
                format!(
                    "{:?}",
                    prev_frame
                        .entropy_queue
                        .iter()
                        .map(|entry| entry.0)
                        .collect::<Vec<_>>()
                )
                .as_bytes(),
            )?;
        }

        stdout.flush()?;
        Ok(())
    }
}

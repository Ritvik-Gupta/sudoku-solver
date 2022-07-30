use std::{io, thread::sleep, time::Duration};
use sudoku_solver::{
    core::{wave_function_collapse::WaveFunction, Cell, GameBoard},
    utils::Vec2D,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();

    let mut gameboard = GameBoard::create_empty(3);
    gameboard[Vec2D(0, 0)] = Cell::Given(1);
    gameboard[Vec2D(0, 1)] = Cell::Given(2);
    gameboard[Vec2D(0, 2)] = Cell::Given(3);
    gameboard[Vec2D(1, 1)] = Cell::Given(4);
    // gameboard[Vec2D(1, 2)] = Cell::Given(5);
    // gameboard[Vec2D(2, 0)] = Cell::Given(6);
    // gameboard[Vec2D(2, 1)] = Cell::Given(7);
    // gameboard[Vec2D(2, 2)] = Cell::Given(8);
    let mut wave_fn = WaveFunction::build(gameboard);

    wave_fn.print(&mut stdout)?;
    // sleep(Duration::from_secs(3));

    loop {
        if !wave_fn.simulate_generation() {
            wave_fn.backtrack_prev_frame();
        }

        wave_fn.print(&mut stdout)?;

        let _ = std::process::Command::new("cmd.exe")
            .arg("/c")
            .arg("pause")
            .status();
    }
}

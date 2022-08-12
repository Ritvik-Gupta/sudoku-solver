use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use sudoku_solver::{gameboard::GameBoard, utils::Vec2D};

fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout();

    let mut gameboard = GameBoard::default();

    gameboard.set_cell(Vec2D::new(1, 2), 5);

    stdout.write(b"\x1B[2J\x1B[1;1H")?;
    stdout.write(format!("{:?}", gameboard).as_bytes())?;
    stdout.flush()?;
    sleep(Duration::new(1, 0));

    gameboard.backtracking_solver();

    loop {
        stdout.write(b"\x1B[2J\x1B[1;1H")?;
        stdout.write(format!("{:?}", gameboard).as_bytes())?;
        stdout.flush()?;
        sleep(Duration::new(1, 0));
    }
}

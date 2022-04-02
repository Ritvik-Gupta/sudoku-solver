use sudoku_solver::{gameboard::GameBoard, utils::Vec2D};

fn main() {
    let mut gameboard = GameBoard::default();

    gameboard.set_cell(Vec2D(1, 2), 5);

    println!("{:?}", gameboard);
    gameboard.backtracking_solver();
    println!("{:?}", gameboard);
}

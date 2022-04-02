#![deny(missing_docs)]

//! A Sudoku game.

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

extern crate find_folder;
extern crate piston_window;

use piston_window::*;

pub use self::gameboard::Gameboard;
pub use self::gameboard_controller::GameboardController;
pub use self::gameboard_view::{GameboardView, GameboardViewSettings};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Sudoku", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_lazy(true);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    while let Some(e) = window.next() {
        gameboard_controller.event(
            gameboard_view.settings.position,
            gameboard_view.settings.size,
            &e,
        );

        window.draw_2d(&e, |c, g, device| {
            clear([0.9; 4], g);
            gameboard_view.draw(&gameboard_controller, &mut glyphs, &c, g);

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use sudoku_solver::{
    core::{wave_function_collapse::WaveFunction, Cell, GameBoard},
    utils::Vec2D,
};

static BOARD_ORDER: usize = 3;
static BOARD_ORDER_F32: f32 = BOARD_ORDER as f32;

static BOARD_SQ_ORDER: usize = BOARD_ORDER * BOARD_ORDER;

static TILE_SIZE: f32 = 25.0;
static CELL_SIZE: f32 = TILE_SIZE * BOARD_ORDER_F32;
static BOX_SIZE: f32 = CELL_SIZE * BOARD_ORDER_F32;

#[derive(Component, Deref, DerefMut)]
struct TilePos {
    pos: Vec2D,
}

#[derive(Component, Deref, DerefMut)]
struct CellPos {
    pos: Vec2D,
}

#[derive(Component, Deref, DerefMut)]
struct BoxPos {
    pos: Vec2D,
}

#[derive(Deref, DerefMut)]
struct ChosenBoxCell {
    pos: Vec2D,
}

enum SudokuState {
    Building(GameBoard),
    Simulating(WaveFunction),
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Sudoku Solver".to_string(),
            width: BOX_SIZE * BOARD_ORDER_F32,
            height: BOX_SIZE * BOARD_ORDER_F32,
            resizable: false,
            transparent: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ChosenBoxCell { pos: Vec2D(0, 0) })
        .insert_resource(SudokuState::Building(GameBoard::create_empty(BOARD_ORDER)))
        .add_startup_system(setup)
        .add_system_set(SystemSet::new().label("draw-board").with_system(draw_board))
        .add_system_set(
            SystemSet::new()
                .after("draw-board")
                .with_run_criteria(|sudoku: Res<SudokuState>| match sudoku.as_ref() {
                    SudokuState::Building(_) => ShouldRun::Yes,
                    _ => ShouldRun::No,
                })
                .with_system(keyboard_chosen_cell_update)
                .with_system(keyboard_cell_value_update)
                .with_system(keyboard_sudoku_state_update)
                .with_system(highlight_cell),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(|sudoku: Res<SudokuState>| match sudoku.as_ref() {
                    SudokuState::Simulating(_) => ShouldRun::Yes,
                    _ => ShouldRun::No,
                })
                .after("draw-board")
                .with_system(keyboard_simulate_wave_function),
        )
        .run();
}

fn highlight_cell(
    chosen_cell: Res<ChosenBoxCell>,
    query_box: Query<&BoxPos>,
    mut query_cell: Query<(&Parent, &mut Sprite, &CellPos)>,
) {
    for (parent, mut sprite, cell_pos) in query_cell.iter_mut() {
        let box_pos = query_box.get(parent.0).unwrap();

        if Vec2D(
            box_pos.0 * BOARD_ORDER + cell_pos.0,
            box_pos.1 * BOARD_ORDER + cell_pos.1,
        ) == chosen_cell.pos
        {
            let hsla = sprite.color.as_hlsa_f32();
            sprite.color = Color::Hsla {
                hue: hsla[0],
                saturation: hsla[1],
                lightness: hsla[2],
                alpha: hsla[3] - 0.2,
            };
        }
    }
}

fn draw_board(
    sudoku_state: Res<SudokuState>,
    query_box: Query<&BoxPos>,
    mut query_cell: Query<(&Parent, &mut Sprite, &CellPos)>,
    mut query_set: ParamSet<(
        Query<(&Parent, &mut Text, &CellPos)>,
        Query<(&Parent, &mut Text, &TilePos)>,
    )>,
) {
    match sudoku_state.as_ref() {
        SudokuState::Building(gameboard) => {
            let mut query_cell_text = query_set.p0();

            for (parent, mut text, cell_pos) in query_cell_text.iter_mut() {
                let (parent, mut sprite, _) = query_cell.get_mut(parent.0).unwrap();
                let box_pos = query_box.get(parent.0).unwrap();

                match gameboard[box_pos.pos * BOARD_ORDER + cell_pos.pos] {
                    Cell::Given(val) => {
                        sprite.color = Color::Hsla {
                            hue: 60.0,
                            saturation: 0.95,
                            lightness: 0.40,
                            alpha: 0.5,
                        };

                        text.sections[0].value = format!("{}", val);
                    }
                    _ => {
                        sprite.color = Color::Hsla {
                            hue: 120.0,
                            saturation: 0.95,
                            lightness: 0.40,
                            alpha: 0.5,
                        };
                    }
                }
            }
        }
        SudokuState::Simulating(wave_function) => {
            let mut query_cell_text = query_set.p0();

            for (parent, mut text, cell_pos) in query_cell_text.iter_mut() {
                let (parent, mut sprite, _) = query_cell.get_mut(parent.0).unwrap();
                let box_pos = query_box.get(parent.0).unwrap();

                match wave_function.state.gameboard[Vec2D(
                    box_pos.0 * BOARD_ORDER + cell_pos.0,
                    box_pos.1 * BOARD_ORDER + cell_pos.1,
                )] {
                    Cell::Given(val) => {
                        sprite.color = Color::Hsla {
                            hue: 60.0,
                            saturation: 0.95,
                            lightness: 0.40,
                            alpha: 0.5,
                        };

                        text.sections[0].value = format!("{}", val);
                    }
                    Cell::Guess(val) => {
                        sprite.color = Color::Hsla {
                            hue: 165.0,
                            saturation: 0.95,
                            lightness: 0.40,
                            alpha: 0.5,
                        };

                        text.sections[0].value = format!("{}", val);
                    }
                    Cell::Empty => {
                        sprite.color = Color::Hsla {
                            hue: 120.0,
                            saturation: 0.95,
                            lightness: 0.40,
                            alpha: 0.5,
                        };
                    }
                }
            }

            let mut query_tile_text = query_set.p1();

            for (parent, mut text, tile_pos) in query_tile_text.iter_mut() {
                let (parent, _, cell_pos) = query_cell.get_mut(parent.0).unwrap();
                let box_pos = query_box.get(parent.0).unwrap();

                text.sections[0].value = match wave_function
                    .state
                    .entropy_queue
                    .get_priority(&(box_pos.pos * BOARD_ORDER + cell_pos.pos))
                {
                    Some(cell_tile)
                        if cell_tile.contains(&(tile_pos.0 * BOARD_ORDER + tile_pos.1 + 1)) =>
                    {
                        format!("{}", tile_pos.0 * BOARD_ORDER + tile_pos.1 + 1)
                    }
                    _ => "".to_string(),
                };
            }
        }
    }
}

fn keyboard_simulate_wave_function(
    keys: Res<Input<KeyCode>>,
    mut sudoku_state: ResMut<SudokuState>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if let SudokuState::Simulating(wave_function) = sudoku_state.as_mut() {
            wave_function.simulate_generation();
        }
    }
}

fn keyboard_chosen_cell_update(
    keys: Res<Input<KeyCode>>,
    mut chosen_box_cell: ResMut<ChosenBoxCell>,
) {
    if keys.just_pressed(KeyCode::Left) {
        chosen_box_cell.0 = (chosen_box_cell.0 + BOARD_SQ_ORDER - 1) % BOARD_SQ_ORDER;
    }

    if keys.just_pressed(KeyCode::Right) {
        chosen_box_cell.0 = (chosen_box_cell.0 + 1) % BOARD_SQ_ORDER;
    }

    if keys.just_pressed(KeyCode::Down) {
        chosen_box_cell.1 = (chosen_box_cell.1 + BOARD_SQ_ORDER - 1) % BOARD_SQ_ORDER;
    }

    if keys.just_pressed(KeyCode::Up) {
        chosen_box_cell.1 = (chosen_box_cell.1 + 1) % BOARD_SQ_ORDER;
    }
}

fn keyboard_cell_value_update(
    mut char_evr: EventReader<ReceivedCharacter>,
    chosen_box_cell: Res<ChosenBoxCell>,
    mut sudoku_state: ResMut<SudokuState>,
) {
    if let SudokuState::Building(gameboard) = sudoku_state.as_mut() {
        if let Some(digit) = char_evr.iter().next().and_then(|ch| ch.char.to_digit(10)) {
            if (1..=9).contains(&digit) {
                gameboard[chosen_box_cell.pos] = Cell::Given(digit as usize);
            }
        }
    }
}

fn keyboard_sudoku_state_update(keys: Res<Input<KeyCode>>, mut sudoku_state: ResMut<SudokuState>) {
    if keys.just_pressed(KeyCode::Numlock) {
        if let SudokuState::Building(gameboard) = sudoku_state.as_mut() {
            *sudoku_state = SudokuState::Simulating(WaveFunction::build(gameboard.clone()));
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Regular.ttf");

    let board_iterations =
        (0..BOARD_ORDER).flat_map(|i| (0..BOARD_ORDER).map(move |j| (i as f32, j as f32)));

    board_iterations.clone().for_each(|(i, j)| {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::DARK_GRAY,
                    custom_size: Some(Vec2::new(BOX_SIZE, BOX_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        (i * BOX_SIZE) - BOX_SIZE * (BOARD_ORDER_F32 - 1.0) / 2.0,
                        (j * BOX_SIZE) - BOX_SIZE * (BOARD_ORDER_F32 - 1.0) / 2.0,
                        0.0,
                    ),
                    scale: Vec3::new(0.954, 0.954, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(BoxPos {
                pos: Vec2D(i as usize, j as usize),
            })
            .with_children(|parent| {
                board_iterations.clone().for_each(|(i, j)| {
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::Hsla {
                                    hue: 120.0,
                                    saturation: 1.0,
                                    lightness: 0.40,
                                    alpha: 0.5,
                                },
                                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    (i * CELL_SIZE) - CELL_SIZE * (BOARD_ORDER_F32 - 1.0) / 2.0,
                                    (j * CELL_SIZE) - CELL_SIZE * (BOARD_ORDER_F32 - 1.0) / 2.0,
                                    0.1,
                                ),
                                scale: Vec3::new(0.954, 0.954, 1.0),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(CellPos {
                            pos: Vec2D(i as usize, j as usize),
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(Text2dBundle {
                                    text: Text::with_section(
                                        "",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: CELL_SIZE,
                                            ..default()
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    transform: Transform {
                                        scale: Vec3::new(0.954, 0.954, 1.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(CellPos {
                                    pos: Vec2D(i as usize, j as usize),
                                });
                        })
                        .with_children(|parent| {
                            board_iterations.clone().for_each(|(i, j)| {
                                parent
                                    .spawn_bundle(Text2dBundle {
                                        text: Text::with_section(
                                            "",
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: TILE_SIZE,
                                                ..default()
                                            },
                                            TextAlignment {
                                                vertical: VerticalAlign::Center,
                                                horizontal: HorizontalAlign::Center,
                                            },
                                        ),
                                        transform: Transform {
                                            translation: Vec3::new(
                                                (i * TILE_SIZE)
                                                    - TILE_SIZE * (BOARD_ORDER_F32 - 1.0) / 2.0,
                                                (j * TILE_SIZE)
                                                    - TILE_SIZE * (BOARD_ORDER_F32 - 1.0) / 2.0,
                                                0.2,
                                            ),
                                            scale: Vec3::new(0.954, 0.954, 1.0),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .insert(TilePos {
                                        pos: Vec2D(i as usize, j as usize),
                                    });
                            });
                        });
                });
            });
    });
}

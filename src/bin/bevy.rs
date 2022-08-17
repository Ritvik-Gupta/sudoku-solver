use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::schedule::ShouldRun,
    prelude::*,
};
use sudoku_solver::{
    core::{wave_function_collapse::WaveFunction, Cell, GameBoard},
    utils::Vec2D,
};

const BOARD_ORDER: usize = 3;
const BOARD_ORDER_F32: f32 = BOARD_ORDER as f32;

const BOARD_SQ_ORDER: usize = BOARD_ORDER * BOARD_ORDER;

const TILE_SIZE: f32 = 20.0;
const CELL_SIZE: f32 = TILE_SIZE * BOARD_ORDER_F32;
const BOX_SIZE: f32 = CELL_SIZE * BOARD_ORDER_F32;

static GIVEN_CELL_COLOR: Color = Color::Hsla {
    hue: 60.0,
    saturation: 0.95,
    lightness: 0.40,
    alpha: 0.5,
};
static EMPTY_CELL_COLOR: Color = Color::Hsla {
    hue: 120.0,
    saturation: 0.95,
    lightness: 0.40,
    alpha: 0.5,
};
static GUESS_CELL_COLOR: Color = Color::Hsla {
    hue: 165.0,
    saturation: 0.95,
    lightness: 0.40,
    alpha: 0.5,
};

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

struct BuildingState {
    gameboard: GameBoard<BOARD_ORDER>,
    chosen_box_cell: Vec2D,
}

struct SimulatingState {
    wave_function: WaveFunction<BOARD_ORDER>,
}

struct InputTimerState {
    timer: Timer,
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
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(BuildingState {
            gameboard: GameBoard::create_empty(),
            chosen_box_cell: Vec2D::new(0, 0),
        })
        .add_startup_system(setup)
        .add_system_set(SystemSet::new().label("draw-board").with_system(draw_board))
        .add_system_set(
            SystemSet::new()
                .after("draw-board")
                .with_run_criteria(|sudoku_building: Option<Res<BuildingState>>| {
                    match sudoku_building.is_some() {
                        true => ShouldRun::Yes,
                        _ => ShouldRun::No,
                    }
                })
                .with_system(keyboard_chosen_cell_update)
                .with_system(keyboard_cell_value_update)
                .with_system(keyboard_sudoku_state_update),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(|sudoku_simulating: Option<Res<SimulatingState>>| {
                    match sudoku_simulating.is_some() {
                        true => ShouldRun::Yes,
                        _ => ShouldRun::No,
                    }
                })
                .after("draw-board")
                .with_system(keyboard_simulate_wave_function),
        )
        .run();
}

fn draw_board(
    sudoku_building_state: Option<Res<BuildingState>>,
    sudoku_simulating_state: Option<Res<SimulatingState>>,
    query_box: Query<&BoxPos>,
    mut query_cell: Query<(&Parent, &mut Sprite, &CellPos)>,
    mut query_set: ParamSet<(
        Query<(&Parent, &mut Text, &CellPos)>,
        Query<(&Parent, &mut Text, &TilePos)>,
    )>,
) {
    if let Some(sudoku_state) = sudoku_building_state {
        if !sudoku_state.is_changed() {
            return;
        }

        let gameboard = &sudoku_state.gameboard;
        let mut query_cell_text = query_set.p0();

        for (parent, mut text, cell_pos) in query_cell_text.iter_mut() {
            let (parent, mut sprite, _) = query_cell.get_mut(parent.0).unwrap();
            let box_pos = query_box.get(parent.0).unwrap();
            let pos = box_pos.pos * BOARD_ORDER + cell_pos.pos;

            let (color_val, text_val) = match gameboard[pos] {
                Cell::Given(val) => (GIVEN_CELL_COLOR, format!("{}", val)),
                _ => (EMPTY_CELL_COLOR, "".to_string()),
            };

            sprite.color = color_val;
            text.sections[0].value = text_val;

            if pos == sudoku_state.chosen_box_cell {
                let hsla = sprite.color.as_hlsa_f32();
                sprite.color = Color::Hsla {
                    hue: hsla[0],
                    saturation: hsla[1],
                    lightness: hsla[2],
                    alpha: hsla[3] - 0.2,
                };
            }
        }
    } else if let Some(sudoku_state) = sudoku_simulating_state {
        if !sudoku_state.is_changed() {
            return;
        }

        let wave_function = &sudoku_state.wave_function;
        let mut query_cell_text = query_set.p0();

        for (parent, mut text, cell_pos) in query_cell_text.iter_mut() {
            let (parent, mut sprite, _) = query_cell.get_mut(parent.0).unwrap();
            let box_pos = query_box.get(parent.0).unwrap();
            let pos = box_pos.pos * BOARD_ORDER + cell_pos.pos;

            let (color_val, text_val) = match wave_function.state.gameboard[pos] {
                Cell::Given(val) => (GIVEN_CELL_COLOR, format!("{}", val)),
                Cell::Guess(val) => (GUESS_CELL_COLOR, format!("{}", val)),
                Cell::Empty => (EMPTY_CELL_COLOR, "".to_string()),
            };

            sprite.color = color_val;
            text.sections[0].value = text_val;

            if let Some((&min_entropy_pos, _)) = wave_function.state.entropy_queue.peek() {
                if pos == min_entropy_pos {
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

        let mut query_tile_text = query_set.p1();

        for (parent, mut text, tile_pos) in query_tile_text.iter_mut() {
            let (parent, _, cell_pos) = query_cell.get_mut(parent.0).unwrap();
            let box_pos = query_box.get(parent.0).unwrap();
            let pos = box_pos.pos * BOARD_ORDER + cell_pos.pos;

            text.sections[0].value = match wave_function.state.entropy_queue.get_priority(&pos) {
                Some(cell_tile)
                    if cell_tile.contains(&(tile_pos.x() * BOARD_ORDER + tile_pos.y() + 1)) =>
                {
                    format!("{}", tile_pos.x() * BOARD_ORDER + tile_pos.y() + 1)
                }
                _ => "".to_string(),
            };
        }
    } else {
        unreachable!()
    }
}

fn keyboard_simulate_wave_function(
    keys: Res<Input<KeyCode>>,
    mut sudoku_state: ResMut<SimulatingState>,
    mut input_timer: ResMut<InputTimerState>,
    time: Res<Time>,
) {
    input_timer.timer.tick(time.delta());

    if input_timer.timer.finished() && keys.any_pressed([KeyCode::Space, KeyCode::Back]) {
        let wave_function = &mut sudoku_state.wave_function;

        if keys.pressed(KeyCode::Space) {
            if !wave_function.simulate_generation() {
                wave_function.backtrack_prev_frame();
            }
        } else if keys.pressed(KeyCode::Back) {
            wave_function.force_backtrack_prev_frame();
        }
    }
}

fn keyboard_chosen_cell_update(keys: Res<Input<KeyCode>>, mut sudoku_state: ResMut<BuildingState>) {
    if keys.any_just_pressed([KeyCode::Left, KeyCode::Right, KeyCode::Down, KeyCode::Up]) {
        let chosen_box_cell = &mut sudoku_state.chosen_box_cell;

        if keys.just_pressed(KeyCode::Left) {
            *chosen_box_cell.x_mut() = (chosen_box_cell.x() + BOARD_SQ_ORDER - 1) % BOARD_SQ_ORDER;
        } else if keys.just_pressed(KeyCode::Right) {
            *chosen_box_cell.x_mut() = (chosen_box_cell.x() + 1) % BOARD_SQ_ORDER;
        } else if keys.just_pressed(KeyCode::Down) {
            *chosen_box_cell.y_mut() = (chosen_box_cell.y() + BOARD_SQ_ORDER - 1) % BOARD_SQ_ORDER;
        } else if keys.just_pressed(KeyCode::Up) {
            *chosen_box_cell.y_mut() = (chosen_box_cell.y() + 1) % BOARD_SQ_ORDER;
        }
    }
}

fn keyboard_cell_value_update(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut sudoku_state: ResMut<BuildingState>,
) {
    let chosen_box_cell = sudoku_state.chosen_box_cell;

    if let Some(digit) = char_evr.iter().next().and_then(|ch| ch.char.to_digit(10)) {
        let value = sudoku_state.gameboard[chosen_box_cell].value() * 10 + digit as usize;
        if value <= BOARD_SQ_ORDER {
            sudoku_state.gameboard[chosen_box_cell] = Cell::as_given(value);
        }
    } else if keys.just_pressed(KeyCode::Back) {
        sudoku_state.gameboard[chosen_box_cell] =
            Cell::as_given(sudoku_state.gameboard[chosen_box_cell].value() / 10);
    }
}

fn keyboard_sudoku_state_update(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    sudoku_state: Res<BuildingState>,
) {
    if keys.just_pressed(KeyCode::Numlock) {
        let gameboard = sudoku_state.gameboard.clone();
        commands.remove_resource::<BuildingState>();

        commands.insert_resource(SimulatingState {
            wave_function: WaveFunction::build(gameboard),
        });
        commands.insert_resource(InputTimerState {
            timer: Timer::from_seconds(0.15, true),
        });
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
                pos: Vec2D::new(i as usize, j as usize),
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
                            pos: Vec2D::new(i as usize, j as usize),
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
                                    pos: Vec2D::new(i as usize, j as usize),
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
                                        pos: Vec2D::new(i as usize, j as usize),
                                    });
                            });
                        });
                });
            });
    });
}

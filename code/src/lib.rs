use bevy::audio::AudioSink;
use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;
use bevy::window::{WindowResizeConstraints};
use rand::Rng;
mod resizable;

#[cfg(target_arch = "wasm32")]
mod web_resizer;

#[cfg(target_arch = "wasm32")]
mod web_controls;

#[cfg(target_arch = "wasm32")]
mod web_audio;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

struct Cell {
    picked: bool,
    entity: Option<Entity>,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    move_cooldown: Timer,
}

#[derive(Default)]
struct Bonus {
    entity: Option<Entity>,
    i: usize,
    j: usize,
}

#[derive(Default)]
struct Game {
    board: Vec<Vec<Cell>>,
    picked_spaces: Vec<Vec<Cell>>,
    player: Player,
    bonus: Bonus,
    flowers_picked: u32,
    total_columns: usize,
    total_rows: usize,
    original_width: f32,
    original_height: f32,
    original_sprite_width: f32,
    original_sprite_height: f32,
}

fn get_original_size(min_width: f32, max_width: f32, min_height: f32, max_height: f32) -> (f32,f32)
{
    let mut width: f32 = 1200.0;
    let mut height: f32 = 600.0;

    #[cfg(target_arch = "wasm32")]
    web_resizer::web_size(&mut width, &mut height);

    width = if width < min_width { min_width } else { width };
    width = if width > max_width { max_width } else { width };

    //code for note keeping aspect ratio.
    height = if height < min_height {
        min_height
    } else {
        height
    };
    height = if height > max_height {
        max_height
    } else {
        height
    };

    (width, height)
}

pub fn play_game() {
    let mut app = App::new();
    
    all(&mut app);
    
    #[cfg(target_arch = "wasm32")]
    setup_web(&mut app);

    app.run();
}

fn all(app: &mut App)
{
    let min_width: f32 = 200.0;
    let min_height: f32 = 100.0;

    let max_width: f32 = 1200.0;
    let max_height: f32 = 600.0;

    let (width, height) = get_original_size(min_width, max_width, min_height, max_height);

    app
        .insert_resource(WindowDescriptor {
            width: width,
            height: height,
            resize_constraints: WindowResizeConstraints {
                min_width: min_width,
                min_height: min_height,
                max_width: max_width,
                max_height: max_height,
            },
            title: "Sunflower Picking".to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: false,
            ..Default::default()
        })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(audio)
        .add_state(GameState::Playing)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(initialize))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
        .add_system(bevy::window::close_on_esc);
}

#[cfg(target_arch = "wasm32")]
pub fn setup_web(app: &mut App) {
    app.add_system(web_resizer::resizer);

    app.add_system(web_audio::web_audio_control);

    web_controls::show_canvas();

    web_controls::toggle_loading();
}

//Note: This needs to be a startup system, or the MusiController resource can't be found by 
//      the web_audio query at runtime.
fn audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let music = asset_server.load("audio/FlowerCollecting.ogg");
        let handle =
            audio_sinks.get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP));
        commands.insert_resource(resizable::MusicController(handle));
}

fn initialize(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut game: ResMut<Game>,
) {
    if let Some(window) = windows.get_primary_mut() {
        //Initialize Game Object
        game.total_columns = 16;
        game.total_rows = 8;

        game.player.i = 0;
        game.player.j = 0;
        game.player.move_cooldown = Timer::from_seconds(0.3, false);

        game.bonus.i = game.total_columns / 2;
        game.bonus.j = game.total_rows / 2;

        game.flowers_picked = 0;
        
        game.original_width = window.width() as f32;
        game.original_height = window.height() as f32;
        game.original_sprite_width = window.width() / (game.total_columns as f32);
        game.original_sprite_height = window.height() / (game.total_rows as f32);

        commands.spawn_bundle(Camera2dBundle::default());

        game.board = (0..game.total_rows)
            .map(|j| {
                (0..game.total_columns)
                    .map(|i| {
                        let mut picked = false;
                        if i == game.bonus.i && j == game.bonus.j {
                            game.flowers_picked += 1;
                            picked = true;
                        }
                        let entity = Some(
                            commands
                                .spawn().insert_bundle(SpriteBundle {
                                    transform: Transform {
                                        translation: setup_game_translation(i as f32,j as f32, &game),
                                        ..Default::default()
                                    },
                                    sprite: Sprite {
                                        color: Color::rgb(0.0, 0.0, 0.0),
                                        custom_size: Some(Vec2::new(
                                            game.original_sprite_width,
                                            game.original_sprite_height,
                                        )),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }).insert(resizable::Resizable)
                                .id(),
                        );
                        Cell {
                            picked,
                            entity,
                        }
                    })
                    .collect()
            })
            .collect();

        game.player.entity = Some(
            commands
                .spawn().insert_bundle(SpriteBundle {
                    transform: Transform {
                        translation: setup_game_translation(game.player.i as f32,game.player.j as f32, &game),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(0.47, 0.52, 0.42),
                        custom_size: Some(Vec2::new(
                            game.original_sprite_width,
                            game.original_sprite_height,
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(resizable::Resizable)
                .id(),
        );

        game.picked_spaces = (0..game.total_rows)
            .map(|j| {
                (0..game.total_columns)
                    .map(|i| {
                        let mut new_red = 0.0;
                        let mut new_green = 0.36;
                        let mut new_blue = 0.73;

                        if j < game.total_rows / 2 {
                            new_red = 1.0;
                            new_green = 0.84;
                            new_blue = 0.0;
                        }

                        let mut visible = false;
                        if i == game.bonus.i && j == game.bonus.j {
                            visible = true;
                        }

                        let entity = Some(
                            commands
                                .spawn().insert_bundle(SpriteBundle {
                                    transform: Transform {
                                        translation: setup_game_translation(i as f32, j as f32, &game),
                                        ..Default::default()
                                    },
                                    sprite: Sprite {
                                        color: Color::rgb(new_red, new_green, new_blue),
                                        custom_size: Some(Vec2::new(
                                            game.original_sprite_width,
                                            game.original_sprite_height,
                                        )),
                                        ..Default::default()
                                    },
                                    visibility: Visibility {
                                        is_visible: visible,
                                    },
                                    ..Default::default()
                                }).insert(resizable::Resizable)
                                .id(),
                        );
                        Cell {
                            picked: false,
                            entity: entity,
                        }
                    })
                    .collect()
            })
            .collect();

        game.bonus.entity = Some(
            commands
                .spawn().insert_bundle(SpriteBundle {
                    transform: Transform {
                        translation: setup_game_translation(game.bonus.i as f32, game.bonus.j as f32, &game),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.68, 0.26),
                        custom_size: Some(Vec2::new(
                            game.original_sprite_width,
                            game.original_sprite_height,
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(resizable::Resizable)
                .id(),
        );
    }
}

fn setup_game_translation(initial_x: f32,initial_y: f32, game: &ResMut<Game>) -> Vec3
{
    Vec3::new(
     initial_x * game.original_sprite_width
            - game.original_width / 2.0
            + game.original_sprite_width / 2.0,
            initial_y as f32 * game.original_sprite_height
            - game.original_height / 2.0
            + game.original_sprite_height / 2.0,
        0.0,
    )
}

#[cfg(target_arch = "wasm32")]
fn check_web_move(web_up: &mut bool, web_down: &mut bool, web_right: &mut bool, web_left: &mut bool) {
    if web_controls::check_up() {
        *web_up = true;
    }

    if web_controls::check_down() {
        *web_down = true;
    }

    if web_controls::check_right() {
        *web_right = true;
    }

    if web_controls::check_left() {
        *web_left = true;
    }
}



//TODO: fix border issue on web where the block can't move to last row or column
fn move_player(
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    mut visiblities: Query<&mut Visibility>,
    time: Res<Time>,
) {
    if *state.current() != GameState::Playing {
        return;
    }
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        
        let mut web_up = false;
        let mut web_down = false;
        let mut web_right = false;
        let mut web_left = false;

        #[cfg(target_arch = "wasm32")]
        check_web_move(&mut web_up, &mut web_down, &mut web_right, &mut web_left);

        if keyboard_input.pressed(KeyCode::Right) || web_right {
            if game.player.i < game.total_columns - 1 {
                game.player.i += 1;
                moved = true;
            }
        }
        if keyboard_input.pressed(KeyCode::Left) || web_left {
            if game.player.i > 0 {
                game.player.i -= 1;
                moved = true;
            }
        }
        if keyboard_input.pressed(KeyCode::Up) || web_up {
            if game.player.j < game.total_rows - 1 {
                game.player.j += 1;
                moved = true;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) || web_down {
            if game.player.j > 0 {
                game.player.j -= 1;
                moved = true;
            }
        }

        if moved {
            game.player.move_cooldown.reset();
            if let Some(player_entity) = game.player.entity {
                if let Ok(mut player_transform) = transforms.get_mut(player_entity) {
                    player_transform.translation = setup_game_translation(game.player.i as f32, game.player.j as f32, &game) * player_transform.scale;
                }
            };
        }
    }

    if let Some(entity) = game.bonus.entity {
        if game.player.i == game.bonus.i && game.player.j == game.bonus.j {
            game.flowers_picked += 1;

            if game.flowers_picked
                > (game.total_rows
                    * game.total_columns) as u32
            {
                if let Some(entity_p) = game.player.entity {
                    commands.entity(entity_p).despawn_recursive();
                    game.player.entity = None;
                }

                let _ = state.overwrite_set(GameState::GameOver);
                return;
            } else {
                loop {
                    let new_i = rand::thread_rng()
                        .gen_range(0..game.total_columns);
                    let new_j = rand::thread_rng()
                        .gen_range(0..game.total_rows);

                    game.bonus.i = new_i;
                    game.bonus.j = new_j;
                    if game.board[game.bonus.j][game.bonus.i].picked {
                        continue;
                    }
                    if game.bonus.i != game.player.i || game.bonus.j != game.player.j {
                        game.board[new_j][new_i].picked = true;
                        break;
                    }
                }

                if let Some(entity) = game.picked_spaces[game.bonus.j][game.bonus.i].entity {
                    if let Ok(mut visiblity) = visiblities.get_mut(entity) {
                        visiblity.is_visible = true;
                    }
                }

                if let Ok(mut bonus_transform) = transforms.get_mut(entity) {
                    bonus_transform.translation = setup_game_translation(game.bonus.i as f32, game.bonus.j as f32, &game) * bonus_transform.scale;
                }
            }
        }
    }
}
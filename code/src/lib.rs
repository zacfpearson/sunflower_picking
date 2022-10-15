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

struct MusicController(Handle<AudioSink>);

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
        .add_state(GameState::Playing)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(initialize))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
        .add_system(bevy::window::close_on_esc);
}

#[cfg(target_arch = "wasm32")]
pub fn setup_web(app: &mut App) {
    app.add_system(web_resizer::resizer);

    web_controls::setup_web_controls();
}

fn initialize(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
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

                        let mut visible = false; //TODO: change to false
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

        let music = asset_server.load("audio/FlowerCollecting.ogg");
        let handle =
            audio_sinks.get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP));
        commands.insert_resource(MusicController(handle));
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

//todo: get rid of this possibly
#[cfg(target_arch = "wasm32")]
fn check_web_move(total_columns: usize, total_rows: usize, player: &mut Player, web_move: &mut bool) {
    if web_controls::check_right() {
        if player.i < total_columns - 1 {
            player.i += 1;
        }
        *web_move = true;
    }
    if web_controls::check_left() {
        if player.i > 0 {
            player.i -= 1;
        }
        *web_move = true;
    }
    if web_controls::check_up() {
        if player.j < total_rows - 1 {
            player.j += 1;
        }
        *web_move = true;
    }
    if web_controls::check_down() {
        if player.j > 0 {
            player.j -= 1;
        }
        *web_move = true;
    }
}

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
        let mut web_moved = false; //todo: change for all directions

        if keyboard_input.pressed(KeyCode::Right) {
            if game.player.i < game.total_columns - 1 {
                game.player.i += 1;
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if game.player.i > 0 {
                game.player.i -= 1;
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            if game.player.j < game.total_rows - 1 {
                game.player.j += 1;
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if game.player.j > 0 {
                game.player.j -= 1;
            }
            moved = true;
        }

        //check_web
        #[cfg(target_arch = "wasm32")]
        check_web_move(game.total_columns, game.total_rows, &mut game.player, &mut web_moved);

        if moved || web_moved {
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
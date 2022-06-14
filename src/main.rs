use bevy::audio::AudioSink;
use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Sunflower Picking".to_string(),
            width: 1200.,
            height: 600.,
            ..Default::default()
        })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

struct Cell {
    picked: bool,
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
    player: Player,
    bonus: Bonus,
    score: i32,
    flower_picked: u32,
}

const BOARD_SIZE_I: i32 = 1200;
const BOARD_SIZE_J: i32 = 600;

const SPRITE_SIZE_I: usize = 75;
const SPRITE_SIZE_J: usize = 75;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    game.flower_picked = 0;
    game.score = 0;
    game.player.i = 0;
    game.player.j = 0;
    game.player.move_cooldown = Timer::from_seconds(0.3, false);

    game.bonus.i = BOARD_SIZE_I as usize / SPRITE_SIZE_I / 2;
    game.bonus.j = BOARD_SIZE_J as usize / SPRITE_SIZE_J / 2;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    game.board = (0..BOARD_SIZE_J as usize / SPRITE_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I as usize / SPRITE_SIZE_I)
                .map(|i| {
                    let mut picked = false;
                    if i == game.bonus.i && j == game.bonus.j {
                        game.flower_picked += 1;
                        picked = true;
                    }
                    commands.spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                i as f32 * SPRITE_SIZE_I as f32 - BOARD_SIZE_I as f32 / 2.0
                                    + SPRITE_SIZE_I as f32 / 2.0,
                                j as f32 * SPRITE_SIZE_J as f32 - BOARD_SIZE_J as f32 / 2.0
                                    + SPRITE_SIZE_J as f32 / 2.0,
                                0.0,
                            ),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(0.0, 0.0, 0.0),
                            custom_size: Some(Vec2::new(
                                SPRITE_SIZE_I as f32,
                                SPRITE_SIZE_J as f32,
                            )),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    Cell { picked }
                })
                .collect()
        })
        .collect();

    game.player.entity = Some(
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.player.i as f32 * SPRITE_SIZE_I as f32 - BOARD_SIZE_I as f32 / 2.0
                            + SPRITE_SIZE_I as f32 / 2.0,
                        game.player.j as f32 * SPRITE_SIZE_J as f32 - BOARD_SIZE_J as f32 / 2.0
                            + SPRITE_SIZE_J as f32 / 2.0,
                        0.0,
                    ),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.47, 0.52, 0.42),
                    custom_size: Some(Vec2::new(SPRITE_SIZE_I as f32, SPRITE_SIZE_J as f32)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id(),
    );
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(
                game.bonus.i as f32 * SPRITE_SIZE_I as f32 - BOARD_SIZE_I as f32 / 2.0
                    + SPRITE_SIZE_I as f32 / 2.0,
                game.bonus.j as f32 * SPRITE_SIZE_J as f32 - BOARD_SIZE_J as f32 / 2.0
                    + SPRITE_SIZE_J as f32 / 2.0,
                0.0,
            ),

            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 0.36, 0.73),
            custom_size: Some(Vec2::new(SPRITE_SIZE_I as f32, SPRITE_SIZE_J as f32)),
            ..Default::default()
        },
        ..Default::default()
    });

    game.bonus.entity = Some(
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.bonus.i as f32 * SPRITE_SIZE_I as f32 - BOARD_SIZE_I as f32 / 2.0
                            + SPRITE_SIZE_I as f32 / 2.0,
                        game.bonus.j as f32 * SPRITE_SIZE_J as f32 - BOARD_SIZE_J as f32 / 2.0
                            + SPRITE_SIZE_J as f32 / 2.0,
                        0.0,
                    ),

                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.68, 0.26),
                    custom_size: Some(Vec2::new(SPRITE_SIZE_I as f32, SPRITE_SIZE_J as f32)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id(),
    );

    let music = asset_server.load("audio/FlowerCollecting.ogg");
    let handle = audio_sinks.get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP));
    commands.insert_resource(MusicController(handle));
}

struct MusicController(Handle<AudioSink>);

fn move_player(
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if *state.current() != GameState::Playing {
        return;
    }
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;

        if keyboard_input.pressed(KeyCode::Right) {
            if game.player.i < BOARD_SIZE_I as usize / SPRITE_SIZE_I - 1 {
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
            if game.player.j < BOARD_SIZE_J as usize / SPRITE_SIZE_I - 1 {
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

        if moved {
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    game.player.i as f32 * SPRITE_SIZE_I as f32 - BOARD_SIZE_I as f32 / 2.0
                        + SPRITE_SIZE_I as f32 / 2.0,
                    game.player.j as f32 * SPRITE_SIZE_J as f32 - BOARD_SIZE_J as f32 / 2.0
                        + SPRITE_SIZE_J as f32 / 2.0,
                    0.0,
                ),
                ..Default::default()
            };
        }
    }

    if let Some(entity) = game.bonus.entity {
        if game.player.i == game.bonus.i && game.player.j == game.bonus.j {
            game.score += 2;
            game.flower_picked += 1;
            commands.entity(entity).despawn_recursive();
            game.bonus.entity = None;

            if game.flower_picked
                > BOARD_SIZE_J as u32 / SPRITE_SIZE_I as u32 * BOARD_SIZE_I as u32
                    / SPRITE_SIZE_J as u32
            {
                if let Some(entity_p) = game.player.entity {
                    commands.entity(entity_p).despawn_recursive();
                    game.player.entity = None;
                }

                let _ = state.overwrite_set(GameState::GameOver);
                return;
            } else {
                loop {
                    let new_i =
                        rand::thread_rng().gen_range(0..BOARD_SIZE_I as usize / SPRITE_SIZE_I);
                    let new_j =
                        rand::thread_rng().gen_range(0..BOARD_SIZE_J as usize / SPRITE_SIZE_J);

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

                let mut new_red = 0.0;
                let mut new_green = 0.36;
                let mut new_blue = 0.73;

                if game.bonus.j < BOARD_SIZE_J as usize / SPRITE_SIZE_J / 2 {
                    new_red = 1.0;
                    new_green = 0.84;
                    new_blue = 0.0;
                }

                commands.spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            game.bonus.i as f32 * SPRITE_SIZE_I as f32 - BOARD_SIZE_I as f32 / 2.0
                                + SPRITE_SIZE_I as f32 / 2.0,
                            game.bonus.j as f32 * SPRITE_SIZE_J as f32 - BOARD_SIZE_J as f32 / 2.0
                                + SPRITE_SIZE_J as f32 / 2.0,
                            0.0,
                        ),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(new_red, new_green, new_blue),
                        custom_size: Some(Vec2::new(SPRITE_SIZE_I as f32, SPRITE_SIZE_J as f32)),
                        ..Default::default()
                    },
                    ..Default::default()
                });

                game.bonus.entity = Some(
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    game.bonus.i as f32 * SPRITE_SIZE_I as f32
                                        - BOARD_SIZE_I as f32 / 2.0
                                        + SPRITE_SIZE_I as f32 / 2.0,
                                    game.bonus.j as f32 * SPRITE_SIZE_J as f32
                                        - BOARD_SIZE_J as f32 / 2.0
                                        + SPRITE_SIZE_J as f32 / 2.0,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(1.0, 0.68, 0.26),
                                custom_size: Some(Vec2::new(
                                    SPRITE_SIZE_I as f32,
                                    SPRITE_SIZE_J as f32,
                                )),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .id(),
                );
            }
        }
    }
}

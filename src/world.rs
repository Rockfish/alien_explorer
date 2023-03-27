use std::f32::consts::PI;

use rand::Rng;
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct CakeSpawnTimer(pub Timer);

pub struct Cell {
    pub(crate) height: f32,
}

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub i: f32,
    pub j: f32,
    pub rotation: f32,
    pub move_cooldown: Timer,
}

#[derive(Default)]
pub struct cake {
    pub(crate) entity: Option<Entity>,
    pub(crate) i: f32,
    pub(crate) j: f32,
    pub(crate) handle: Handle<Scene>,
}

#[derive(Resource, Default)]
pub struct Game {
    pub board: Vec<Vec<Cell>>,
    pub player: Player,
    pub cake: cake,
    pub score: i32,
    pub cake_eaten: u32,
    pub camera_should_focus: Vec3,
    pub camera_is_focus: Vec3,
}

pub const BOARD_SIZE_I: f32 = 14.0;
pub const BOARD_SIZE_J: f32 = 21.0;

pub const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_I / 2.0,
    0.0,
    BOARD_SIZE_J / 2.0 - 0.5,
];


// control the game character
pub fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::Up) {
            if game.player.i < BOARD_SIZE_I - 1.0 {
                game.player.i += 0.25;
            }
            game.player.rotation = -PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if game.player.i > 0.0 {
                game.player.i -= 0.25;
            }
            game.player.rotation = PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if game.player.j < BOARD_SIZE_J - 1.0 {
                game.player.j += 0.25;
            }
            game.player.rotation = PI;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if game.player.j > 0.0 {
                game.player.j -= 0.25;
            }
            game.player.rotation = 0.0;
            moved = true;
        }

        // move on the board
        if moved {
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    game.player.i as f32,
                    game.board[game.player.j.round() as usize][game.player.i.round() as usize].height,
                    game.player.j as f32,
                ),
                rotation: Quat::from_rotation_y(game.player.rotation),
                ..default()
            };
        }
    }

    // eat the cake!
    if let Some(entity) = game.cake.entity {
        if game.player.i == game.cake.i && game.player.j == game.cake.j {
            game.score += 2;
            game.cake_eaten += 1;
            commands.entity(entity).despawn_recursive();
            game.cake.entity = None;
        }
    }
}

pub fn spawn_cake(
    time: Res<Time>,
    mut timer: ResMut<CakeSpawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
) {
    // make sure we wait enough time before spawning the next cake
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    info!("Spawning a new cake");

    if let Some(entity) = game.cake.entity {
        game.score -= 3;
        commands.entity(entity).despawn_recursive();
        game.cake.entity = None;
        // if game.score <= -5 {
        //     next_state.set(GameState::GameOver);
        //     return;
        // }
    }

    // ensure cake doesn't spawn on the player
    loop {
        game.cake.i = rand::thread_rng().gen_range(0..BOARD_SIZE_I.round() as usize) as f32;
        game.cake.j = rand::thread_rng().gen_range(0..BOARD_SIZE_J.round() as usize) as f32;
        if game.cake.i != game.player.i || game.cake.j != game.player.j {
            break;
        }
    }
    game.cake.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform::from_xyz(
                    game.cake.i,
                    game.board[game.cake.j.round() as usize][game.cake.i.round() as usize].height + 0.2,
                    game.cake.j,
                ),
                scene: game.cake.handle.clone(),
                ..default()
            })
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::rgb(1.0, 1.0, 0.0),
                        intensity: 1000.0,
                        range: 10.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..default()
                });
            })
            .id(),
    );
}

pub fn circling_cake (
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {

    let x = (BOARD_SIZE_I - 2.0) / 2.0;
    let y = (BOARD_SIZE_J - 2.0) / 2.0;

    game.cake.i = (time.elapsed_seconds() * 0.4).sin() * x + x + 1.0;
    game.cake.j = (time.elapsed_seconds() * 0.4).cos() * y + y + 1.0;


    *transforms.get_mut(game.cake.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            game.cake.i,
            0.4,
            game.cake.j,
        ),
        // rotation: Quat::from_rotation_y(rotation),
        ..default()
    };

}

// update the score displayed during the game
pub fn display_system(
    time: Res<Time>,
    game: Res<Game>,
    mut query: Query<&mut Text>
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("time: {}\nposition: {}, {}\nrotation: {}",
                                     time.elapsed_seconds(),
                                     game.player.i,
                                     game.player.j,
                                     game.player.rotation
    );
}
use std::f32::consts::PI;
use bevy::prelude::*;
use crate::game_state::*;


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
        // let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::Up) {
            if game.player.i < BOARD_SIZE_I - 1.0 {
                game.player.i += 0.25;
            }
            game.player.rotation = PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if game.player.i > 0.0 {
                game.player.i -= 0.25;
            }
            game.player.rotation = -PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if game.player.j < BOARD_SIZE_J - 1.0 {
                game.player.j += 0.25;
            }
            game.player.rotation = 0.0;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if game.player.j > 0.0 {
                game.player.j -= 0.25;
            }
            game.player.rotation = PI;
            moved = true;
        }

        // move on the board
        if moved {
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    game.player.i,
                    game.board[game.player.j.round() as usize][game.player.i.round() as usize].height,
                    game.player.j,
                ),
                rotation: Quat::from_rotation_y(game.player.rotation),
                ..default()
            };
        }
    }

    // eat the Cake!
    if let Some(entity) = game.cake.entity {
        if game.player.i == game.cake.i && game.player.j == game.cake.j {
            game.score += 2;
            game.cake_eaten += 1;
            commands.entity(entity).despawn_recursive();
            game.cake.entity = None;
        }
    }
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
pub fn update_display(
    time: Res<Time>,
    game: Res<Game>,
    mut query: Query<&mut Text>
) {
    // info!("Updating display");

    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("time: {}\nposition: {}, {}\nrotation: {}",
                                         time.elapsed_seconds(),
                                         game.player.i,
                                         game.player.j,
                                         game.player.rotation
        );
    }
}
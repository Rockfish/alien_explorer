use crate::game_state::*;
use bevy::prelude::*;
use std::f32::consts::PI;

// control the game character
pub fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms_query: Query<&mut Transform>,
    time: Res<Time>,
) {
    let move_step = 0.1;

    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        // let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::Up) {
            if game.player.i < BOARD_SIZE_I - 1.0 {
                game.player.i += move_step;
            }
            game.player.rotation = PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if game.player.i > 0.0 {
                game.player.i -= move_step;
            }
            game.player.rotation = -PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if game.player.j < BOARD_SIZE_J - 1.0 {
                game.player.j += move_step;
            }
            game.player.rotation = 0.0;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if game.player.j > 0.0 {
                game.player.j -= move_step;
            }
            game.player.rotation = PI;
            moved = true;
        }

        // move on the board
        if moved {
            game.player.move_cooldown.reset();

            let new_player_transform = Transform {
                translation: Vec3::new(
                    game.player.i,
                    game.board[game.player.j.round() as usize][game.player.i.round() as usize]
                        .height,
                    game.player.j,
                ),
                rotation: Quat::from_rotation_y(game.player.rotation),
                ..default()
            };

            // let player_entity = game.player.entity.unwrap();
            // *transforms_query.get_mut(player_entity).unwrap() = new_player_transform;
            // let mut player_transform = transforms_query.get_mut(player_entity).unwrap();
            // *player_transform = new_player_transform;

            if let Some(player_entity) = game.player.entity {
                if let Ok(mut player_transform) = transforms_query.get_mut(player_entity) {
                    *player_transform = new_player_transform;
                }
            }
        }
    }

    // eat the Cake!
    if let Some(cake_entity) = game.cake.entity {
        if game.player.i == game.cake.i && game.player.j == game.cake.j {
            game.score += 2;
            game.cake_eaten += 1;
            commands.entity(cake_entity).despawn_recursive();
            game.cake.entity = None;
        }
    }
}

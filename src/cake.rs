use std::f32::consts::PI;
use bevy::prelude::*;
use crate::game_state::*;

pub fn update_cake (
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
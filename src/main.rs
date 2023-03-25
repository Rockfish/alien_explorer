use bevy::prelude::*;
use crate::camera_pan_orbit::{pan_orbit_camera, spawn_camera};
use crate::lights::spawn_light;
use crate::lines::{LineMaterial, setup_cylinders, setup_lines};
use crate::world::{BonusSpawnTimer, Game, GameState, move_player, setup};

mod camera_pan_orbit;
mod lines;
mod cylinder;
mod scene_setup;
mod world;
mod lights;

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(BonusSpawnTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_state::<GameState>()
        .add_startup_system(setup_lines)
        .add_startup_system(setup_cylinders)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_light)
        .add_systems((
        //     setup_cameras.on_startup(),
            setup.in_schedule(OnEnter(GameState::Playing)),
        ))
        .add_systems(
            (
                move_player,
                pan_orbit_camera,
                // focus_camera,
                // rotate_bonus,
                // scoreboard_system,
                // spawn_bonus,
            )
                .in_set(OnUpdate(GameState::Playing)),
        )
        // .add_systems((
        //     teardown.in_schedule(OnExit(GameState::Playing)),
        //     display_score.in_schedule(OnEnter(GameState::GameOver)),
        //     gameover_keyboard.in_set(OnUpdate(GameState::GameOver)),
        //     teardown.in_schedule(OnExit(GameState::GameOver)),
        // ))
        .add_system(bevy::window::close_on_esc)
        .run();
}

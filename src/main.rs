use bevy::prelude::*;
use crate::camera_pan_orbit::*;
use crate::lights::spawn_light;
use crate::lines::*;
use crate::scene_setup::scene_setup;
use crate::world::*;

mod camera_pan_orbit;
mod lines;
mod cylinder;
mod scene_setup;
mod world;
mod lights;

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(CakeSpawnTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_state::<GameState>()
        .add_startup_systems((
            setup_lines,
            setup_cylinders,
            spawn_camera,
            // spawn_light
        ))
        .add_systems((
            //     setup_cameras.on_startup(),
            scene_setup.in_schedule(OnEnter(GameState::Playing)),
            display_system,
        ))
        .add_systems( (
                move_player,
                pan_orbit_camera,
                circling_cake,
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

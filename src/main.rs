use bevy::prelude::*;
use crate::camera_pan_orbit::*;
use crate::lines::*;
use crate::update::*;
use crate::spawn::*;
use crate::game_state::*;

mod camera_pan_orbit;
mod lines;
mod cylinder;
mod update;
mod game_state;
mod lights;
mod spawn;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        .init_resource::<Game>()
        .insert_resource(CakeSpawnTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_state::<GameState>()
        .add_systems(Startup, (
            spawn_lines,
            spawn_cylinders,
            spawn_camera,
            spawn_point_light,
            setup_game_state,
            spawn_game_board.after(setup_game_state),
            spawn_character.after(spawn_game_board),
            spawn_cake.after(spawn_game_board),
            spawn_scoreboard.after(setup_game_state),
        ))
        // .add_systems(OnEnter(GameState::Playing), (
        //         setup_cameras.on_startup(),
            // update_display,
        // ))
        .add_systems(Update, (
                move_player,
                pan_orbit_camera,
                circling_cake,
                update_display.after(move_player),
                // focus_camera,
                // rotate_bonus,
                // scoreboard_system,
                // spawn_bonus,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // .add_systems((
        //     teardown.in_schedule(OnExit(GameState::Playing)),
        //     display_score.in_schedule(OnEnter(GameState::GameOver)),
        //     gameover_keyboard.in_set(OnUpdate(GameState::GameOver)),
        //     teardown.in_schedule(OnExit(GameState::GameOver)),
        // ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

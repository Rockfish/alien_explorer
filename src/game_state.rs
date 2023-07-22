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
pub struct Cake {
    pub(crate) entity: Option<Entity>,
    pub(crate) i: f32,
    pub(crate) j: f32,
    pub(crate) handle: Handle<Scene>,
}

#[derive(Resource, Default)]
pub struct Game {
    pub board: Vec<Vec<Cell>>,
    pub player: Player,
    pub cake: Cake,
    pub score: i32,
    pub cake_eaten: u32,
    pub camera_should_focus: Vec3,
    pub camera_is_focus: Vec3,
}

pub const BOARD_SIZE_I: f32 = 14.0;
pub const BOARD_SIZE_J: f32 = 21.0;

// pub const RESET_FOCUS: [f32; 3] = [
//     BOARD_SIZE_I / 2.0,
//     0.0,
//     BOARD_SIZE_J / 2.0 - 0.5,
// ];

pub fn setup_game_state(mut game: ResMut<Game>) {
    info!("Setting up game state");

    // reset the game state
    game.cake_eaten = 0;
    game.score = 0;
    game.player.i = BOARD_SIZE_I / 2.0;
    game.player.j = BOARD_SIZE_J / 2.0;
    game.player.move_cooldown = Timer::from_seconds(0.01, TimerMode::Once);
}

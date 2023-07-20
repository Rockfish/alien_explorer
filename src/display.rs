use bevy::prelude::*;
use crate::game_state::*;


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
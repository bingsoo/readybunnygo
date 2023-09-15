pub mod background;
mod systems;

use background::BackgroundPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BackgroundPlugin);
    }
}

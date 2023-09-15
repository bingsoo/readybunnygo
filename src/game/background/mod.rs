use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}

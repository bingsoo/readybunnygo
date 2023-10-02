use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct BunnyPlugin;

impl Plugin for BunnyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_bunny)
            .add_systems(Update, update_bunny);
    }
}

use bevy::prelude::*;

mod systems;
use systems::*;

mod bullet;
use bullet::*;

pub struct BunnyPlugin;

impl Plugin for BunnyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_bunny)
            .add_systems(Update, update_bunny)
            .add_systems(Update, update_bullet);
    }
}

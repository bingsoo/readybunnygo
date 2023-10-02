use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct EnemyShip {
    pub enemy_type: EnemyType,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy)
            .add_systems(Update, update_enemy);
    }
}

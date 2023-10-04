use bevy::prelude::*;

mod systems;

use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy)
            .add_systems(Update, update_enemy);
    }
}

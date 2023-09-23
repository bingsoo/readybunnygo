pub mod background;
pub mod enemy;
mod systems;

use background::BackgroundPlugin;
use enemy::EnemyPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BackgroundPlugin).add_plugins(EnemyPlugin);
    }
}

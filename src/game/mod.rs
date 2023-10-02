pub mod background;
pub mod bunny;
pub mod enemy;
mod systems;

use background::BackgroundPlugin;
use bunny::BunnyPlugin;
use enemy::EnemyPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BackgroundPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(BunnyPlugin);
    }
}

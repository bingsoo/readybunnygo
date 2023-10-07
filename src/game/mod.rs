pub mod background;
pub mod bunny;
pub mod enemy;

use crate::prelude::*;

use background::BackgroundPlugin;
use bunny::BunnyPlugin;
use enemy::EnemyPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BackgroundPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(BunnyPlugin)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

use crate::prelude::*;

mod systems;
use systems::*;

mod tile;
pub use tile::*;

mod shared;
pub use shared::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_systems(Startup, spawn_background)
            .add_systems(Update, update_background)
            .add_systems(Update, tile::update_tiles)
            .add_systems(Update, update_user_input)
            .add_systems(Update, update_camera)
            .add_systems(Update, component_animator_system::<OrthographicProjection>);
    }
}

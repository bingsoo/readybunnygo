use crate::prelude::*;

mod systems;
use systems::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_systems(Startup, spawn_background)
            .add_systems(Update, update_background)
            .add_systems(Update, update_tiles)
            .add_systems(Update, speed_control)
            .add_systems(Update, update_camera)
            .add_systems(Update, component_animator_system::<OrthographicProjection>);
    }
}

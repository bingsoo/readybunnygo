#![warn(clippy::pedantic)]

mod game;
use game::GamePlugin;

// modules
mod components;

// systems and etc
mod prelude {
    pub use crate::components::*;
    pub use bevy::prelude::*;
    pub use bevy::window::{PresentMode, WindowTheme};
}

use prelude::*;

fn setup(mut commands: Commands) {
    let cam = Camera2dBundle::default();
    commands.spawn((GameCamera, cam));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ready Bunny Go!".into(),
                resolution: (1920., 1080.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_plugins(GamePlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

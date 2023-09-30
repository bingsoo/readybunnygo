use bevy::prelude::*;
use bevy_tweening::*;

pub mod components;
mod systems;

#[derive(Component)]
pub struct BackgroundPanel;

#[derive(Resource)]
pub struct GlobalData {
    pub current_pos_y: f32,
    pub move_y: f32,
    should_zoom: bool,
    speed: ScrollSpeed,
}

#[derive(Debug)]
pub enum ScrollSpeed {
    Speed0,
    Speed1,
    Speed2,
}

impl ScrollSpeed {
    // Helper function to increment the speed
    fn increment(&mut self) {
        *self = match *self {
            ScrollSpeed::Speed0 => ScrollSpeed::Speed1,
            ScrollSpeed::Speed1 => ScrollSpeed::Speed2,
            ScrollSpeed::Speed2 => ScrollSpeed::Speed2,
        };
    }

    // Helper function to decrement the speed
    fn decrement(&mut self) {
        *self = match *self {
            ScrollSpeed::Speed0 => ScrollSpeed::Speed0,
            ScrollSpeed::Speed1 => ScrollSpeed::Speed0,
            ScrollSpeed::Speed2 => ScrollSpeed::Speed1,
        };
    }

    fn get_zoom_scale(&self) -> f32 {
        match *self {
            ScrollSpeed::Speed0 => 0.95,
            ScrollSpeed::Speed1 => 1.0,
            ScrollSpeed::Speed2 => 1.1,
        }
    }

    fn get_scroll_speed(&self) -> f32 {
        match *self {
            ScrollSpeed::Speed0 => 0.5,
            ScrollSpeed::Speed1 => 3.0,
            ScrollSpeed::Speed2 => 8.0,
        }
    }
}

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

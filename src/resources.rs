use crate::prelude::*;

pub const DEFAULT_SIZE: f32 = 70.0;

#[derive(Resource)]
pub struct GlobalData {
    // background
    pub current_background_y: f32,
    pub total_move_distance: f32,
    pub speed: ScrollSpeed,

    // zoom
    pub should_zoom: bool,
    pub zoomed_in: bool,

    // dash
    pub is_dash_on: bool,
    pub dash_charging_time: f32,
    pub dash_bonus_speed: f32,
}

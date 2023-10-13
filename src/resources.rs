use crate::prelude::*;

pub const DEFAULT_SIZE: f32 = 70.0;

#[derive(Resource)]
pub struct GlobalData {
    pub current_background_y: f32,
    pub total_move_distance: f32,
    pub should_zoom: bool,
    pub speed: ScrollSpeed,
    pub is_dash_on: bool,
    pub dash_time: f32,
}

use crate::prelude::*;

pub const DEFAULT_SIZE: f32 = 70.0;

#[derive(Resource)]
pub struct GlobalData {
    pub current_pos_y: f32,
    pub move_y: f32,
    pub should_zoom: bool,
    pub speed: ScrollSpeed,
}

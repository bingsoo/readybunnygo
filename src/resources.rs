use crate::prelude::*;

#[derive(Resource)]
pub struct GlobalData {
    pub current_pos_y: f32,
    pub move_y: f32,
    pub should_zoom: bool,
    pub speed: ScrollSpeed,
}

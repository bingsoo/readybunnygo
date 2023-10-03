use crate::prelude::*;

#[derive(Resource)]
pub struct GlobalData {
    pub current_pos_y: f32,
    pub move_y: f32,
    pub should_zoom: bool,
    pub speed: ScrollSpeed,
}

#[derive(Debug)]
pub enum ScrollSpeed {
    Speed0,
    Speed1,
    Speed2,
}

impl ScrollSpeed {
    // Helper function to increment the speed
    pub fn increment(&mut self) {
        *self = match *self {
            ScrollSpeed::Speed0 => ScrollSpeed::Speed1,
            ScrollSpeed::Speed1 => ScrollSpeed::Speed2,
            ScrollSpeed::Speed2 => ScrollSpeed::Speed2,
        };
    }

    // Helper function to decrement the speed
    pub fn decrement(&mut self) {
        *self = match *self {
            ScrollSpeed::Speed0 => ScrollSpeed::Speed0,
            ScrollSpeed::Speed1 => ScrollSpeed::Speed0,
            ScrollSpeed::Speed2 => ScrollSpeed::Speed1,
        };
    }

    pub fn get_zoom_scale(&self) -> f32 {
        match *self {
            ScrollSpeed::Speed0 => 0.99,
            ScrollSpeed::Speed1 => 1.0,
            ScrollSpeed::Speed2 => 1.05,
        }
    }

    pub fn get_scroll_speed(&self) -> f32 {
        match *self {
            ScrollSpeed::Speed0 => 0.8,
            ScrollSpeed::Speed1 => 3.0,
            ScrollSpeed::Speed2 => 9.0,
        }
    }
}

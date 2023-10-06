use crate::prelude::*;

pub fn get_real_location(loc: &Vec3, global_data: &Res<GlobalData>) -> Vec3 {
    *loc - Vec3::new(00., global_data.move_y, 0.0)
}

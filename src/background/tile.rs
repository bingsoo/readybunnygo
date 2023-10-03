use crate::prelude::*;

pub fn get_tile_type(row_pos: i32) -> TileType {
    let mut rng = rand::thread_rng();
    let rand_number: usize = rng.gen_range(0..=500);
    let mut tile_type = TileType::Normal1;

    match row_pos {
        x if x == LAND_ROW_COUNT => return TileType::Tree1,
        x if x > LAND_ROW_COUNT => return tile_type,
        _ => {},
    }

    match rand_number {
        6..=200 => tile_type = TileType::Normal2,
        1 => tile_type = TileType::House1,
        2 => tile_type = TileType::House2,
        3 => tile_type = TileType::Tree1,
        4 => tile_type = TileType::Tree2,
        5 => tile_type = TileType::Tree3,
        _ => {},
    }

    tile_type
}

pub fn update_tiles(mut tile_position: Query<&mut Transform, With<Tile>>, global_data: Res<GlobalData>) {
    for mut transform in &mut tile_position {
        if transform.translation.y + global_data.current_pos_y < -860.0 {
            transform.translation.y += BG_CELL_SIZE * LAND_COL_COUNT as f32;
        }
    }
}

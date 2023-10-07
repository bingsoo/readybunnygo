pub const BG_CELL_SIZE: f32 = 40.0;
pub const LAND_COL_COUNT: i32 = 60;
pub const LAND_ROW_COUNT: i32 = 20;
pub const CAMERA_MOVE_TIME: u64 = 200;

pub enum TileType {
    Normal1,
    Normal2,
    House1,
    House2,
    Tree1,
    Tree2,
    Tree3,
}

impl TileType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TileType::Normal1 => "background/land1.png",
            TileType::Normal2 => "background/land2.png",
            TileType::House1 => "background/house1.png",
            TileType::House2 => "background/house2.png",
            TileType::Tree1 => "background/tree1.png",
            TileType::Tree2 => "background/tree2.png",
            TileType::Tree3 => "background/tree3.png",
        }
    }
}

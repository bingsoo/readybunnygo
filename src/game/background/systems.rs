use bevy::prelude::*;
use bevy::render::prelude::SpatialBundle;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const BG_CELL_SIZE: f32 = 40.0;
pub const LAND_ROW_COUNT: i32 = 20;

#[derive(Component)]
struct Parent;

enum TileType {
    Normal1,
    Normal2,
    House1,
    House2,
    Tree1,
    Tree2,
    Tree3,
}

impl TileType {
    fn as_str(&self) -> &'static str {
        match *self {
            TileType::Normal1 => "land1.png",
            TileType::Normal2 => "land2.png",
            TileType::House1 => "house1.png",
            TileType::House2 => "house2.png",
            TileType::Tree1 => "tree1.png",
            TileType::Tree2 => "tree2.png",
            TileType::Tree3 => "tree3.png",
        }
    }
}

fn add_bg_cell(commands: &mut Commands, asset_server: &Res<AssetServer>, loc: Vec3, parent: Entity, type_type: TileType) {
    let file_name = type_type.as_str();

    let sprite = commands
        .spawn(SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(BG_CELL_SIZE, BG_CELL_SIZE)), ..default() },
            texture: asset_server.load(file_name),
            transform: Transform::from_translation(loc),
            ..Default::default()
        })
        .id();

    commands.entity(parent).push_children(&[sprite]);
}

fn get_tile_type(row_pos: i32) -> TileType {
    let mut rng = rand::thread_rng();
    let rand_number: usize = rng.gen_range(0..=1000);
    let mut tile_type = TileType::Normal1;

    if row_pos == LAND_ROW_COUNT {
        return TileType::Tree1;
    } else if row_pos > LAND_ROW_COUNT {
        return tile_type;
    }

    match rand_number {
        6..=500 => tile_type = TileType::Normal2,
        1 => tile_type = TileType::House1,
        2 => tile_type = TileType::House2,
        3 => tile_type = TileType::Tree1,
        4 => tile_type = TileType::Tree2,
        5 => tile_type = TileType::Tree3,
        _ => {},
    }

    tile_type
}

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    println!("window size = {} / {} = {}", window.width(), BG_CELL_SIZE, window.width() / BG_CELL_SIZE);
    let row_side_count = (window.width() / BG_CELL_SIZE) as i32 / 2 + 1;
    let col_count = (window.height() / BG_CELL_SIZE) as i32 + 1;

    println!("spawn background");

    let parent = commands
        .spawn((
            SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(Vec3::new(0., window.height() * -1.0 * 0.5, 0.)),
                ..default()
            },
            Parent,
        ))
        .id();

    for i in 0..col_count {
        let h_pos = i as f32 * BG_CELL_SIZE;
        add_bg_cell(&mut commands, &asset_server, Vec3::new(0., h_pos, 0.), parent, get_tile_type(0));
        for j in 1..row_side_count {
            let loc_right = Vec3::new(j as f32 * BG_CELL_SIZE, h_pos, 0.);
            let loc_left = Vec3::new(-loc_right.x, loc_right.y, loc_right.z);
            add_bg_cell(&mut commands, &asset_server, loc_right, parent, get_tile_type(j));
            add_bg_cell(&mut commands, &asset_server, loc_left, parent, get_tile_type(j));
        }
    }
}

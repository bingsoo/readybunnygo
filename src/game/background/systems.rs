use bevy::prelude::*;
use bevy::render::prelude::SpatialBundle;
use bevy::window::PrimaryWindow;
//use rand::prelude::*;

pub const DEFAULT_DISTANCE: usize = 1000;
pub const BG_CELL_SIZE: f32 = 50.0;

#[derive(Component)]
struct Parent;

pub fn add_bg_cell(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    loc: Vec3,
    parent: Entity,
    right_edge: bool,
    left_edge: bool,
) {
    let mut file_name = "bg2.png";
    if left_edge {
        file_name = "bg1.png";
    } else if right_edge {
        file_name = "bg3.png";
    }

    let sprite = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(BG_CELL_SIZE, BG_CELL_SIZE)),
                ..default()
            },
            texture: asset_server.load(file_name),
            transform: Transform::from_translation(loc),
            ..Default::default()
        })
        .id();

    commands.entity(parent).push_children(&[sprite]);
}

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    println!("spawn background");
    let parent = commands
        .spawn((
            SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(Vec3::new(
                    0.,
                    window.height() * -1.0 * 0.5,
                    0.,
                )),
                ..default()
            },
            Parent,
        ))
        .id();

    let row_count = 14;
    for i in 0..DEFAULT_DISTANCE {
        let h_pos = i as f32 * BG_CELL_SIZE;

        add_bg_cell(
            &mut commands,
            &asset_server,
            Vec3::new(0., h_pos, 0.),
            parent,
            false,
            false,
        );

        for j in 1..row_count {
            let loc = Vec3::new(j as f32 * BG_CELL_SIZE, h_pos, 0.);
            add_bg_cell(
                &mut commands,
                &asset_server,
                loc,
                parent,
                j == row_count - 1,
                false,
            );
        }

        for j in 1..row_count {
            let loc = Vec3::new(j as f32 * BG_CELL_SIZE * -1., h_pos, 0.);
            add_bg_cell(
                &mut commands,
                &asset_server,
                loc,
                parent,
                false,
                j == row_count - 1,
            );
        }
    }
}

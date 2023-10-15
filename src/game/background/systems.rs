use crate::prelude::*;

use bevy::render::prelude::SpatialBundle;
use bevy::window::PrimaryWindow;
use std::time::Duration;

use super::shared::*;
use super::tile::get_tile_type;

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    println!(
        "window size = {} / {} = {}",
        window.width(),
        BG_CELL_SIZE,
        window.width() / BG_CELL_SIZE
    );
    let row_side_count = (window.width() / BG_CELL_SIZE) as i32 / 2 + 1 + 13;
    let col_count = LAND_COL_COUNT; //(window.height() / BG_CELL_SIZE) as i32 + 1;
    let begin_y = window.height() * -1.0 * 0.5;

    println!("spawn background");

    let parent = commands
        .spawn(SpatialBundle {
            visibility: Visibility::Visible,
            transform: Transform::from_translation(Vec3::new(0., begin_y, 0.)),
            ..default()
        })
        .insert(BackgroundPanel)
        .id();

    for i in 0..col_count {
        let h_pos = i as f32 * BG_CELL_SIZE;
        add_bg_cell(
            &mut commands,
            &asset_server,
            Vec3::new(0., h_pos, 0.),
            parent,
            get_tile_type(0),
        );
        for j in 1..row_side_count {
            let loc_right = Vec3::new(j as f32 * BG_CELL_SIZE, h_pos, 0.);
            let loc_left = Vec3::new(-loc_right.x, loc_right.y, loc_right.z);
            add_bg_cell(&mut commands, &asset_server, loc_right, parent, get_tile_type(j));
            add_bg_cell(&mut commands, &asset_server, loc_left, parent, get_tile_type(j));
        }
    }

    commands.insert_resource(GlobalData {
        current_background_y: begin_y,
        total_move_distance: 0.0,
        should_zoom: false,
        speed: ScrollSpeed::Speed1,
        is_dash_on: false,
        dash_time: 0.0,
        zoomed_in: true,
    });
    println!("starting pos y = {}", begin_y);
}

fn add_bg_cell(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    loc: Vec3,
    parent: Entity,
    type_type: TileType,
) {
    let file_name = type_type.as_str();
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
        .insert(BackgroundTile)
        .id();

    commands.entity(parent).push_children(&[sprite]);
}

pub fn update_camera(
    q: Query<(Entity, &OrthographicProjection), With<GameCamera>>,
    mut global_data: ResMut<GlobalData>,
    mut commands: Commands,
) {
    let (id, projection) = q.single();
    if global_data.should_zoom {
        global_data.should_zoom = false;
        println!("camera scale = {}", projection.scale);

        let zoom_tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(CAMERA_MOVE_TIME),
            TransformProjectionLens {
                start: projection.scale,
                end: global_data.speed.get_zoom_scale(),
            },
        );
        commands.entity(id).insert(Animator::new(zoom_tween));
        println!(
            "camera scale = {} -> {}",
            projection.scale,
            global_data.speed.get_zoom_scale()
        );
    }
}

pub fn update_background(
    mut background_query: Query<&mut Transform, With<BackgroundPanel>>,
    mut global_data: ResMut<GlobalData>,
) {
    let mut transform = background_query.single_mut();
    let mut current_speed = global_data.speed.get_scroll_speed();
    if global_data.is_dash_on {
        current_speed += DASH_SPEED;
    }

    transform.translation.y -= current_speed;
    global_data.current_background_y = transform.translation.y;
    global_data.total_move_distance += current_speed;
}

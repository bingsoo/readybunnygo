use crate::prelude::*;
use bevy::app::AppExit;

use super::bullet;

pub const BUNNY_SPEED: f32 = 550.0;
//pub const PRELOAD_BULLET_COUNT: usize = 500;

pub fn spawn_bunny(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("spawn bunny !");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..default()
            },
            texture: asset_server.load("ships/player_001.png"),
            transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 9.0 }),
            ..Default::default()
        })
        .insert(Bunny)
        .insert(BulletTimer(Timer::from_seconds(2.5, TimerMode::Repeating)));

    // TODO : preload bullets (object pooling)
    // for _ in 0..PRELOAD_BULLET_COUNT {
    //     pre_spawn_bullets(&mut commands, &asset_server);
    // }
}

pub fn update_bunny(
    mut query: Query<(&mut Transform, &mut BulletTimer), With<Bunny>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_data: ResMut<GlobalData>,
    mut exit: EventWriter<AppExit>,
) {
    let mut translation = Vec3::ZERO;
    let current_speed = BUNNY_SPEED;

    if global_data.is_dash_on {
        global_data.dash_time -= time.delta_seconds();
        if global_data.dash_time < 0.0 {
            global_data.is_dash_on = false;
        }
    }

    if let Ok((mut transform, _)) = query.get_single_mut() {
        for key in keyboard_input.get_pressed() {
            match key {
                KeyCode::A => translation.x -= current_speed * time.delta_seconds(),
                KeyCode::D => translation.x += current_speed * time.delta_seconds(),
                KeyCode::W => translation.y += current_speed * time.delta_seconds(),
                KeyCode::S => translation.y -= current_speed * time.delta_seconds(),
                KeyCode::Escape => exit.send(AppExit),
                _ => {},
            }
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            bullet::spawn_bullet(&mut commands, &asset_server, &transform)
        }

        if keyboard_input.just_pressed(KeyCode::ShiftRight) {
            println!("shift right pressed");
            global_data.is_dash_on = true;
            global_data.dash_time = DASH_TIME;
        }

        transform.translation += translation;
        // if timer.0.tick(time.delta()).just_finished() {
        //     bullet::spawn_bullet(&mut commands, &asset_server, &transform);
        // }

        if global_data.dash_time > 0.0 {
            println!("dash time = {}", global_data.dash_time);
        }
    }
}

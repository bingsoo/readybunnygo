use crate::prelude::*;
use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;

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
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut BulletTimer), With<Bunny>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut global_data: ResMut<GlobalData>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut translation = Vec3::ZERO;
    let current_speed = BUNNY_SPEED;

    if let Ok((mut transform, _)) = query.get_single_mut() {
        // bullet fire
        if keyboard_input.just_pressed(KeyCode::Space) {
            bullet::spawn_bullet(&mut commands, &asset_server, &transform);
        }

        // basic movement
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

        let test_charge_time = 0.7;

        // dash
        for key in mouse_button_input.get_pressed() {
            if *key == MouseButton::Right && global_data.dash_charging_time >= test_charge_time {
                global_data.is_dash_on = true;
                global_data.dash_charging_time = 0.0;
            } else if !global_data.is_dash_on {
                global_data.dash_charging_time += time.delta_seconds();
            }
        }

        if mouse_button_input.just_released(MouseButton::Right) {
            global_data.is_dash_on = false;
            global_data.dash_charging_time = 0.0;
        }

        // wheel zoom
        for ev in scroll_evr.iter() {
            if ev.y > 0.0 && !global_data.zoomed_in {
                global_data.speed.increment();
                global_data.should_zoom = true;
                global_data.zoomed_in = true;
                println!("zoom in");
            } else if ev.y < 0.0 && global_data.zoomed_in {
                global_data.speed.decrement();
                global_data.should_zoom = true;
                global_data.zoomed_in = false;
                println!("zoom out");
            }
        }

        transform.translation += translation;
    }

    let bonus_delta = 0.5;
    if global_data.is_dash_on && global_data.dash_bonus_speed < DASH_SPEED {
        global_data.dash_bonus_speed += bonus_delta * 2.0;
        global_data.dash_bonus_speed = f32::min(DASH_SPEED, global_data.dash_bonus_speed);
    } else if !global_data.is_dash_on && global_data.dash_bonus_speed > 0.0 {
        global_data.dash_bonus_speed -= bonus_delta;
        global_data.dash_bonus_speed = f32::max(0.0, global_data.dash_bonus_speed);
    }
}

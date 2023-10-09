use crate::prelude::*;
use bevy::app::AppExit;

pub const BUNNY_SPEED: f32 = 550.0;
pub const PRELOAD_BULLET_COUNT: usize = 500;

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
        .insert(Bunny);

    for _ in 0..PRELOAD_BULLET_COUNT {
        pre_spawn_bullets(&mut commands, &asset_server);
    }
}

pub fn update_bunny(
    mut query: Query<&mut Transform, With<Bunny>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    let mut translation = Vec3::ZERO;
    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::A => translation.x -= BUNNY_SPEED * time.delta_seconds(),
            KeyCode::D => translation.x += BUNNY_SPEED * time.delta_seconds(),
            KeyCode::W => translation.y += BUNNY_SPEED * time.delta_seconds(),
            KeyCode::S => translation.y -= BUNNY_SPEED * time.delta_seconds(),
            KeyCode::Escape => exit.send(AppExit),
            _ => {},
        }
    }
    if let Ok(mut transform) = query.get_single_mut() {
        transform.translation += translation;
    }
}

fn pre_spawn_bullets(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            texture: asset_server.load("bullets/bullet.png"),
            transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 9.0 }),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        BulletObject,
    ));
}

use crate::prelude::*;

pub const BUNNY_SPEED: f32 = 550.0;

pub fn spawn_bunny(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("spawn bunny !");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..default()
            },
            texture: asset_server.load("ships/player_001.png"),
            transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 9.0 }),
            ..Default::default()
        },
        Bunny,
    ));
}

pub fn update_bunny(mut query: Query<&mut Transform, With<Bunny>>, keyboard_input: Res<Input<KeyCode>>, time: Res<Time>) {
    if let Ok(mut transform) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= BUNNY_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += BUNNY_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += BUNNY_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= BUNNY_SPEED * time.delta_seconds();
        }
    }
}

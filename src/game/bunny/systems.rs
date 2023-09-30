use bevy::prelude::*;

use super::components::Bunny;

pub fn spawn_bunny(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("spawn bunny !");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(70.0, 70.0)), ..default() },
            texture: asset_server.load("ships/player_001.png"),
            transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 9.0 }),
            ..Default::default()
        },
        Bunny,
    ));
}

pub fn update_bunny(mut query: Query<&mut Transform, With<Bunny>>, keyboard_input: Res<Input<KeyCode>>, time: Res<Time>) {
    //for (mut bunny, mut transform) in query.iter_mut() {}

    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            println!("update bunny !");
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        println!("direction {:?}", direction);
        transform.translation += direction * 500.0 * time.delta_seconds();
        println!("transform {:?}", transform.translation);
    }
}

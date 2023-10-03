use bevy::prelude::*;

use crate::GameCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera { ..default() },
            ..default()
        },
        GameCamera,
    ));
}

use crate::prelude::*;

pub fn spawn_bullet(commands: &mut Commands, asset_server: &Res<AssetServer>, newtrans: &Transform) {
    let mut transform = newtrans.clone();
    transform.translation.z = 999.0;
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(DEFAULT_SIZE * 0.7, DEFAULT_SIZE * 0.7)),
                ..default()
            },
            texture: asset_server.load("bullets/bullet.png"),
            transform,
            visibility: Visibility::Visible,
            ..Default::default()
        })
        .insert(BulletObject);
}

pub fn update_bullet(mut bullet_query: Query<&mut Transform, With<BulletObject>>) {
    for mut transform in bullet_query.iter_mut() {
        transform.translation.y += 10.0;
    }
}

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

pub fn update_bullet(mut commands: Commands, mut bullet_query: Query<(Entity, &mut Transform), With<BulletObject>>) {
    for (entity, mut transform) in bullet_query.iter_mut() {
        transform.translation.y += 10.0;

        // for enemy_transform in enemy_query.iter() {
        //     if transform.translation.distance(enemy_transform.translation) < 50.0 {
        //         //transform.translation.y = 9999.0;
        //         println!("!!!");
        //     }
        // }

        if transform.translation.y > 2000.0 {
            commands.entity(entity).despawn();
            println!("despawn bullet");
        }
    }
}

pub fn update_bullet_hit(
    bullet_query: Query<&Transform, With<BulletObject>>,
    enemy_query: Query<&Transform, With<EnemyShip>>,
    global_data: Res<GlobalData>,
) {
    for bullet_transform in bullet_query.iter() {
        for enemy_transform in enemy_query.iter() {
            let loc = get_real_location(&enemy_transform.translation, &global_data);

            println!("bullet loc = {:?}", bullet_transform.translation);
            println!("enemy loc = {:?}", enemy_transform.translation);
            if bullet_transform.translation.distance(loc) < 50.0 {
                println!("!!!");
            }
        }
    }
}

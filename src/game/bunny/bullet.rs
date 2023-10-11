use crate::prelude::*;
use std::collections::HashSet;

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

        if transform.translation.y > 2000.0 {
            commands.entity(entity).despawn();
            println!("despawn bullet");
        }
    }
}

pub fn update_bullet_hit(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<BulletObject>>,
    enemy_query: Query<(Entity, &Transform), With<EnemyShip>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        if despawned_entities.contains(&enemy_entity) {
            continue;
        }

        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            if despawned_entities.contains(&bullet_entity) {
                continue;
            }

            let dx = enemy_transform.translation.x - bullet_transform.translation.x;
            let dy = enemy_transform.translation.y - bullet_transform.translation.y;
            let distance_2d = (dx * dx + dy * dy).sqrt();
            //println!("distance_2d = {}", distance_2d);

            if distance_2d < HIT_DISTANCE {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                println!("hit 2d space !!!");

                despawned_entities.insert(bullet_entity);
                despawned_entities.insert(enemy_entity);
            }
        }
    }
}

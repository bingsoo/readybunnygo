use crate::game::background::BackgroundPanel;
use bevy::prelude::*;

use rand::Rng;

#[derive(Debug)]
enum EnemyType {
    Type0,
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
    Type6,
    Type7,
    Type8,
    Type9,
}

impl EnemyType {
    fn get_image_file(&self) -> String {
        match self {
            EnemyType::Type0 => "ships/ship_0000.png".to_string(),
            EnemyType::Type1 => "ships/ship_0001.png".to_string(),
            EnemyType::Type2 => "ships/ship_0002.png".to_string(),
            EnemyType::Type3 => "ships/ship_0003.png".to_string(),
            EnemyType::Type4 => "ships/ship_0004.png".to_string(),
            EnemyType::Type5 => "ships/ship_0005.png".to_string(),
            EnemyType::Type6 => "ships/ship_0006.png".to_string(),
            EnemyType::Type7 => "ships/ship_0007.png".to_string(),
            EnemyType::Type8 => "ships/ship_0008.png".to_string(),
            EnemyType::Type9 => "ships/ship_0009.png".to_string(),
        }
    }
}

pub const NUM_ENEMY: usize = 500;

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>, q: Query<Entity, With<BackgroundPanel>>) {
    println!("spawn enemy");

    let bg = q.single();

    for i in 0..NUM_ENEMY {
        let enemy_location = Vec3::new(0.0, i as f32 * 90.0 + 500.0, 10.);
        let enemy_type = random_enemy_type();
        add_enemy(&mut commands, &asset_server, bg, enemy_location, enemy_type);
    }

    for entity in q.iter() {
        println!("in spawn_enemy found id {}", entity.index());
    }
}

fn add_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, bg_panel: Entity, loc: Vec3, enemy_type: EnemyType) {
    let enemy = commands
        .spawn((SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(40.0, 40.0)), ..default() },
            texture: asset_server.load(enemy_type.get_image_file()),
            transform: Transform::from_translation(loc),
            ..Default::default()
        },))
        .id();

    commands.entity(bg_panel).push_children(&[enemy]);
}

fn random_enemy_type() -> EnemyType {
    let index = rand::thread_rng().gen_range(0..=9); // Adjust the range to match the number of variants.

    match index {
        0 => EnemyType::Type0,
        1 => EnemyType::Type1,
        2 => EnemyType::Type2,
        3 => EnemyType::Type3,
        4 => EnemyType::Type4,
        5 => EnemyType::Type5,
        6 => EnemyType::Type6,
        7 => EnemyType::Type7,
        8 => EnemyType::Type8,
        _ => EnemyType::Type9, // Handle any other value (shouldn't happen in this case).
    }
}

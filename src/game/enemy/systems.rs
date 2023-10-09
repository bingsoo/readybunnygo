use crate::prelude::*;

const NUM_ENEMY: usize = 1500;

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q: Query<Entity, With<BackgroundPanel>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    println!("spawn enemy");

    let bg = q.single();
    let window = window_query.single();
    let mut rng = rand::thread_rng();

    for i in 0..NUM_ENEMY {
        let enemy_x: f32 = rng.gen_range(0.0..=window.width()) - window.width() * 0.5;
        let enemy_location = Vec3::new(enemy_x, i as f32 * 50.0 + 900.0, 10.);
        let enemy_type = random_enemy_type();
        add_enemy(&mut commands, &asset_server, bg, enemy_location, enemy_type);
    }

    for entity in q.iter() {
        println!("in spawn_enemy found id {}", entity.index());
    }
}

pub fn update_enemy(
    mut enemy_query: Query<(&mut Transform, &EnemyShip)>,
    global_data: Res<GlobalData>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let current_loc = get_real_location(&transform.translation, &global_data);
        let speed = enemy.enemy_type.get_speed();
        if current_loc.y < window.height() + 500.0 {
            transform.translation.y -= speed;
        }
    }
}

fn add_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    bg_panel: Entity,
    loc: Vec3,
    enemy_type: EnemyType,
) {
    let enemy = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..default()
            },
            texture: asset_server.load(enemy_type.get_image_file()),
            transform: Transform::from_translation(loc),
            ..Default::default()
        })
        .insert(EnemyShip { enemy_type })
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

use crate::prelude::*;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Bunny;

#[derive(Component)]
pub struct EnemyShip {
    pub enemy_type: EnemyType,
    pub appear_distance: f32,
}

#[derive(Component)]
pub struct BackgroundPanel;

#[derive(Component)]
pub struct BackgroundTile;

#[derive(Component)]
pub struct BulletObject;

#[derive(Component)]
pub struct BulletTimer(pub Timer);

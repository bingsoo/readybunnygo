use crate::prelude::*;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Bunny;

#[derive(Component)]
pub struct EnemyShip {
    pub enemy_type: EnemyType,
}

#[derive(Component)]
pub struct BackgroundPanel;

#[derive(Component)]
pub struct Tile;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_enemy(window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    println!("spawn enemy!");
    println!("window size in enemy = {} / {}", window.width(), window.height());
}

use bevy::prelude::*;
use crate::health::Health;

#[allow(dead_code)]
#[derive(Component)]
pub struct Enemy {
    radius: f32,
}

#[allow(dead_code)]
#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    health: Health,
    sprite: SpriteBundle,
}


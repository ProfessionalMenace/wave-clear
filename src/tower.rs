use crate::health::Health;
use crate::projectile::*;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::collections::HashMap;
use std::time::Duration;

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_update);
    }
}

#[derive(Component, Clone)]
pub struct Tower {
    pub stopwatch: Stopwatch,
    pub cooldown: Duration,
}

#[derive(Bundle, Clone)]
pub struct TowerBundle {
    pub tower: Tower,
    pub health: Health,
    pub sprite: SpriteBundle,
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct TowerSpawner {
    pub spawner: HashMap<String, TowerBundle>, // tower name -> Tower + Health + Sprite + Projectile
}

// creating tower from tower_id (not implemented)
pub fn tower_create(pos: Vec2, image_handle: Handle<Image>) -> TowerBundle {
    let translation = Vec3::new(pos.x, pos.y, 0.0);
    TowerBundle {
        tower: Tower {
            stopwatch: Stopwatch::new(),
            cooldown: Duration::from_secs(4),
        },
        health: Health { hp: 100 },
        sprite: SpriteBundle {
            texture: image_handle,
            transform: Transform::from_translation(translation),
            ..default()
        },
    }
}

// tower spawn projectile in time interval
pub fn tower_update(
    time: Res<Time>,
    mut commands: Commands,
    spawner: Res<ProjectileSpawner>,
    mut query: Query<(&mut Tower, &Transform)>,
) {
    for (mut tower, transform) in &mut query {
        if tower.stopwatch.elapsed() > tower.cooldown {
            if let Some(projectile_bundle) = spawner.spawner.get("default") {
                let mut new_pb = (*projectile_bundle).clone();
                new_pb.sprite.transform.translation = transform.translation.clone();
                commands.spawn(new_pb); // too much clone?
            };
            tower.stopwatch.reset();
        }
        tower.stopwatch.tick(time.delta());
    }
}

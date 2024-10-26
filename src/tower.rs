use crate::health::Health;
use crate::projectile::*;
use crate::spawner::Spawner;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::time::Duration;

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_update);
    }
}

#[derive(Component, Clone)]
pub struct Tower {
    pub proj_name: String,
    pub stopwatch: Stopwatch,
    pub cooldown: Duration,
}

#[derive(Bundle, Clone)]
pub struct TowerBundle {
    pub tower: Tower,
    pub health: Health,
    pub sprite: SpriteBundle,
}

// tower spawn projectile in time interval
pub fn tower_update(
    time: Res<Time>,
    mut commands: Commands,
    spawner: Res<Spawner<ProjectileBundle>>,
    mut query: Query<(&mut Tower, &Transform)>,
) {
    for (mut tower, transform) in &mut query {
        if tower.stopwatch.elapsed() > tower.cooldown {
            let projectile_bundle = spawner.get(&tower.proj_name);
            let mut new_pb = (*projectile_bundle).clone();
            new_pb.sprite.transform.translation = transform.translation.clone();
            commands.spawn(new_pb); // too much clone?
            tower.stopwatch.reset();
        }
        tower.stopwatch.tick(time.delta());
    }
}

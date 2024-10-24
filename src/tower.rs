use std::time::Duration;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use crate::projectile::*;
use crate::health::Health;

pub struct TowerPlugin;
impl Plugin for TowerPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_update);
    }
}

#[derive(Component)]
pub struct Tower
{
    pub stopwatch: Stopwatch,
    pub cooldown: Duration,
}

#[derive(Bundle)]
pub struct TowerBundle
{
    pub tower: Tower,
    pub health: Health,
    pub sprite: SpriteBundle,
}

// creating tower from tower_id (not implemented)
pub fn tower_create(
    pos: Vec2,
    image_handle: Handle<Image>,
) -> TowerBundle {
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
        }
    }
}

// tower spawn projectile in time intervals
pub fn tower_update(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, // maybe posible to remove
    spawner : Res<ProjectileSpawner>, // should contain sprite handle
    mut query: Query<(&mut Tower, &Transform)>,
){
    for (mut tower, transform) in &mut query
    {
        if tower.stopwatch.elapsed() > tower.cooldown
        {
            let Some(projectile) = spawner.spawner.get("default") else {
                continue; // skips the reset -> tries spawn every time
            };
            let bundle = ProjectileBundle{
                projectile: projectile.clone(),
                sprite_bundle: SpriteBundle{
                    transform: Transform::from_translation(transform.translation),
                    texture: asset_server.load("Projectiles/Default.png"),
                    ..default()
                },
            };
            commands.spawn(bundle);
            tower.stopwatch.reset();
        }
        tower.stopwatch.tick(time.delta());
    }
}
